{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-26.05";

    flake-parts.url = "github:hercules-ci/flake-parts";

    systems.url = "github:nix-systems/default";

    git-hooks = {
      url = "github:cachix/git-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    flake-parts,
    systems,
    git-hooks,
    ...
  } @ inputs:
    flake-parts.lib.mkFlake {inherit inputs;} {
      imports = [git-hooks.flakeModule];

      systems = import systems;

      perSystem = {
        pkgs,
        lib,
        config,
        self',
        ...
      }: {
        packages = {
          cgen = let
            cargo-toml = fromTOML (builtins.readFile ./Cargo.toml);
          in
            pkgs.rustPlatform.buildRustPackage {
              pname = cargo-toml.package.name;
              version = cargo-toml.package.version;

              src = ./.;

              nativeBuildInputs = with pkgs; [installShellFiles];

              cargoLock.lockFile = ./Cargo.lock;

              LIBCLANG_PATH = lib.makeLibraryPath (with pkgs; [libclang]);

              postInstall = ''
                installManPage target/man/cgen.1

                installShellCompletion target/completions/cgen.{bash,fish,nu}
                installShellCompletion --zsh target/completions/_cgen
              '';
            };

          default = self'.packages.cgen;
        };

        pre-commit.settings = {
          hooks = {
            alejandra.enable = true;

            mdformat.enable = true;

            rustfmt.enable = true;

            clippy = {
              enable = true;
              settings = {
                extraArgs = "--fix --allow-dirty";
              };
            };

            tests = {
              enable = true;

              name = "Tests";
              entry = "${pkgs.cargo}/bin/cargo test";

              pass_filenames = false;
            };
          };
        };

        devShells.default = pkgs.mkShell {
          packages = with pkgs; [cargo libclang] ++ config.pre-commit.settings.enabledPackages;

          inherit (config.pre-commit) shellHook;

          inherit (self'.packages.cgen) LIBCLANG_PATH;
        };
      };
    };
}
