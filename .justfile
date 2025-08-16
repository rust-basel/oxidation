set dotenv-load


export RUST_LOG := "debug"

run *args:
    cargo run {{args}}


@verify: test lint
    echo ------------ verify done! ------------

# Run tests
test:
    cargo test
    
# Run the static code analysis
lint:
    cargo fmt --all --check
    cargo clippy -- -Dwarnings

fmt:
    cargo fmt
    cargo fix --allow-dirty
    
