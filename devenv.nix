{ pkgs, ... }:
{
  packages = with pkgs; [
    git
    lld
    wasm-pack
  ];

  languages.rust = {
    enable = true;
    channel = "stable";
    components = [
      "rustc"
      "cargo"
      "clippy"
      "rustfmt"
      "rust-analyzer"
    ];
  };

  languages.javascript = {
    enable = true;
    directory = "./web";
    pnpm = {
      enable = true;
      install.enable = true;
    };
  };
}
