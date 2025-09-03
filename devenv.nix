{ pkgs, ... }:
{
  packages = with pkgs; [
    git
    lld
    wasm-pack
  ];

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
