# Define a recursive wildcard function
rwildcard=$(foreach d,$(wildcard $(1:=/*)),$(call rwildcard,$d,$2) $(filter $(subst *,%,$2),$d))

# Define source directories
TERMINAL_HANDLER_DIR := ./terminal-handler
FS_DIR := ./dist/fs

# Get all files recursively from both directories
TERMINAL_HANDLER_FILES := $(call rwildcard,$(TERMINAL_HANDLER_DIR),*)
FS_FILES := $(call rwildcard,$(FS_DIR),*)

# Target rule
./dist/terminal/terminal_handler.wasm: $(TERMINAL_HANDLER_FILES) $(FS_FILES)
	cd ./terminal-handler && cargo build --release --target wasm32-unknown-unknown
	cp ./terminal-handler/target/wasm32-unknown-unknown/release/terminal_handler.wasm ./dist/terminal/terminal_handler.wasm

.PHONY: clean build

clean:
	rm ./dist/terminal/terminal_handler.wasm

build: ./dist/terminal/terminal_handler.wasm
