
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
  ];

  env = {
  };



  # See full reference at https://devenv.sh/reference/options/
}
