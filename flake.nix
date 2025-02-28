{
  # inspired heavily by xe iaso's go template and https://dev.to/misterio/how-to-package-a-rust-app-using-nix-3lh3
  description = "meow";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.11";
  };

  outputs = { self, nixpkgs }:

  let 
    version = builtins.substring 0 8 self.lastModifiedDate;
    supportedSystems = ["x86_64-linux"];
    forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
    nixpkgsFor = forAllSystems (system: import nixpkgs { inherit system; });
  in  {

    packages = forAllSystems (system:
      let pkgs = nixpkgsFor.${system}; in
      {
        default = pkgs.rustPlatform.buildRustPackage rec {
          pname = "repiquemos";
          inherit version;
          src = pkgs.lib.cleanSource ./.;
          cargoLock.lockFile = ./Cargo.lock;
        };
      }
    );

  };
}
