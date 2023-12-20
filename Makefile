# Makefile

# Compile and run the Rip programming language
run:
	cargo run

# Clean the project
clean:
	cargo clean

# Install Rip compiler
install:
	sudo cp ripc.sh /usr/bin/ripc
	sudo chmod +x /usr/bin/ripc
