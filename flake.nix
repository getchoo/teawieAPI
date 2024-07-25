{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    fenix = {
      url = "github:nix-community/fenix";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        rust-analyzer-src.follows = "";
      };
    };

    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      fenix,
      treefmt-nix,
    }:
    let
      inherit (nixpkgs) lib;
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      forAllSystems = lib.genAttrs systems;
      nixpkgsFor = forAllSystems (system: nixpkgs.legacyPackages.${system});
      treefmtFor = forAllSystems (system: treefmt-nix.lib.evalModule nixpkgsFor.${system} ./treefmt.nix);
    in
    {
      checks = forAllSystems (system: {
        treefmt = treefmtFor.${system}.config.build.check self;
      });

      devShells = forAllSystems (
        system:
        let
          pkgs = nixpkgsFor.${system};
        in
        {
          default = pkgs.mkShell {
            packages = [
              pkgs.clippy
              pkgs.rustfmt
              pkgs.rust-analyzer

              self.formatter.${system}
              pkgs.nil
              pkgs.statix
            ];

            inputsFrom = [ self.packages.${system}.teawie-api ];

            env = {
              RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
            };
          };
        }
      );

      formatter = forAllSystems (system: treefmtFor.${system}.config.build.wrapper);

      nixosModules.default = import ./nix/module.nix self;

      packages = forAllSystems (
        system:
        let
          pkgs = nixpkgsFor.${system};
          packages' = self.packages.${system};

          staticFor = pkgs.callPackage ./nix/static.nix {
            inherit (packages') teawie-api;
            fenix = fenix.packages.${system};
          };

          containerFor =
            teawie-api:
            let
              arch = teawie-api.stdenv.hostPlatform.ubootArch;
            in
            pkgs.dockerTools.buildLayeredImage {
              name = "teawie-api";
              tag = "latest-${arch}";
              config.Cmd = [ (lib.getExe teawie-api) ];
              architecture = arch;
            };
        in
        {
          container-x86_64 = containerFor packages'.static-x86_64;
          container-aarch64 = containerFor packages'.static-aarch64;

          static-x86_64 = staticFor "x86_64";
          static-aarch64 = staticFor "aarch64";

          teawie-api = pkgs.callPackage ./nix/package.nix { };
          default = self.packages.${pkgs.system}.teawie-api;
        }
      );
    };
}
