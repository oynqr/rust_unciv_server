{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

  outputs =
    { nixpkgs, ... }:
    let
      forAllSystems =
        function:
        nixpkgs.lib.genAttrs [
          "aarch64-darwin"
          "aarch64-linux"
          "x86_64-linux"
        ] (system: function nixpkgs.legacyPackages.${system});
    in
    {
      devShells = forAllSystems (
        pkgs: with pkgs; {
          default = mkShell {
            buildInputs = [
              cargo
              cargo-bloat
              cargo-duplicates
              cargo-outdated
              pre-commit
              rust-analyzer
              rustc
              rustfmt
              rustPackages.clippy
            ];
            RUST_SRC_PATH = rustPlatform.rustLibSrc;
          };
        }
      );

      packages = forAllSystems (pkgs: {
        default = pkgs.callPackage ./package.nix { };
      });
    };
}
