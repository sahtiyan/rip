# Makefile

# Compile and run the Rip programming language
run:
	cargo run

# Clean the project
clean:
	cargo clean

# Install Rip compiler
install:
	sudo mkdir -p /usr/bin/rip
	sudo cp -r * /usr/bin/rip
	sudo chmod +x /usr/bin/rip/ripc.sh
