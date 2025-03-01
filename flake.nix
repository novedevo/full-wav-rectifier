{
  # inspired heavily by xe iaso's go template and https://dev.to/misterio/how-to-package-a-rust-app-using-nix-3lh3
  description = "meow";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.11";
  };

  outputs = { self, nixpkgs }:

  let 

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
        default = pkgs.rustPlatform.buildRustPackage {
          pname = "repiquemos";
          src = pkgs.lib.cleanSource ./.;
          cargoLock.lockFile = ./Cargo.lock;
          nativeBuildInputs = [ clippy ];
          buildInputs = [ clippy ];
        };
      }
    );

    apps = forAllSystems (system: {
      default = {
        type = "app";
        program = "${self.packages.${system}.default}/bin/repiquemos";
      };
    });

    # devShells = forAllSystems (system: 
    #   let 
    #     pkgs = nixpkgsFor.${system};
    #   in {
    #     default = pkgs.mkShell {
    #       # https://nixos.wiki/wiki/Rust#Shell.nix_example
    #       RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
    #       buildInputs = with pkgs; [
    #         rustc 
    #         rust-analyzer 
    #         rustfmt 
    #         vscode-extensions.rust-lang.rust-analyzer
    #         clippy
    #       ];
    #     };
    #   }
    #   );

  };
}
