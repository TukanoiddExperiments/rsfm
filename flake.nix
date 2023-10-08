{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    nci = {
      url = "github:yusdacra/nix-cargo-integration";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    parts.url = "github:hercules-ci/flake-parts";
    parts.inputs.nixpkgs-lib.follows = "nixpkgs";
  };

  outputs = inputs @ {
    parts,
    nci,
    ...
  }:
    parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux"];
      imports = [nci.flakeModule];
      perSystem = {
        pkgs,
        config,
        ...
      }: let
        crateName = "rsfm";
        crateOutputs = config.nci.outputs.${crateName};

        shellDeps = with pkgs; [
          cargo-expand
        ];
      in {
        nci = {
            toolchainConfig = ./rust-toolchain.toml;
            projects.${crateName}.path = ./.;
            crates = {
                "macros" = {
                      export = true;
                    };

                    ${crateName} = {
                      export = true;
                      runtimeLibs = with pkgs; with pkgs.xorg; [
                        pkg-config

                        wayland

                        vulkan-loader
                        vulkan-validation-layers

                        libGL
                        libGLU

                        libX11
                        libxkbcommon
                        libxcb
                        libXcursor
                        libXrandr
                        libXi
                      ];
                    };
          };
        };

        devShells.default = crateOutputs.devShell.overrideAttrs (old: {
          packages = (old.packages or []) ++ shellDeps;

           shellHook = ''
            ln -s
           '';
        });
        packages.default = crateOutputs.packages.release;
      };
    };
}
