with import <nixpkgs> {};

mkShell {
  nativeBuildInputs = [
    cmake
    pkg-config
  ];

  buildInputs = [
    pkg-config
    libxkbcommon

    fontconfig
    freetype
    expat
    libxml2
    glfw
    glew
    libGL

    xorg.libX11
    xorg.libXrandr
    xorg.libXinerama
    xorg.libXcursor
    xorg.libXi
    # Add other dependencies your project might need

    wayland
    wayland-protocols
  ];
  shellHook = ''
    export LIBGL_DRIVERS_PATH=${pkgs.mesa.drivers}
    export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath [
      pkgs.xorg.libX11
      pkgs.xorg.libXcursor
      pkgs.xorg.libXrandr
      pkgs.xorg.libXi
      pkgs.libGL
      pkgs.libxkbcommon
      pkgs.glfw
    ]}
  '';

}
