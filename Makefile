.PHONY: all clean

# Detect operating system to handle executable suffixes correctly (.exe on Windows)
ifeq ($(OS),Windows_NT)
    EXE_SUFFIX := .exe
else
    EXE_SUFFIX :=
endif

# OpenBench passes the output path via the EXE variable (e.g., EXE=rudim-master)
EXE ?= rudim$(EXE_SUFFIX)

all:
	# Clone the bullet submodule dynamically if it is empty/missing
	@if [ ! -f "deps/bullet/Cargo.toml" ]; then \
		echo "Cloning and checking out deps/bullet submodule..."; \
		rm -rf deps/bullet; \
		git clone https://github.com/jw1912/bullet deps/bullet; \
		cd deps/bullet && git checkout d372d487aedfeb8bdc256b9f694dbcd41016bf82; \
	fi
	cargo build --release
	cp target/release/rudim$(EXE_SUFFIX) $(EXE)

clean:
	cargo clean
