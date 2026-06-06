{
  description = "Passivate";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
    passivate-nextest.url = "github:TobiasvdVenOrg/passivate-nextest/passivate-compatible";
  };



  outputs = { self, nixpkgs, rust-overlay, flake-utils, crane, passivate-nextest, ... }:


    flake-utils.lib.eachDefaultSystem (system:
    let
      overlays = [ rust-overlay.overlays.default ];

      pkgs = import nixpkgs {
        inherit system overlays;
      };

      libPath = pkgs.lib.makeLibraryPath [
        pkgs.wayland
        pkgs.libxkbcommon
        pkgs.libGL
      ];

      crane-pkgs = nixpkgs.legacyPackages.${system};
      craneLib = crane.mkLib pkgs;

      root-src = ./.;

      mod-src = pkgs.runCommand "passivate" {} ''
                  echo "mkdir $out"
                  mkdir -p $out

                  echo "cp ${root-src}"

                  cp -r ${root-src}/passivate $out/passivate
                  cp -r ${passivate-nextest} $out/passivate-nextest
              '';

      passivate-src = mod-src + "/passivate";

      clean-src = craneLib.cleanCargoSource passivate-src;

      commonArgs = {
        pname = "passivate";
        version = "0.1.0";

        src = clean-src;

        strictDeps = true;

        buildInputs = [
          # Add additional build inputs here
        ];
      };

      deps-args = {
          cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        };

        crate-args = commonArgs // deps-args;
        my-crate = craneLib.buildPackage crate-args;
    in
    {
      devShells.default = pkgs.mkShell {
        buildInputs = [
          (
            pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
              extensions = [ "rust-src" "rust-analyzer" ];
              #targets = [ "x86_64-unknown-linux-gnu" ]
            })
          )
          pkgs.cargo-nextest
        ];

        LD_LIBRARY_PATH = libPath;
        PASSIVATE_TEST_DATA = "${toString ./.}/test_data";
      };

      packages.default = my-crate;
    }
  );
}
