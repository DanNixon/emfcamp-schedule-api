{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-23.11";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = (import nixpkgs) {
          inherit system;
        };

        rustPlatform = pkgs.makeRustPlatform {
          cargo = pkgs.cargo;
          rustc = pkgs.rustc;
        };

        cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);

        version = cargoToml.workspace.package.version;
        gitRevision = self.shortRev or self.dirtyShortRev;

        lintingRustFlags = "-D unused-crate-dependencies";
      in rec {
        devShell = pkgs.mkShell {
          packages = with pkgs; [
            # Rust toolchain
            cargo
            rustc

            # Code analysis tools
            clippy
            rust-analyzer

            # Code formatting tools
            treefmt
            alejandra
            rustfmt
            mdl

            # Rust dependency linting
            cargo-deny

            # Container image management tool
            skopeo
          ];

          RUSTFLAGS = lintingRustFlags;
        };

        packages =
          import ./adapter {inherit pkgs rustPlatform version gitRevision;}
          // import ./cli {inherit pkgs rustPlatform version gitRevision;};
      }
    );
}
