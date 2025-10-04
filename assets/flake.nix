{
  description = "opensbi build";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
  };

  outputs =
    { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        localSystem.system = system;
        crossSystem.system = "riscv64-unknown-linux-gnu";
      };
      rv_isa = "rv64ima_zicsr_zifencei";
      rv_abi = "lp64";
    in
    {
      packages.${system} = {
        opensbi = pkgs.stdenv.mkDerivation {
          pname = "opensbi";
          version = "1.7";

          src = pkgs.fetchFromGitHub {
            owner = "riscv-software-src";
            repo = "opensbi";
            rev = "v1.7";
            sha256 = "0lp2j4cgnv9i1rhw8kdjfzqpzbrg4hvl1c6mxk1l3j612z2rkrjp";
          };

          nativeBuildInputs = [
            pkgs.buildPackages.gnumake
            pkgs.buildPackages.python3
          ];

          makeFlags = [
            "PLATFORM=generic"
            "FW_DYNAMIC=y"
            "PLATFORM_RISCV_XLEN=64"
            "PLATFORM_RISCV_ISA=${rv_isa}"
            "PLATFORM_RISCV_ABI=${rv_abi}"
            "CROSS_COMPILE=${pkgs.stdenv.cc.targetPrefix}"
          ];

          # fix shebangs
          postPatch = ''
            scripts=$(find . -type f -name "*.py" -o -name "*.sh")
            for s in $scripts; do
              patchShebangs $s
            done
          '';

          installPhase = ''
            mkdir -p $out
            cp build/platform/generic/firmware/fw_dynamic.bin $out/opensbi.bin
          '';
        };

        dts = pkgs.stdenv.mkDerivation {
          name = "dts";

          src = ./.;

          nativeBuildInputs = [
            pkgs.buildPackages.dtc
          ];
          buildPhase = ''
            ls . -all
            dtc virt.dts -I dts -O dtb -o virt.dtb
          '';

          installPhase = ''
            mkdir -p $out
            mv virt.dtb $out/
          '';
        };

        test_asm = pkgs.stdenv.mkDerivation {
          name = "test_asm";

          src = ./../tests/asm;

          nativeBuildInputs = [
            pkgs.buildPackages.gcc
          ];

          buildPhase = ''
            riscv64-unknown-linux-gnu-gcc -c include/exit.s -o exit.o -march=${rv_isa} -mabi=${rv_abi}
            for f in $(ls $src/*.s); do
                name=$(basename $f .s)
                riscv64-unknown-linux-gnu-gcc -c $f -o $name.o -march=${rv_isa} -mabi=${rv_abi}
                # put the file with _start first for it to be the direct entry
                riscv64-unknown-linux-gnu-ld -Ttext=0x80000000 -o $name.elf $name.o exit.o
                riscv64-unknown-linux-gnu-objcopy -O binary $name.elf $name.bin
            done
          '';

          installPhase = ''
            mkdir -p $out
            mv *.bin $out/
          '';
        };
      };
    };
}
