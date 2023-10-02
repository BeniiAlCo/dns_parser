{
  description = "A Nix-flake-based Rust development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      devShells.${system}.default = pkgs.mkShell
        {
          packages = with pkgs; [
            rustc
            cargo
            rust-analyzer
            rustfmt
            clippy
          ];

          RUST_BACKTRACE = "1";
        };
    };
}

