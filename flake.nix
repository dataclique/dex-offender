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
      in
      {
        devShells.default = devenv.lib.mkShell {
          inherit inputs pkgs;

          modules = [
            {
              packages = [
                pkgs.solc
                pkgs.gcc
                foundry-pkg
                pkgs.go-ethereum
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

              scripts.bind-attack.exec = ''
                ${forge} install
                ${forge} fmt
                ${forge} bind -b ./attack/src/abi --module --force --overwrite
                pre-commit run rustfmt -a > /dev/null || true
              '';

              # NOTE: forge bind was only used for ctf/contracts, which was
              # replaced by the pinned level submodules in lib/. ABI bindings
              # will be generated via alloy's sol! macro (see ctf/src/abi).

              scripts.deploy-levels.exec = ''
                if [ -f state.json ]; then
                  rm -v state.json
                fi
                cargo run --bin deploy_levels
              '';

              # https://devenv.sh/pre-commit-hooks/
              git-hooks.hooks = {
                nixfmt = {
                  enable = true;
                  fail_fast = true;
                  package = pkgs.nixfmt;
                };
                rustfmt = {
                  enable = true;
                  packageOverrides = { inherit (channel) cargo rustfmt; };
                };
                # bind-attack-contracts = {
                #   enable = true;
                #   name = "Bind attack contracts";
                #   description =
                #     "Build attack/contracts/ contracts and generate Rust ABI bindings";
                #   files = "attack/contracts/.*.sol$";
                #   entry = ".devenv/profile/bin/bind-attack";
                #   pass_filenames = false;
                #   verbose = true;
                # };
                # bind-ctf-contracts = {
                #   enable = true;
                #   name = "Bind CTF contracts";
                #   description =
                #     "Compile CTF smart contracts and generate Rust ABI bindings";
                #   files = "ctf/contracts/.*.sol$";
                #   entry = ".devenv/profile/bin/bind-ctfs";
                #   pass_filenames = false;
                #   verbose = true;
                # };
                # deploy-levels = {
                #   enable = true;
                #   name = "Deploy levels";
                #   files = "ctf/src/.*.rs";
                #   entry = ".devenv/profile/bin/deploy-levels";
                #   pass_filenames = false;
                #   verbose = true;
                # };
              };
            }
          ];
        };
      }
    );
}
