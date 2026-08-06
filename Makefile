.PHONY: all clean coverage coverage-open coverage-release coverage-release-open mutants quality install-deps setup-hooks

# Detect operating system to handle executable suffixes correctly (.exe on Windows)
ifeq ($(OS),Windows_NT)
    EXE_SUFFIX := .exe
else
    EXE_SUFFIX :=
endif

# OpenBench passes the output path via the EXE variable (e.g., EXE=rudim-master)
# Default to "rudim" if not specified
EXE ?= rudim$(EXE_SUFFIX)

# This is only temporary to fix OpenBench
all:
	@if [ ! -f "deps/bullet/crates/bullet_lib/Cargo.toml" ]; then \
		echo "Submodule not found. Creating placeholder dummy package..."; \
		mkdir -p deps/bullet/crates/bullet_lib/src; \
		printf '[package]\nname = "bullet_lib"\nversion = "0.1.0"\nedition = "2021"\n\n[features]\ncuda = []\nrocm = []\n' > deps/bullet/crates/bullet_lib/Cargo.toml; \
		echo '// Dummy shim' > deps/bullet/crates/bullet_lib/src/lib.rs; \
	fi
	cargo build --release
	cp target/release/rudim$(EXE_SUFFIX) $(EXE)

clean:
	cargo clean

install-deps: setup-hooks
	rustup component add clippy rustfmt llvm-tools-preview
	cargo install cargo-llvm-cov
	cargo install cargo-mutants

setup-hooks:
	git config --local core.hooksPath .githooks
	chmod +x .githooks/pre-push


# Non OpenBench
coverage:
	cargo llvm-cov --lib --html

coverage-open:
	cargo llvm-cov --lib --open

coverage-release:
	cargo llvm-cov --lib --release --html

coverage-release-open:
	cargo llvm-cov --lib --release --open

mutants:
	cargo mutants -- --lib

quality:
	cargo fmt --check
	cargo clippy --all-targets -- -D warnings
	cargo test --lib
	cargo test --tests --release
	# TODO: improve coverage, cases
	cargo llvm-cov --lib --html --fail-under-lines 70
