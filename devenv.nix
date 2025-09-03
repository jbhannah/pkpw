{ pkgs, ... }:
{
  packages = with pkgs; [ git ];

  languages.rust.enable = true;
  languages.javascript = {
    enable = true;
    directory = "./web";
    pnpm = {
      enable = true;
      install.enable = true;
    };
  };
}
