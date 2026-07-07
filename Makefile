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

all:
	cargo build --release
	cp target/release/rudim$(EXE_SUFFIX) $(EXE)

clean:
	cargo clean
