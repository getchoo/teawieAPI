{
  inputs = {
    nixpkgs.url = "nixpkgs/nixpkgs-unstable";

    parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };

    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    { parts, treefmt-nix, ... }@inputs:
    parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      imports = [ treefmt-nix.flakeModule ];

      perSystem =
        { self', pkgs, ... }:
        {
          devShells = {
            default = pkgs.mkShellNoCC {
              packages = with pkgs; [
                # node
                nodejs_20
                corepack_20
                wrangler
                nrr
                typescript-language-server
                vscode-langservers-extracted # for eslint server

                # github actions
                actionlint

                # nix
                self'.formatter
                nil
                statix
              ];

              env = {
                # https://github.com/NixOS/nixpkgs/pull/330808
                SSL_CERT_FILE = "${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt";
              };
            };

            ci = pkgs.mkShellNoCC {
              shellHook = ''
                corepack install
              '';

              packages = with pkgs; [
                nodejs_20
                corepack_20
                nrr
              ];
            };
          };

          treefmt = {
            projectRootFile = ".git/config";

            programs = {
              actionlint.enable = true;
              deadnix.enable = true;
              nixfmt.enable = true;
              prettier.enable = true;
              statix.enable = true;
            };
          };
        };
    };
}
