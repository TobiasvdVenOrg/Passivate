{
  description = "Passivate";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, crane, ... }:
    let
      allSystems = flake-utils.lib.eachDefaultSystem (system:
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

          defaultCraneLib = crane.mkLib pkgs;
          craneLib = defaultCraneLib.overrideToolchain (p: p.rust-bin.nightly.latest.default);

          commonArgs = {
            pname = "passivate";
            version = "0.1.0";

            postUnpack = ''
              cd $sourceRoot/passivate
              sourceRoot="."
            '';

            src = ./.;

            cargoToml = ./passivate/Cargo.toml;
            cargoLock = ./passivate/Cargo.lock;

            strictDeps = true;
            doCheck = false;

            PASSIVATE_TEST_DATA = "${./test_data}";
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
    in
    {
      packages = allSystems.packages;

      hydraJobs.x86_64-linux.default = allSystems.packages.x86_64-linux.default;
    };
}
