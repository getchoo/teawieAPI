{
  lib,
  arch,
  teawieapi,
  fenix,
  pkgsCross,
}: let
  crossTargetFor = with pkgsCross; {
    x86_64 = musl64.pkgsStatic;
    aarch64 = aarch64-multiplatform.pkgsStatic;
  };

  rustcTargetFor = lib.mapAttrs (lib.const (pkgs: pkgs.stdenv.hostPlatform.rustcTarget)) crossTargetFor;
  rustStdFor = lib.mapAttrs (lib.const (rustcTarget: fenix.targets.${rustcTarget}.stable.rust-std)) rustcTargetFor;

  toolchain = with fenix;
    combine (
      [stable.cargo stable.rustc]
      ++ lib.attrValues rustStdFor
    );

  crossPlatformFor =
    lib.mapAttrs (
      lib.const (pkgs:
        pkgs.makeRustPlatform (
          lib.genAttrs ["cargo" "rustc"] (lib.const toolchain)
        ))
    )
    crossTargetFor;
in
  teawieapi.override {
    rustPlatform = crossPlatformFor.${arch};
    optimizeSize = true;
  }
