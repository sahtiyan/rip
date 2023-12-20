install:
	sudo mkdir -p /usr/bin/rip
	sudo cp -r * /usr/bin/rip
	sudo chmod +x /usr/bin/rip/ripc.sh
	sudo echo 'export PATH=$$PATH:/usr/bin/rip' >> ~/.bashrc

uninstall:
	sudo rm -rf /usr/bin/rip
	sudo sed -i '/\/usr\/bin\/rip/d' ~/.bashrc

run:
	cargo run

clean:
	cargo clean
