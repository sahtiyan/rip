install:
	sudo mkdir -p /usr/bin/rip
	sudo cp -r * /usr/bin/rip
	sudo chmod +x /usr/bin/rip/ripc.sh

uninstall:
	sudo rm -rf /usr/bin/rip

run:
	cargo run

clean:
	cargo clean
