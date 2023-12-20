# Makefile

# Compile and run the Rip programming language
run:
	cargo run

# Clean the project
clean:
	cargo clean

# Install Rip compiler
install:
	cp ripc.sh /usr/bin/ripc
	chmod +x /usr/bin/ripc
