{ lib, rustPlatform }:
let
  manifest = (lib.importTOML ./Cargo.toml).package;
in
rustPlatform.buildRustPackage {
  pname = manifest.name;
  version = manifest.version;

  cargoLock.lockFile = ./Cargo.lock;
  src = lib.cleanSource ./.;

  postInstall = ''
    mkdir -p $out/static
    cp -r ./static $out/static
  '';
}
