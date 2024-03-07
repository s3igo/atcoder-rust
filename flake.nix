{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    dotfiles.url = "github:s3igo/dotfiles";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      dotfiles,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        tasks = import ./tasks.nix { inherit nixpkgs system; };
        neovim = self.neovim.${system} [ ];
      in
      {
        neovim =
          modules:
          dotfiles.neovim.${system} {
            inherit pkgs;
            modules =
              with dotfiles.nixosModules;
              [
                im-select
                nix
              ]
              ++ modules;
          };

        packages = {
          inherit neovim;
        };

        devShells.default = pkgs.mkShell { buildInputs = tasks ++ [ neovim ]; };
      }
    );
}
