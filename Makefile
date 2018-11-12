build_osx_release:
	mkdir -p artifacts
	cargo build --release
	mv target/release/detect artifacts/detect-osx
	strip artifacts/detect-osx

build_linux_release:
	mkdir -p artifacts
	docker run --rm -it -v "$(PWD)":/home/rust/src omnijar/rust:linux-musl /bin/bash -c "cargo build --release && strip target/x86_64-unknown-linux-musl/release/detect"
	mv target/x86_64-unknown-linux-musl/release/detect artifacts/detect-linux

build_release: build_osx_release build_linux_release
