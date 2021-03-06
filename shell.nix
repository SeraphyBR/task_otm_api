# Use nixpkgs with oxalica rust-bin overlay
let
    rust_overlay = import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");
    nixpkgs = import <nixpkgs> { overlays = [ rust_overlay ]; };
    rust_channel = nixpkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
in
# Avoid typing `nixpkgs.` before each package name
with nixpkgs;

pkgs.mkShell {
    buildInputs = with pkgs; [ stdenv zlib pkg-config openssl ];
    # nativeBuildInputs is usually what you want -- tools you need to run
    nativeBuildInputs = with pkgs; [
        # Rust Compiler and Cargo
        rust_channel
        # Cargo extensions
        cargo-edit
        cargo-make
        # Docker
        docker-compose
        # Ngrok (Run my localhost as online url)
        ngrok
        # LP Solver
        cbc

        # Use steam-run, for a fhs enviroment, allow vscode code-lldb debugger to run
        # $ steam-run code .
        (steam.override {
            extraPkgs = pkgs: [ pkg-config zsh cbc ];
            extraLibraries = pkgs: [ zlib openssl ];
            nativeOnly = true;
        }).run
    ];
}
