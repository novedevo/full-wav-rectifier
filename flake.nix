{
  # inspired heavily by xe iaso's go template and https://dev.to/misterio/how-to-package-a-rust-app-using-nix-3lh3
  description = "meow";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.11";
  };

  outputs = { self, nixpkgs }:

  let 
    version = builtins.substring 0 8 self.lastModifiedDate; # e.g. 20240228 for Feb 28th 2024

    # build matrix
    arch = ["x86_64" "aarch64"];
    kernel = ["linux" "darwin"];
    supportedSystems = nixpkgs.lib.mapCartesianProduct (x: "${x.arch}-${x.kernel}") {inherit arch kernel;}; # hehe cartesian product

    forAllSystems = nixpkgs.lib.genAttrs supportedSystems; # partial application :3
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

    apps = forAllSystems (system: {
      default = {
        type = "app";
        program = "${self.packages.${system}.default}/bin/repiquemos";
      };
    });

    devShells = forAllSystems (system: 
      let pkgs = nixpkgsFor.${system};
      in {
        default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustc 
            rust-analyzer 
            rustfmt 
            vscode-extensions.rust-lang.rust-analyzer
            clippy
          ];
        };
      }
      );

  };
}
