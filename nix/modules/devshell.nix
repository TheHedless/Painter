{ inputs, ... }:
{
  perSystem = { config, self', pkgs, lib, ... }:
    let
      libPath = with pkgs; lib.makeLibraryPath [
        libGL
        libxkbcommon
        wayland
      ];
    in
    {
      devShells.default = pkgs.mkShell {
        name = "Painter-shell";
        inputsFrom = [
          self'.devShells.rust
          config.pre-commit.devShell # See ./nix/modules/pre-commit.nix
        ];
        packages = with pkgs; [
          just
          nixd # Nix language server
          bacon
          config.process-compose.cargo-doc-live.outputs.package
        ];

        RUST_LOG = "debug";
        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        LD_LIBRARY_PATH = libPath;
      };
    };
}
