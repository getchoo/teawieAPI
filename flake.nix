{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
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

          basePackages = [
            pkgs.nodejs
            pkgs.corepack

            pkgs.nrr
            pkgs.wrangler
          ];
        in
        {
          default = pkgs.mkShellNoCC {
            packages = basePackages ++ [
              pkgs.typescript-language-server
              pkgs.vscode-langservers-extracted # for eslint server

              # github actions
              pkgs.actionlint

              # nix
              self.formatter.${system}
              pkgs.nil
              pkgs.statix
            ];
          };

          ci = pkgs.mkShellNoCC { packages = basePackages; };
        }
      );

      formatter = forAllSystems (system: treefmtFor.${system}.config.build.wrapper);
    };
}
