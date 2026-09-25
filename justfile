# list recipes
default:
  just --list

# run the app
run:
  cargo run

# clippy
clippy:
  cargo clippy -- -D warnings

# format
fmt:
  cargo fmt
  slint-lsp format --inline **/*.slint
  alejandra .

# check (format -> clippy)
check: fmt clippy

# build release binary
build:
 cargo build --release
