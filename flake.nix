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
    }: let
      formatter = self.formatter.${pkgs.system};
    in {
      deadnix = pkgs.runCommand "check-deadnix" {} ''
        ${lib.getExe pkgs.deadnix} --fail ${./.}
        touch $out
      '';

      editorconfig = pkgs.runCommand "check-editorconfig" {} ''
        cd ${./.}
        ${lib.getExe pkgs.editorconfig-checker} \
          -exclude '.git|target' .
        touch $out
      '';

      "${formatter.pname}" = pkgs.runCommand "check-${formatter.pname}" {} ''
        ${lib.getExe formatter} --check ${./.}
        touch $out
      '';

      rustfmt =
        pkgs.runCommand "check-rustfmt" {
          nativeBuildInputs = [pkgs.cargo pkgs.rustfmt];
        } ''
          cd ${./.}
          cargo fmt --check
          touch $out
        '';

      statix = pkgs.runCommand "check-statix" {} ''
        ${lib.getExe pkgs.statix} check ${./.}
        touch $out
      '';
    });

    devShells = forAllSystems (pkgs: let
      toolchain = [
        pkgs.rustfmt
        pkgs.clippy

        pkgs.worker-build
        pkgs.wasm-pack
      ];

      inputsFrom = [self.packages.${pkgs.system}.teawieapi];
      RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
    in {
      default = pkgs.mkShell {
        packages = toolchain;
        inherit inputsFrom RUST_SRC_PATH;
      };

      full = pkgs.mkShell {
        packages =
          toolchain
          ++ [
            pkgs.rust-analyzer

            self.formatter.${pkgs.system}
            pkgs.statix
            pkgs.deadnix
            pkgs.nil
          ];

        inherit inputsFrom RUST_SRC_PATH;
      };
    });

    formatter = forAllSystems (pkgs: pkgs.alejandra);

    nixosModules.default = import ./nix/module.nix self;

    packages = forAllSystems (pkgs: {
      teawieapi = pkgs.callPackage ./nix {inherit self;};
      default = self.packages.${pkgs.system}.teawieapi;
    });
  };
}
