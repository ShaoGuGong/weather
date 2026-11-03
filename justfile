list:
    @just -l

build:
    cargo build --release

run: build
    weather -c 新店+新和國小 -f 2

