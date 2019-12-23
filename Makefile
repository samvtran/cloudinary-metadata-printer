all: build upload
build:
	wasm-pack build --target no-modules --out-name print_meta
upload:
	npm start