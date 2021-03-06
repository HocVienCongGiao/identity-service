# Check identity-app
clear
cd ../identity-app &&
cargo fmt --all -- --check &&
cargo clippy --all-targets -- -D clippy::all &&
cargo check --all
cargo build
cargo test
echo "identity-app checked"
# Check identity-lambda
cd ../identity-lambda &&
cargo fmt --all -- --check &&
cargo clippy --all-targets -- -D clippy::all &&
cargo check --all
cargo build
cargo test
# Check cognito-gateway-app
cd ../cognito-gateway-app &&
cargo fmt --all -- --check &&
cargo clippy --all-targets -- -D clippy::all &&
cargo check --all
cargo build
cargo test
echo "identity-lambda checked"
cd ../identity-dynamodb-lambda &&
cargo fmt --all -- --check &&
cargo clippy --all-targets -- -D clippy::all &&
cargo check --all
cargo build
cargo test
echo "identity-dynamodb-lambda checked"