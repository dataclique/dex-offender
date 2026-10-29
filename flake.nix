{
  inputs = {
    devenv.url = "github:cachix/devenv";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    foundry = {
      url = "github:shazow/foundry.nix/monthly";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      fenix,
      foundry,
      devenv,
    }@inputs:
    flake-utils.lib.eachSystem [ "aarch64-darwin" "aarch64-linux" "x86_64-linux" ] (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        foundry-pkg = foundry.defaultPackage.${system};
        forge = "${foundry-pkg}/bin/forge";
        channel = fenix.packages.${system}.stable;

        # Official sui CLI from the MystenLabs release binaries, same
        # packaging as dataclique/nth (per-system sha256 pins). Candidate
        # for extraction into a shared dataclique/sui.nix flake input.
        suiPkg =
          let
            baseUrl = "https://github.com/MystenLabs/sui/releases/download";
            version = "v1.76.1";
            assets = {
              aarch64-darwin = {
                suffix = "macos-arm64";
                hash = "sha256-/F5Gh1AbdRLu1q07PLwe8ls8loJqyyhOPGwHdp+00Ms=";
              };
              aarch64-linux = {
                suffix = "ubuntu-aarch64";
                hash = "sha256-exjWtHfSoQM3XD+d8nYh8epiWdmNO7aPmAiGNfAg3Y8=";
              };
              x86_64-linux = {
                suffix = "ubuntu-x86_64";
                hash = "sha256-uKgmoJiKGnIfrGZ/4T00Azw7AAPw45cH4yXMnTU2jvM=";
              };
            };
            asset = assets.${system} or (throw "sui ${version} has no prebuilt binary for ${system}");
          in
          pkgs.stdenv.mkDerivation {
            name = "sui";
            src = pkgs.fetchzip {
              url = "${baseUrl}/testnet-${version}/sui-testnet-${version}-${asset.suffix}.tgz";
              inherit (asset) hash;
              stripRoot = false;
            };
            installPhase = ''
              mkdir -p $out/bin
              cp -v sui $out/bin
            '';
          };
      in
      {
        packages.sui = suiPkg;

        devShells.default = devenv.lib.mkShell {
          inherit inputs pkgs;

          modules = [
            {
              packages = [
                pkgs.solc
                pkgs.gcc
                pkgs.cargo-watch
                foundry-pkg
                pkgs.go-ethereum
                pkgs.nushell
                pkgs.deno
                pkgs.taplo
                suiPkg
              ]
              ++ pkgs.lib.optionals pkgs.stdenv.hostPlatform.isDarwin [ pkgs.libiconv ];

              # dotenv is CLI-only in current devenv; never auto-load secrets here.
              env.FOUNDRY_DISABLE_NIGHTLY_WARNING = "1";

              difftastic.enable = true;

              # https://devenv.sh/languages/
              languages.nix.enable = true;
              languages.rust = {
                enable = true;
                toolchainPackage = channel.toolchain;
              };

              # NOTE: ABI bindings are generated via alloy sol! macros loading
              # ctf/abi/*.json artifacts (see scripts/build-abis.nu). forge
              # bind is no longer used.

              scripts.watch-levels.exec = ''
                set -e
                cd "$(git -c core.fsmonitor=false rev-parse --show-toplevel)"
                cargo watch --poll --no-vcs-ignores --no-dot-ignores \
                  -w attack/src -w ctf/src \
                  -w Cargo.toml -w attack/Cargo.toml -w ctf/Cargo.toml \
                  -x run
              '';

              scripts.deploy-levels.exec = ''
                if [ -f state.json ]; then
                  rm -v state.json
                fi
                cargo run -p ctf --bin deploy_levels
              '';
              # https://devenv.sh/pre-commit-hooks/
              git-hooks.hooks = {
                nixfmt = {
                  enable = true;
                  fail_fast = true;
                  package = pkgs.nixfmt;
                  excludes = [ "^lib/" ];
                };
                rustfmt = {
                  enable = true;
                  packageOverrides = { inherit (channel) cargo rustfmt; };
                  excludes = [ "^lib/" ];
                };
                taplo = {
                  enable = true;
                  excludes = [ "^lib/" ];
                };
                denofmt = {
                  enable = true;
                  files = "\\.md$";
                  excludes = [ "^lib/" ];
                };
              };
            }
          ];
        };
      }
    );
}
