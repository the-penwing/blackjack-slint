# list recipes
default:
  just --list

# run the app inside the nix shell
run:
  nix develop -c cargo run

# clippy
clippy:
  nix develop -c cargo clippy -- -D warnings

# format
fmt:
  nix develop -c cargo fmt
  nix develop -c slint-lsp format --inline **/*.slint
  nix develop -c alejandra .

# check (format -> clippy)
check: fmt clippy

# build release binary
build:
 nix develop -c cargo build --release
