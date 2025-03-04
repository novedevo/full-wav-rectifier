{
  # inspired heavily by xe iaso's go template and https://dev.to/misterio/how-to-package-a-rust-app-using-nix-3lh3
  description = "meow";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.11";    
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, fenix, nixpkgs }:

  let 

    # build matrix
    arch = ["x86_64" "aarch64"];
    kernel = ["linux" "darwin"];
    supportedSystems = nixpkgs.lib.mapCartesianProduct (x: "${x.arch}-${x.kernel}") {inherit arch kernel;}; # hehe cartesian product

    forAllSystems = nixpkgs.lib.genAttrs supportedSystems; # partial application :3
    nixpkgsFor = forAllSystems (system: import nixpkgs { inherit system; });

  in  {
    # packages = forAllSystems (system:
    #   let pkgs = nixpkgsFor.${system}; in
    #   {
    #     default = pkgs.rustPlatform.buildRustPackage {
    #       pname = "repiquemos";
    #       src = pkgs.lib.cleanSource ./.;
    #       cargoLock.lockFile = ./Cargo.lock;
    #       nativeBuildInputs = [ pkgs.clippy ];
    #       buildInputs = [ pkgs.clippy ];
    #     };
    #   }
    # );

    # apps = forAllSystems (system: {
    #   default = {
    #     type = "app";
    #     program = "${self.packages.${system}.default}/bin/repiquemos";
    #   };
    # });

    devShells = forAllSystems (system: 
      let 
        pkgs = nixpkgsFor.${system};
      in {
        default = pkgs.mkShell {
          nativeBuildInputs = [
            fenix.packages.${system}.stable.toolchain
          ];
        };
      }
      );

  };
}
