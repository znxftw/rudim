.PHONY: all clean

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
