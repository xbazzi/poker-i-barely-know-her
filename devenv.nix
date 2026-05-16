{
  pkgs,
  lib,
  config,
  inputs,
  ...
}:

{
  languages.rust = {
    enable = true;
    channel = "stable";
    components = [ "rustc" "cargo" "clippy" "rustfmt" "rust-analyzer" ];
  };

  packages = with pkgs; [
    cargo-watch
    cargo-edit
    cargo-nextest
  ];

  scripts = {
    dev.exec = "cargo watch -x run";
    check.exec = "cargo clippy -- -D warnings && cargo fmt --check";
  };
}
