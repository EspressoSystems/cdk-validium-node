{
  description = "cdk-validium-node, with go development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    flake-compat.url = "github:edolstra/flake-compat";
    flake-compat.flake = false;
    pre-commit-hooks = { 
      url = "github:cachix/pre-commit-hooks.nix";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-compat.follows = "flake-compat";
      };
    };
  };

  outputs = { self, nixpkgs, flake-utils, flake-compat, pre-commit-hooks }:
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = import nixpkgs { inherit system; };
      in with pkgs; {
        checks = {
          pre-commit-check = pre-commit-hooks.lib.${system}.run {
            src = ./.;
            hooks = {
              nixpkgs-fmt.enable = true;
              go-fmt = {
                enable = true;
                description = "Enforce go fmt";
                entry = "go fmt";
                types_or = [ "go" ];
                pass_filenames = true;
              };
            };
          };
        };

        devShells.default =
          let # nixWithFlakes allows pre v2.4 nix installations to use
            # flake commands (like `nix flake update`)
            nixWithFlakes = pkgs.writeShellScriptBin "nix" ''
              exec ${pkgs.nixFlakes}/bin/nix --experimental-features "nix-command flakes" "$@"
            '';
          in
          mkShell {
            buildInputs = [
              go
              protobuf
              docker
              docker-compose

              git
              nixWithFlakes
              nixpkgs-fmt
            ];
            shellHook = ''
              # avoid polluting system installed GOPATH
              export GOPATH=$PWD/.go-nix
              export PATH=$GOPATH/bin:$PATH
            '' + self.checks.${system}.pre-commit-check.shellHook;
          };
      });
}
