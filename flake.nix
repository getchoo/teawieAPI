{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable-small";

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
      systems = lib.systems.flakeExposed;

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
            pkgs.deno
          ];
        in
        {
          default = pkgs.mkShellNoCC {
            packages = basePackages ++ [
              # CI Tools
              pkgs.actionlint

              # Nix Tools
              self.formatter.${system}
              pkgs.deadnix
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
