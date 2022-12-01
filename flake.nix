{
  inputs = {
    fenix.url = "github:nix-community/fenix";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, fenix, flake-utils, naersk, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
        };

        toolchain = with fenix.packages.${system};
          combine [
            minimal.rustc
            minimal.cargo
            targets.x86_64-unknown-linux-gnu.latest.rust-std
            targets.x86_64-unknown-linux-gnu.latest.clippy
            targets.x86_64-unknown-linux-gnu.latest.rustfmt
            targets.x86_64-pc-windows-gnu.latest.rust-std
          ];

        naersk' = naersk.lib.${system}.override {
          cargo = toolchain;
          rustc = toolchain;
        };

        naerskBuildPackage = target: args:
          naersk'.buildPackage (
            args
              // { CARGO_BUILD_TARGET = target; }
              // cargoConfig
          );

        cargoConfig = {
          CARGO_TARGET_X86_64_PC_WINDOWS_GNU_RUNNER = pkgs.writeScript "wine-wrapper" ''
            export WINEPREFIX="$(mktemp -d)"
            exec wine64 $@
          '';
          LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
          BINDGEN_EXTRA_CLANG_ARGS = "-isystem ${pkgs.llvmPackages.libclang.lib}/lib/clang/${pkgs.lib.getVersion pkgs.clang}/include";

          preConfigure = ''
            mkdir /tmp/deps
            rsync -aL $crate_sources/ /tmp/deps
            chmod 777 -R /tmp/deps
            crate_sources=/tmp/deps
          '';

          preBuild = ''
            echo ${pkgs.llvmPackages.libclang}
            echo '[source]' > $CARGO_HOME/config
            echo '"crates-io" = { "replace-with" = "nix-sources" }' >> $CARGO_HOME/config
            echo '"nix-sources" = { directory = "/tmp/deps" }' >> $CARGO_HOME/config
          '';

        };

      in rec {
        defaultPackage = packages.x86_64-pc-linux-gnu;

        packages.x86_64-pc-windows-gnu = naerskBuildPackage "x86_64-pc-windows-gnu" {
          src = ./.;
          doCheck = true;
          strictDeps = true;
          singleStep = true;
          copyLibs = true;

          depsBuildBuild = with pkgs; [
            pkgsCross.mingwW64.stdenv.cc
            pkgsCross.mingwW64.windows.pthreads
          ];

          nativeBuildInputs = with pkgs; [
            # We need Wine to run tests:
            wineWowPackages.stable
            llvmPackages.libclang
            llvmPackages.libcxxClang
            clang
            mpich
          ];
        };

        packages.x86_64-pc-linux-gnu = naerskBuildPackage "x86_64-unknown-linux-gnu" {
          src = ./.;
          strictDeps = true;

          nativeBuildInputs = with pkgs; [
            llvmPackages.libclang
            llvmPackages.libcxxClang
            clang
            mpich
          ];

          doCheck = true;
          copyLibs = true;
          singleStep = true;
        };
        devShell = pkgs.mkShell {
          nativeBuildInputs = [ toolchain pkgs.rust-analyzer pkgs.llvmPackages.libclang pkgs.llvmPackages.libcxxClang pkgs.clang pkgs.mpich ];
          depsBuildBuild = with pkgs; [
            pkgsCross.mingwW64.stdenv.cc
            pkgsCross.mingwW64.windows.pthreads
          ];
          LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
          BINDGEN_EXTRA_CLANG_ARGS = "-isystem ${pkgs.llvmPackages.libclang.lib}/lib/clang/${pkgs.lib.getVersion pkgs.clang}/include";

          preConfigure = ''
            mkdir /tmp/deps
            rsync -aL $crate_sources/ /tmp/deps
            chmod 777 -R /tmp/deps
            crate_sources=/tmp/deps
          '';

          preBuild = ''
            echo '[source]' > $CARGO_HOME/config
            echo '"crates-io" = { "replace-with" = "nix-sources" }' >> $CARGO_HOME/config
            echo '"nix-sources" = { directory = "/tmp/deps" }' >> $CARGO_HOME/config
          '';
        };
      }
    );
}
