import 'docker.just'

image_name := "ghcr.io/rust-basel/oxidation"

run *args:
    cargo run {{args}}




@verify: test lint
    echo ------------ verify done! ------------

# Run tests
test:
    cargo test
    
# Run the static code analysis
lint:
    cargo fmt --all -- --check
    cargo clippy

fmt:
    cargo fmt