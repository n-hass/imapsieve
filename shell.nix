with import <nixpkgs> {};

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustc
    cargo
    rustfmt
    rust-analyzer
    clippy
    lldb
    
    pkg-config
    systemd
    openssl
  ];

  RUST_BACKTRACE = 1;
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

  NIX_LD_LIBRARY_PATH = lib.makeLibraryPath [
    stdenv.cc.cc
    openssl
    glibc
    libz
  ];
}