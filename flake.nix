{
  description = "Passivate";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
    self.submodules = true;
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

            nativeBuildInputs = [
              pkgs.makeWrapper
              pkgs.autoPatchelfHook
            ];

            buildInputs = [
              pkgs.libgcc
            ];
          };

          deps-args = {
              postInstall = ''
                # wrapProgram $out/bin/passivate --prefix LD_LIBRARY_PATH : ${libPath}
              '';

              libs = libPath;

              postFixup = ''
                  patchelf $out/bin/passivate
                '';

              cargoArtifacts = craneLib.buildDepsOnly commonArgs;
            };

          crate-args = commonArgs // deps-args // {
            buildInputs = [
              pkgs.wayland
              pkgs.libxkbcommon
              pkgs.libGL
            ];
          };

          passivate-build = craneLib.buildPackage crate-args;
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
              pkgs.wayland
              pkgs.wl-clipboard
            ];

            LD_LIBRARY_PATH = libPath;
            # LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
            #             pkgs.wayland
            #             pkgs.libxkbcommon
            #             pkgs.libGL
            #           ];

            PASSIVATE_TEST_DATA = "${toString ./.}/test_data";
          };

          packages.default = passivate-build;
        }
      );
    in
    {
      packages = allSystems.packages;
      devShells = allSystems.devShells;
      hydraJobs.x86_64-linux.default = allSystems.packages.x86_64-linux.default;
    };
}
