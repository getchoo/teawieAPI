{
  inputs = {
    nixpkgs.url = "nixpkgs/nixpkgs-unstable";

    parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };

    pre-commit = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.nixpkgs-stable.follows = "nixpkgs";
    };
  };

  outputs = {
    parts,
    pre-commit,
    ...
  } @ inputs:
    parts.lib.mkFlake {inherit inputs;} {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      imports = [pre-commit.flakeModule];

      perSystem = {
        config,
        pkgs,
        ...
      }: {
        devShells.default = pkgs.mkShell {
          shellHook = ''
            [ ! -d node_modules ] && pnpm install --frozen-lockfile
            ${config.pre-commit.installationScript}
          '';

          packages = with pkgs; [
            nodejs_20
            (nodePackages_latest.pnpm.override {nodejs = nodejs_20;})

            actionlint
            editorconfig-checker

            config.formatter
            deadnix
            nil
            statix
          ];
        };

        formatter = pkgs.alejandra;

        pre-commit.settings = {
          hooks = {
            actionlint.enable = true;
            editorconfig-checker.enable = true;

            # typescript
            eslint.enable = true;
            prettier.enable = true;

            # nix
            ${config.formatter.pname}.enable = true;
            deadnix.enable = true;
            nil.enable = true;
            statix.enable = true;
          };

          settings = {
            eslint.extensions = "\\.(js|jsx|ts|tsx)$";
          };
        };
      };
    };
}
