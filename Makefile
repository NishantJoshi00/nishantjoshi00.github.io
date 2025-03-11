

./dist/terminal/terminal_handler.wasm: ./terminal-handler/
	cd ./terminal-handler && cargo build --release --target wasm32-unknown-unknown
	cp ./terminal-handler/target/wasm32-unknown-unknown/release/terminal_handler.wasm ./dist/terminal/terminal_handler.wasm

build: ./dist/terminal/terminal_handler.wasm
