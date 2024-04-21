{
  lib,
  rustPlatform,
  self,
}:
rustPlatform.buildRustPackage {
  pname = "teawie-api";
  version = self.shortRev or self.dirtyShortRev or "unknown";

  src = lib.fileset.toSource {
    root = ../.;
    fileset = lib.fileset.gitTracked ../.;
  };

  cargoLock = {
    lockFile = ../Cargo.lock;
  };

  meta = with lib; {
    mainProgram = "server";
    description = "okay so like basically, it's just a web service for teawie stuff (so cool!!)";
    homepage = "https://github.com/getchoo/teawieAPI";
    license = licenses.mit;
    maintainers = with maintainers; [getchoo];
  };
}
