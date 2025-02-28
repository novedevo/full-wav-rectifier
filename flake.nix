{
  # inspired heavily by xe iaso's go template
  description = "meow";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.11";
  };

  outputs = { self, nixpkgs }:

    let version = builtins.substring 0 8 self.lastModifiedDate;
    in  {

    


  };
}
