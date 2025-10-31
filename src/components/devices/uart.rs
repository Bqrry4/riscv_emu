#![allow(unused_parens)]
use std::collections::VecDeque;

use arbitrary_int::{u1, u2, u4};
use bitbybit::{bitenum, bitfield};

use crate::{
    components::trap::Exception,
    util::{F, T},
};

/// Size of the addressable region
pub const UART_SIZE: u64 = 0x100;
pub const IRQ_UART: u32 = 0x0a;

/* uart register addresses*/
/// Receiver Buffer (read)
const RBR: (u8, usize) = (0b0, 0b000);
/// Transmitter Holding (write)
const THR: (u8, usize) = (0b0, 0b000);
/// Interrupt Enable Register
const IER: (u8, usize) = (0b0, 0b001);
/// Interrupt Identification (read)
const IIR: usize = 0b010;
/// FIFO Control (write)
const FCR: usize = 0b010;
/// Line Control
const LCR: usize = 0b011;
/// MODEM Control
const MCR: usize = 0b100;
/// Line Status
const LSR: usize = 0b101;
/// MODEM Status
const MSR: usize = 0b110;
/// Scratch
const SCR: usize = 0b111;
/// Divisor Latch (LSB)
const DLL: (u8, usize) = (0b1, 0b000);
/// Divisor Latch (MSB)
const DLH: (u8, usize) = (0b1, 0b000);

#[bitfield(u8)]
struct LCR {
    /// Divisor latch access bit
    #[bit(7, rw)]
    dlab: u1,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[bitenum(u2, exhaustive = true)]
enum RxFifoTriggerLvl {
    L1 = 0b00,
    L4 = 0b01,
    L8 = 0b10,
    L14 = 0b11,
}
impl RxFifoTriggerLvl {
    fn value(&self) -> u8 {
        match self {
            RxFifoTriggerLvl::L1 => 1,
            RxFifoTriggerLvl::L4 => 4,
            RxFifoTriggerLvl::L8 => 8,
            RxFifoTriggerLvl::L14 => 14,
        }
    }
}
#[bitfield(u8)]
struct FCR {
    /// RCVR FIFO Trigger Level
    #[bits(6..=7, rw)]
    rftl: RxFifoTriggerLvl,
    /// DMA Mode Select
    #[bit(3, rw)]
    dms: u1,
    /// XMIT Fifo Reset
    #[bit(2, rw)]
    xfr: u1,
    /// RCVR Fifo Reset
    #[bit(1, rw)]
    rfr: u1,
    /// FIFO Enable
    #[bit(0, rw)]
    fe: u1,
}

#[bitfield(u8)]
struct IIR {
    /// FIFOs Enabled
    #[bits(6..=7, rw)]
    fe: u2,
    /// Interrupt ID
    // Contains the nip bit, as the interrupts are nicely encoded
    #[bits(0..=3, rw)]
    id: u4,
    /// Interrupt Pending on "0"
    #[bit(0, rw)]
    nip: u1,
}
impl IIR {
    /* IIR interrupt IDs */
    /// No interrupts
    const ID_NONE: u4 = u4::new(0x1);
    /// Modem status
    const ID_MSI: u4 = u4::new(0x0);
    /// Transmitter Holding Register Empty
    const ID_THREI: u4 = u4::new(0x2);
    /// Received Data Available
    const ID_RDAI: u4 = u4::new(0x4);
    /// Receiver line status
    const ID_RLSI: u4 = u4::new(0x6);
    /// Character Timeout Indication
    const ID_CTI: u4 = u4::new(0xC);
}

#[bitfield(u8)]
struct LSR {
    /// Transmitter empty
    #[bit(6, rw)]
    temt: u1,
    /// Transmit-hold-register empty
    #[bit(5, rw)]
    thre: u1,
    /// Break Interrupt
    #[bit(4, rw)]
    bi: u1,
    /// Frame error
    #[bit(3, rw)]
    fe: u1,
    /// Parity error
    #[bit(2, rw)]
    pe: u1,
    /// Overrun error
    #[bit(1, rw)]
    oe: u1,
    /// Data Ready
    #[bit(0, rw)]
    dr: u1,

    /// Overrun Error, Parity Error, Framing Error and Break Interrupt
    /// are interrupt sources.
    #[bits(1..=4, r)]
    isrc: u4,
}

#[bitfield(u8)]
struct MCR {
    /// Enable loopback test mode
    #[bit(4, rw)]
    lpb: u1,
    /// Out2 complement
    #[bit(3, rw)]
    out2: u1,
    /// Out1 complement
    #[bit(2, rw)]
    out1: u1,
    /// RTS complement
    #[bit(1, rw)]
    rts: u1,
    /// DTR complement
    #[bit(0, rw)]
    dtr: u1,
}
#[bitfield(u8)]
struct IER {
    /// Enable MODEM Status Interrupt
    #[bit(3, rw)]
    edssi: u1,
    /// Enable Receiver Line Status Interrupt
    #[bit(2, rw)]
    elsi: u1,
    /// Enable Transmitter Holding Register Empty Interrupt
    #[bit(1, rw)]
    etbei: u1,
    /// Enable Received Data Available Interrupt
    #[bit(0, rw)]
    erbfi: u1,
    #[bits(0..=3, rw)]
    value: u4,
}

/// Implements the NS16550A uart device.
/// https://wiki.osdev.org/UART.
/// Qemu uses this one in it's virt machine.
/// https://github.com/qemu/qemu/blob/master/hw/char/serial.c
/// https://courses.grainger.illinois.edu/ece391/su2025/docs/NS16550A.pdf
pub struct Uart {
    rbr: u8,
    thr: u8,
    recv_fifo: VecDeque<u8>,
    xmit_fifo: VecDeque<u8>,

    ier: IER,
    iir: IIR,
    fcr: FCR,
    lcr: LCR,
    mcr: MCR,
    lsr: LSR,
    msr: u8,
    scr: u8,

    /// Interrupt output signal
    intr: bool,
    thr_ipending: bool,
    recv_fifo_itl: u8,

    reg: [u8; UART_SIZE as usize],
}

impl Uart {
    pub fn new() -> Uart {
        Self {
            rbr: 0,
            thr: 0,
            recv_fifo: VecDeque::with_capacity(16),
            xmit_fifo: VecDeque::with_capacity(16),

            ier: IER::ZERO,
            iir: IIR::ZERO,
            fcr: FCR::ZERO,
            lcr: LCR::ZERO,
            mcr: MCR::ZERO,
            lsr: LSR::ZERO,
            msr: 0,
            scr: 0,

            intr: false,
            thr_ipending: false,
            recv_fifo_itl: 0,

            reg: [0; UART_SIZE as usize],
        }
    }

    pub fn is_interrupting(&self) -> bool {
        self.intr
    }

    fn update_iir(&mut self) {
        let rlsi = (self.ier.elsi() == T)
            //& Overrun Error or Parity Error or Framing Error or Break Interrupt
            && (self.lsr.isrc().value() != 0);
        let rdai = (self.ier.erbfi() == T)
            //& Receiver Data Available or Trigger Level Reached
            && (self.lsr.dr() == T)
            // Either a byte is ready in simple mode
            && ((self.fcr.fe() == F)
                // Or itl has been reached for fifo
                || (self.recv_fifo.len() >= self.recv_fifo_itl as usize));
        // In qemu srcode, it is said that this might get masked by the above handling
        //TODO:
        let cti = self.ier.erbfi() == T;
        let threi = (self.ier.etbei() == T)
            //& Transmitter Holding Register Empty
            && (self.thr_ipending);
        //TODO:
        let msi = self.ier.edssi() == T;

        let id = match () {
            _ if rlsi => IIR::ID_RLSI,
            _ if rdai => IIR::ID_RDAI,
            _ if cti => IIR::ID_CTI,
            _ if threi => IIR::ID_THREI,
            _ if msi => IIR::ID_MSI,
            _ => IIR::ID_NONE,
        };

        self.iir.set_id(id);
        self.intr = (id != IIR::ID_NONE);
    }

    fn write_fcr(&mut self, value: u8) {
        let mut val = FCR::new_with_raw_value(value);

        //& When changing from FIFO Mode to NS16450 Mode and vice versa, data is automatically cleared from the FIFOs.
        if (val.fe() ^ self.fcr.fe()) == T {
            // set reset bits on
            val.set_rfr(T);
            val.set_xfr(T);
        }

        //&  Writing a 1 to FCR1 clears all bytes in the RCVR FIFO and resets its counter logic to 0.
        if val.rfr() == T {
            self.recv_fifo.clear();
        }

        //& Writing a 1 to FCR2 clears all bytes in the XMIT FIFO and resets its counter logic to 0.
        if val.xfr() == T {
            self.xmit_fifo.clear();
        }

        self.fcr = FCR::new_with_raw_value(
            //xfr and rfr bits are cleared out
            val.raw_value() & 0xC9,
        );

        // Update the IIR
        let iir_fe = if self.fcr.fe() == T {
            // Fifo enabled, so update recv_fifo trigger Level as well
            self.recv_fifo_itl = self.fcr.rftl().value();

            u2::new(0b11)
        } else {
            u2::new(0b00)
        };
        self.iir.set_fe(iir_fe);

        self.update_iir();
    }

    pub fn read(&mut self, offset: u64) -> Result<u8, Exception> {
        let dlab: u8 = self.lcr.dlab().value();

        let ret: u8;
        let data = match (dlab, offset as usize) {
            RBR => {
                if self.fcr.fe() == T {
                    ret = self.recv_fifo.pop_back().unwrap_or(0);

                    if self.recv_fifo.is_empty() {
                        self.lsr.set_dr(F);
                        self.lsr.set_bi(F);
                    }
                } else {
                    ret = self.rbr;

                    self.lsr.set_dr(F);
                    self.lsr.set_bi(F);
                };
                self.update_iir();

                ret
            }
            IER => self.ier.raw_value(),
            (_, IIR) => {
                // Reading the IIR Register resets THREI
                if self.iir.id() == IIR::ID_THREI {
                    self.thr_ipending = false;
                    self.update_iir();
                }
                self.iir.raw_value()
            }
            (_, LCR) => self.lcr.raw_value(),
            (_, MCR) => self.mcr.raw_value(),
            (_, LSR) => {
                ret = self.lsr.raw_value();
                self.lsr.set_oe(F);
                self.lsr.set_pe(F);
                self.lsr.set_fe(F);
                self.lsr.set_bi(F);

                self.update_iir();
                ret
            }
            (_, MSR) => 0,
            (_, SCR) => self.scr,
            _ => self.reg[offset as usize],
        };

        Ok(data)
    }

    pub fn write(&mut self, offset: u64, value: u8) -> Result<(), Exception> {
        let dlab: u8 = self.lcr.dlab().value();

        match (dlab, offset as usize) {
            THR => {
                self.thr = value;

                if self.fcr.fe() == T {
                    // In case an overrun occurs
                    if self.recv_fifo.len() == self.recv_fifo.capacity() {
                        self.recv_fifo.pop_back();
                        self.lsr.set_oe(T);
                    }

                    self.recv_fifo.push_front(value);
                };

                // Writing into the THR resets THREI
                self.thr_ipending = false;
                self.lsr.set_thre(F);
                self.lsr.set_temt(F);
                self.update_iir();

                //TODO:
                // how this should be consumed?
            }
            IER => {
                let val = IER::new_with_raw_value(value);
                // If the bit changed
                if (self.ier.etbei() ^ val.etbei()) == T {
                    if (val.etbei() == T) && (self.lsr.thre() == T) {
                        self.thr_ipending = true;
                    } else {
                        self.thr_ipending = false;
                    }
                }

                self.ier = val;
                self.update_iir();
            }
            (_, FCR) => self.write_fcr(value),
            (_, LCR) => {
                //TBD
            }
            (_, MCR) => {
                self.mcr = MCR::new_with_raw_value(value);
            }
            (_, MSR) => {
                self.msr = value;
            }
            (_, SCR) => {
                self.scr = value;
            }
            //& The Line Status Register is intended for read operations only.
            //& Writing to this register is not recommended as this operation is only used for factory testing.
            (_, LSR) | _ => {}
        }
        Ok(())
    }
}
