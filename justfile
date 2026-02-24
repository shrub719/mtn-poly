[default]
dev:
    cargo build

build:
    cargo build --release

test:
    cargo run -- ./test/test.mtn -o ./test/output.mtb

run input output:
    cargo run -- {{input}} -o {{output}}

