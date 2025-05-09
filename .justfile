import 'docker.just'

image_name := "ghcr.io/rust-basel/oxidation"

export RUST_LOG := "info"

run *args:
    cargo run {{args}}




@verify: test lint build-and-run-api-test
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
    
    
hurl_opts := "--variables-file api_tests/hurl_env --test --jobs 1"   
    
wait-for-api:
    hurl api_tests/health.hurl --retry 60 {{ hurl_opts }}

# run acceptance tests against the running test stack
api-test *args: wait-for-api
    hurl api_tests/*.hurl {{ hurl_opts }} {{ args }}


build-and-run-api-test: build (up "-d") api-test
    docker compose down    