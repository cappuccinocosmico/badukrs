
{ pkgs, lib, config, inputs, ... }:
{
  # Base configuration shared across all project devenvs
  dotenv.enable = true;
  dotenv.disableHint = true;
  cachix.enable = false;

  # Common packages available to all projects
  languages.rust.enable = true;

  packages = with pkgs; [
    openssl
    lld_21
    # Bevy dependencies
    pkg-config
    alsa-lib
    wayland
    wayland-protocols
    libxkbcommon
    libGL
    # X11 libraries
    xorg.libX11
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi
    # Vulkan
    vulkan-tools
    vulkan-headers
    vulkan-loader
    vulkan-validation-layers
    udev
  ];

  env = {
    LD_LIBRARY_PATH = lib.makeLibraryPath [
      pkgs.libxkbcommon
      pkgs.wayland
      pkgs.vulkan-loader
      pkgs.libGL
      pkgs.xorg.libX11
      pkgs.xorg.libXcursor
      pkgs.xorg.libXrandr
      pkgs.xorg.libXi
      pkgs.alsa-lib
    ];
  };



  # See full reference at https://devenv.sh/reference/options/
}
