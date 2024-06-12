{
  description = "Build a cargo project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { nixpkgs, rust-overlay, ... }:
    let
      pkgs = import nixpkgs {
        system = "x86_64-linux";
        overlays = [ (import rust-overlay) ];
      }; 
    in
    {
      packages.x86_64-linux.default = pkgs.callPackage ./. { };
      devShells.x86_64-linux.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          rust-bin.stable.latest.default
        ];
      };
    };
}
