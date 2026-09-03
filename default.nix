{
  lib,
  rustPlatform,
  installShellFiles,
  libclang,
  cargo-toml ? fromTOML (builtins.readFile ./Cargo.toml),
}:
rustPlatform.buildRustPackage {
  pname = cargo-toml.package.name;
  version = cargo-toml.package.version;

  src = ./.;

  nativeBuildInputs = [installShellFiles];

  cargoLock.lockFile = ./Cargo.lock;

  LIBCLANG_PATH = lib.makeLibraryPath [libclang];

  postInstall = ''
    installManPage target/man/cgen.1
    installShellCompletion target/completions/cgen.{bash,fish,nu} --zsh target/completions/_cgen
  '';
}
