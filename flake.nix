{
  inputs = {
    nixpkgs = {
      type = "github";
      owner = "NixOS";
      repo = "nixpkgs";
      ref = "nixos-unstable";
    };
  };

  outputs = {
    self,
    nixpkgs,
  }: let
    supportedSystems = ["x86_64-linux"];

    perSystem = attrs:
      nixpkgs.lib.genAttrs supportedSystems (system: let
        pkgs = nixpkgs.legacyPackages.${system};
      in
        attrs system pkgs);
  in {
    packages = perSystem (system: pkgs: {
      default = pkgs.callPackage ({rustPlatform, ...}: let
        cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
        rev = self.shortRev or self.dirtyShortRev or "dirty";
      in
        rustPlatform.buildRustPackage {
          pname = "bazarr-bulk";
          version = "${cargoToml.package.version}-${rev}";
          src = ./.;
          strictDeps = true;
          cargoLock.lockFile = ./Cargo.lock;
        }) {};
    });

    formatter = perSystem (_: pkgs: pkgs.alejandra);

    devShells = perSystem (_: pkgs: {
      default = pkgs.mkShell {
        packages = with pkgs; [
          alejandra
          # ... more dev packages
        ];
      };
    });
  };
}
