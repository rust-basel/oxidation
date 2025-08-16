set dotenv-load


export RUST_LOG := "debug"
export OX_TEST_WORK := "200"

run *args:
    cargo run {{args}}


@verify: test_run test lint
    echo ------------ verify done! ------------

# Run tests
test:
    cargo test

test_run:
    just run test

# Run the static code analysis
lint:
    cargo fmt --all --check
    cargo clippy -- -Dwarnings

fmt:
    cargo fmt
    cargo fix --allow-dirty
    __CARGO_FIX_YOLO=1 cargo clippy --fix --allow-dirty
