{
  lib,
  rustPlatform,
  self,
  lto ? true,
  optimizeSize ? false,
}:
rustPlatform.buildRustPackage {
  pname = "teawie-api";
  version = (lib.importTOML ../teawie_api/Cargo.toml).package.version + "-" + self.shortRev or self.dirtyShortRev or "unknown";

  src = lib.fileset.toSource {
    root = ../.;
    fileset = lib.fileset.gitTracked ../.;
  };

  cargoLock = {
    lockFile = ../Cargo.lock;
  };

  env = let
    toRustFlags = lib.mapAttrs' (
      name:
        lib.nameValuePair
        "CARGO_BUILD_RELEASE_${lib.toUpper (builtins.replaceStrings ["-"] ["_"] name)}"
    );
  in
    lib.optionalAttrs lto (toRustFlags {
      lto = "thin";
    })
    // lib.optionalAttrs optimizeSize (toRustFlags {
      codegen-units = 1;
      opt-level = "s";
      panic = "abort";
      strip = "symbols";
    });

  meta = with lib; {
    mainProgram = "server";
    description = "okay so like basically, it's just a web service for teawie stuff (so cool!!)";
    homepage = "https://github.com/getchoo/teawieAPI";
    license = licenses.mit;
    maintainers = with maintainers; [getchoo];
  };
}
