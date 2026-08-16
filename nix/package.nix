{
  pkgs,
  craneLib,
}:
craneLib.buildPackage {
  src = pkgs.lib.fileset.toSource {
    root = ../.;
    fileset = pkgs.lib.fileset.unions [
      (craneLib.fileset.commonCargoSources ../.)
      ../ui
      ../assets
    ];
  };
  strictDeps = true;
  nativeBuildInputs = [pkgs.pkg-config pkgs.makeWrapper];
  buildInputs = with pkgs; [
    fontconfig
    libxkbcommon
    wayland
    wayland-protocols
    vulkan-loader
    libGL
    mesa
  ];
  postInstall = ''
    wrapProgram $out/bin/blackjack-slint \
      --set LD_LIBRARY_PATH ${pkgs.lib.makeLibraryPath [
      pkgs.libxkbcommon
      pkgs.wayland
      pkgs.vulkan-loader
      pkgs.fontconfig
      pkgs.libGL
      pkgs.mesa
    ]}
  '';
}
