{
  lib,
  rustPlatform,
  lto ? true,
  optimizeSize ? false,
}:
let
  fs = lib.fileset;
in
rustPlatform.buildRustPackage rec {
  pname = "teawie-api";
  inherit ((lib.importTOML ../Cargo.toml).workspace.package) version;

  src = fs.toSource {
    root = ../.;
    fileset = fs.intersection (fs.gitTracked ../.) (
      fs.unions [
        ../Cargo.toml
        ../Cargo.lock

        ../server
        ../teawie-api
      ]
    );
  };

  cargoLock = {
    lockFile = ../Cargo.lock;
  };

  cargoBuildFlags = [
    "--package"
    "teawie-api-server"
  ];
  cargoTestFlags = cargoBuildFlags;

  env =
    let
      toRustFlags = lib.mapAttrs' (
        name:
        lib.nameValuePair "CARGO_BUILD_RELEASE_${
          lib.toUpper (builtins.replaceStrings [ "-" ] [ "_" ] name)
        }"
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

  meta = {
    description = "okay so like basically, it's just a web service for teawie stuff (so cool!!)";
    homepage = "https://github.com/getchoo/teawieAPI";
    license = lib.licenses.mit;
    maintainers = with lib.maintainers; [ getchoo ];
    mainProgram = "teawie-api";
    platforms = lib.platforms.unix;
  };
}
