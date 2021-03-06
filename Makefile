.PHONY: codecov
codecov: 
	export CARGO_INCREMENTAL=0
	export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off"
	cargo +nightly test
	grcov ./target/debug/ -s . -t html --llvm --branch --ignore-not-existing -o ./target/debug/coverage/

opencov: target/debug/coverage/index.html
	open target/debug/coverage/index.html
