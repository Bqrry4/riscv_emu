pub enum Trap {
    /* The trap is visible to, and handled by, software running inside the execution environment. */
    Contained,
    /* The trap is a synchronous exception that is an explicit call to the execution environment
     * requesting an action on behalf of software inside the execution environment. */
    Requested,
    /* The trap is handled transparently by the execution environment
     * and execution resumes normally after the trap is handled. */
    Invisible,
    /* The trap represents a fatal failure and causes the execution environment to terminate execution. */
    Fatal,
}

#[repr(u8)]
pub enum Exception {
    InstructionAddressMisaligned = 0,
    InstructionAccessFault,
    IllegalInstruction,
    Breakpoint,
    LoadAddressMisaligned,
    LoadAccessFault,
    StoreAddressMisaligned,
    StoreAccessFault,
    EnvironmentCallFromUMode,
    EnvironmentCallFromSMode,
    EnvironmentCallFromMMode = 11,
    InstructionPageFault,
    LoadPageFault,
    StorePageFault = 15,
    DoubleTrap,
    SoftwareCheck = 18,
    HardwareError,
}
