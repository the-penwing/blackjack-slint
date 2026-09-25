{
  pkgs,
  rustToolchain,
}:
pkgs.mkShell {
  nativeBuildInputs = [pkgs.pkg-config rustToolchain];
  name = "blackjack-slint";
  packages = with pkgs; [
    just
    slint-viewer
    slint-lsp
    alejandra
  ];
  buildInputs = with pkgs; [
    fontconfig
    libxkbcommon
    wayland
    wayland-protocols
    vulkan-loader
    libGL
    mesa
  ];
  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
    pkgs.libxkbcommon
    pkgs.wayland
    pkgs.vulkan-loader
    pkgs.fontconfig
    pkgs.libGL
    pkgs.mesa
  ];
}
