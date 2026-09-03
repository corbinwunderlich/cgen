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
        config,
        self',
        ...
      }: {
        packages = {
          cgen = pkgs.callPackage ./default.nix {};

          default = self'.packages.cgen;
        };

        pre-commit.settings = {
          hooks = {
            alejandra.enable = true;

            deadnix.enable = true;

            statix.enable = true;

            taplo.enable = true;

            mdformat = {
              enable = true;
              excludes = ["CONFIG.md" "USAGE.md"];
            };

            rustfmt.enable = true;

            clippy = {
              enable = true;
              settings = {
                extraArgs = "--fix --allow-dirty";
              };
            };

            xtask = {
              enable = true;

              name = "cargo xtask";
              entry = let
                entry = pkgs.writeShellScript "cargo-xtask" ''
                  ${pkgs.cargo}/bin/cargo xtask manpages
                  ${pkgs.cargo}/bin/cargo xtask completions
                  ${pkgs.cargo}/bin/cargo xtask json-schema
                '';
              in "${entry}";

              pass_filenames = false;
            };

            json-schema-for-humans = {
              enable = true;

              name = "json schema for humans";
              entry = "${pkgs.json-schema-for-humans}/bin/generate-schema-doc config.schema.json CONFIG.md --config-file jsfh.yaml";

              pass_filenames = false;
            };

            generate-usage = {
              enable = true;

              name = "generate USAGE.md";
              entry = let
                entry = pkgs.writeShellScript "generate-usage" ''
                  cat > USAGE.md << EOF
                  \`\`\`
                  $(${pkgs.cargo}/bin/cargo run --quiet -- --help)
                  \`\`\`
                  EOF
                '';
              in "${entry}";

              pass_filenames = false;
            };
          };
        };

        devShells.default = pkgs.mkShell {
          packages = with pkgs; [cargo libclang json-schema-for-humans] ++ config.pre-commit.settings.enabledPackages;

          inherit (config.pre-commit) shellHook;

          inherit (self'.packages.cgen) LIBCLANG_PATH;
        };
      };
    };
}
