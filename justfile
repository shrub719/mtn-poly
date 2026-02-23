[default]
dev:
    cargo build

build:
    cargo build --release

test:
    cargo run -- ./test/test.mtn -o ./test/output.mtb
