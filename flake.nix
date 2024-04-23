{
  inputs.nixpkgs.url = "nixpkgs/nixpkgs-unstable";

  outputs = {
    self,
    nixpkgs,
  }: let
    systems = [
      "x86_64-linux"
      "aarch64-linux"
      "x86_64-darwin"
      "aarch64-darwin"
    ];

    forAllSystems = fn: nixpkgs.lib.genAttrs systems (system: fn nixpkgs.legacyPackages.${system});
  in {
    checks = forAllSystems ({
      pkgs,
      lib,
      ...
    }: {
      alejandra = pkgs.runCommand "check-alejandra" {} ''
        ${lib.getExe pkgs.alejandra} --check ${../.}
        touch $out
      '';

      actionlint = pkgs.runCommand "check-actionlint" {} ''
        ${lib.getExe pkgs.actionlint} ${../.github/workflows}/*
        touch $out
      '';

      biome-fmt = pkgs.runCommand "check-biome-fmt" {} ''
        ${lib.getExe pkgs.biome} format ${../.}/*
        touch $out
      '';

      biome-lint = pkgs.runCommand "check-biome-lint" {} ''
        ${lib.getExe pkgs.biome} lint ${../.}/**/*
        touch $out
      '';

      deadnix = pkgs.runCommand "check-deadnix" {} ''
        ${lib.getExe pkgs.deadnix} --fail ${../.}
        touch $out
      '';

      editorconfig = pkgs.runCommand "check-editorconfig" {} ''
        cd ${../.}
        ${lib.getExe pkgs.editorconfig-checker} \
          -exclude '.git|node_modules|dist' .
        touch $out
      '';

      statix = pkgs.runCommand "check-statix" {} ''
        ${lib.getExe pkgs.statix} check ${../.}
        touch $out
      '';
    });

    devShells = forAllSystems (pkgs: let
      common = [pkgs.nodejs pkgs.corepack pkgs.wrangler];
    in {
      default = pkgs.mkShellNoCC {
        packages = common;
      };

      full = pkgs.mkShellNoCC {
        packages =
          common
          ++ [
            # lsp
            pkgs.nodePackages.typescript-language-server

            # formatting/lint
            pkgs.actionlint
            pkgs.biome
            pkgs.editorconfig-checker

            # nix
            self.formatter.${pkgs.system}
            pkgs.deadnix
            pkgs.statix
            pkgs.nil
          ];
      };
    });

    formatter = forAllSystems (pkgs: pkgs.alejandra);
  };
}
