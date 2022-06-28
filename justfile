# build the tool
build:
    cargo build --release

# install it to your local machine via cargo
install:
    cargo install --path .

# run the tests
test:
    cargo test

# lint the crate
lint:
    cargo check
    cargo clippy
