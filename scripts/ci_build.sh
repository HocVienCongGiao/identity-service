# Check identity-app
cd ../identity-app &&
cargo fmt --all -- --check &&
cargo clippy --all-targets -- -D clippy::all &&
cargo check --all
echo "identity-app checked successfully"
# Check identity-lambda
cd ../identity-lambda && cargo fmt --all -- --check &&
cargo clippy --all-targets -- -D clippy::all &&
cargo check --all
echo "identity-lambda checked successfully"