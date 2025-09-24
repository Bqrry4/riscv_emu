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
    in
    {
      packages.${system}.opensbi = pkgs.stdenv.mkDerivation {
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
          "PLATFORM_RISCV_ISA=rv64ima_zicsr_zifencei"
          "PLATFORM_RISCV_ABI=lp64"
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
    };
}
