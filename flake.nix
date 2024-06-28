{
  description = "Build the cargo project while also compiling the standard library";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, crane, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachSystem [
      "aarch64-darwin"
      "aarch64-linux"
      "i686-linux"
      "x86_64-darwin"
      "x86_64-linux"
    ]
      (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ (import rust-overlay) ];
          };

          inherit (pkgs) lib;

          rustToolchain = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
            extensions = [ "rust-src" ];
            targets = [
              "aarch64-apple-darwin"
              "aarch64-unknown-linux-gnu"
              "i686-unknown-linux-gnu"
              "x86_64-apple-darwin"
              "x86_64-unknown-linux-gnu"
            ];
          });

          craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

          src = lib.cleanSourceWith {
            src = ./.;
            filter = path: type:
              (lib.hasSuffix "\.slint" path) ||
              (craneLib.filterCargoSources path type);
          };

          slint-rust-calculator = craneLib.buildPackage {
            inherit src;
            strictDeps = true;

            cargoVendorDir = craneLib.vendorMultipleCargoDeps {
              inherit (craneLib.findCargoFiles src) cargoConfigs;
              cargoLockList = [
                ./Cargo.lock

                # Unfortunately this approach requires IFD (import-from-derivation)
                # otherwise Nix will refuse to read the Cargo.lock from our toolchain
                # (unless we build with `--impure`).
                #
                # Another way around this is to manually copy the rustlib `Cargo.lock`
                # to the repo and import it with `./path/to/rustlib/Cargo.lock` which
                # will avoid IFD entirely but will require manually keeping the file
                # up to date!
                "${rustToolchain.passthru.availableComponents.rust-src}/lib/rustlib/src/rust/Cargo.lock"
              ];
            };


            cargoExtraArgs = {
              "aarch64-darwin" = "-Z build-std --target aarch64-apple-darwin";
              "aarch64-linux" = "-Z build-std --target aarch64-unknown-linux-gnu";
              "i686-linux" = "-Z build-std --target i686-unknown-linux-gnu";
              "x86_64-darwin" = "-Z build-std --target x86_64-apple-darwin";
              "x86_64-linux" = "-Z build-std --target x86_64-unknown-linux-gnu";
            }."${system}";

            nativeBuildInputs = with pkgs; [
              pkg-config
              openssl
            ];

            runtimeDependencies = with pkgs; [
              libxkbcommon
              fontconfig

              # Wayland
              wayland

              # Xorg/X11
              xorg.libX11
              xorg.libXcursor
              xorg.libXrandr
              xorg.libXi
            ];
          };
        in
        {
          checks = { inherit slint-rust-calculator; };

          packages.default = slint-rust-calculator;

          apps.default = flake-utils.lib.mkApp { drv = slint-rust-calculator; };

          devShells.default = craneLib.devShell rec {
            inputsFrom = builtins.attrValues self.checks.${system};
            LD_LIBRARY_PATH = pkgs.lib.strings.makeLibraryPath (builtins.concatMap (d: d.runtimeDependencies) inputsFrom);
          };
        });
}
