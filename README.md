# Leptos Axum MySQL Bootstrap Website Template

This it a basic template for a Leptos SSR website using Nix. Should be fully reproducible on most systems. Use it as you wish, and create an issue if you come across any, have any suggestions or recomendations.
Based on the [Leptos Axum Starter Template](https://github.com/leptos-rs/start-axum).


## Setting up your project 

```bash
nix-shell
```

Will enter the shell, download neccesary pieces and set environment variables.

Alternatively, if you have nix-direnv installed:

```bash
echo "use nix" > .envrc && direnv allow
```

If you are developing remotely, you will probably need to change the IP from localhost to your machine's IP. Edit `Cargo.toml` `site-addr` as neccesary.

## Running your project

```bash
run
```

Will start the web server for testing. Note: **this will not start the `mysqld` service!**

```bash
start
```

Will start the `mysqld` service.

```bash
stop
```

Will stop the `mysqld` service. Note that the service will continue to run when you exit the shell!

```bash
sql
```

Will open the `mysql` cli.

## Compiling for Release
```bash
cargo leptos build --release
```

Will generate your server binary in target/server/release and your site package in target/site

## Testing Your Project
```bash
cargo leptos end-to-end
```

```bash
cargo leptos end-to-end --release
```

Cargo-leptos uses Playwright as the end-to-end test tool.  
Tests are located in end2end/tests directory.

## Executing a Server on a Remote Machine Without the Toolchain
After running a `cargo leptos build --release` the minimum files needed are:

1. The server binary located in `target/server/release`
2. The `site` directory and all files within located in `target/site`

Copy these files to your remote server. The directory structure should be:
```text
leptos-sssr
site/
```
Set the following environment variables (updating for your project as needed):
```text
LEPTOS_OUTPUT_NAME="leptos-sssr"
LEPTOS_SITE_ROOT="site"
LEPTOS_SITE_PKG_DIR="pkg"
LEPTOS_SITE_ADDR="127.0.0.1:3000"
LEPTOS_RELOAD_PORT="3001"
```
Finally, run the server binary.

## Licensing

This template itself is released under the Unlicense. You should replace the LICENSE for your own application with an appropriate license if you plan to release it publicly.
