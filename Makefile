all: run

run: ./src/main.rs
	@cargo build
	@clear
	@./target/debug/blueprint

build: ./src/main.rs
	@cargo build -r

clean:
	@cargo clean
