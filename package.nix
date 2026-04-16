{
  lib,
  rustPlatform,
  pkg-config,
  openssl,
}:
let
  manifest = (lib.importTOML ./Cargo.toml).package;
in
rustPlatform.buildRustPackage {
  pname = manifest.name;
  version = manifest.version;

  cargoLock.lockFile = ./Cargo.lock;
  src = lib.cleanSource ./.;

  buildInputs = [
    pkg-config
    openssl
  ];

  postInstall = ''
    mkdir -p $out/static
    cp -r ./static $out/static
  '';
}
