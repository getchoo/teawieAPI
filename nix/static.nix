{
  lib,
  arch,
  teawieapi,
  fenix,
  pkgsCross,
}: let
  crossTargets = with pkgsCross; {
    x86_64 = musl64.pkgsStatic;
    aarch64 = aarch64-multiplatform.pkgsStatic;
  };

  rustStdFor = pkgs: fenix.targets.${pkgs.stdenv.hostPlatform.rust.rustcTarget}.stable.rust-std;
  toolchain = with fenix;
    combine (lib.flatten [
      stable.cargo
      stable.rustc
      (lib.mapAttrsToList (lib.const rustStdFor) crossTargets)
    ]);

  rustPlatformFor = pkgs:
    pkgs.makeRustPlatform (
      lib.genAttrs ["cargo" "rustc"] (lib.const toolchain)
    );
  crossPlatforms = lib.mapAttrs (lib.const rustPlatformFor) crossTargets;
in
  teawieapi.override {
    rustPlatform = crossPlatforms.${arch};
    optimizeSize = true;
  }
