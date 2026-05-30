{
  description = "Passivate";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-25.11";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        libPath = with pkgs; lib.makeLibraryPath [
          wayland
          libxkbcommon
          libGL
        ];
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            (
              rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
                extensions = [ "rust-src" "rust-analyzer" ];
                #targets = [ "x86_64-unknown-linux-gnu" ]
              })
            )
            cargo-nextest
          ];

          LD_LIBRARY_PATH = libPath;
          PASSIVATE_TEST_DATA = "${toString ./.}/test_data";
        };
      }
    );
}
