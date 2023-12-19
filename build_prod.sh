set -ex

cargo build --release

cargo doc --no-deps --all-features

cp -r ultra_16_gs/doc/images target/doc/ultra_16_gs/images
