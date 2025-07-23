
all: rust cpp

rust:
	@echo
	@echo Rust:
	cargo run < input.txt

cpp:
	@echo
	@echo c++:
	g++ c++/main.cpp -o target/main
	target/main < input.txt
