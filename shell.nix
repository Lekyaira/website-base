{ pkgs ? import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/nixos-24.11.tar.gz") {} }:
let
# Rust nightly with wasm32-unknown-unknown target.
  f = import (fetchTarball "https://github.com/nix-community/fenix/archive/main.tar.gz") {};
  fenix = f.combine [
    f.complete.toolchain
    f.targets.wasm32-unknown-unknown.latest.rust-std
  ];
  # Get project directory.
  pd = builtins.toString ./.;
in
pkgs.mkShell {
  packages = with pkgs; [
    fenix 
    cargo-leptos
    cargo-generate
    llvmPackages.bintools
    wasm-pack
    mysql80
    dart-sass
  ];

# Fix Cargo build errors and contain everything for reproducibility.
  TMPDIR = "${pd}/target";
# Fix WASM linker issues.
  CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER = "lld";

# MySQL data directory.
  MYSQL_DATA = "${pd}/.nix/mysql/data";
# MySQL Unix port file location.
  MYSQL_UNIX_PORT = "${pd}/.nix/mysql/mysql.sock";

  shellHook = ''
    #### MYSQL ####
    MYSQL_HOME=${pd}/.nix/mysql 
    # export MYSQL_DATA=$MYSQL_HOME/data 
    # export MYSQL_UNIX_PORT=$MYSQL_HOME/mysql.sock

    mkdir -p $MYSQL_HOME 
    # Dunno why we need this but `mysqld` won't initializs without it. 
    mkdir $TMPDIR 

    # Initialize MySQL data without password.
    # Since this is for dev builds only and we're using internal Unix 
    # ports that shouldn't be open to the world anyway, we don't 
    # really need a password.
    if [ ! -d $MYSQL_DATA ]; then 
      mysqld --initialize-insecure --datadir=$MYSQL_DATA 
    fi 

    #### Bootstrap ####
    BOOTSTRAP=${pd}/bootstrap

    # Pull in Bootstrap if we don't already have it.
    # Version is pinned, change it here and delete `bootstrap`
    # directory to upgrade.
    if [ ! -d $BOOTSTRAP ]; then 
      git clone https://github.com/twbs/bootstrap --branch v5.3.3
    fi

    # Delete the Bootstrap `.git` since Git will complain about it 
    # and this repo isn't for contributing anyway.
    rm -rf bootstrap/.git

    #### Utils ####
    BINDIR=${pd}/.nix/bin 
    export PATH=$PATH:$BINDIR
  '';
}
