.ONESHELL:
BINARY="pihole_restore"

.DEFAULT_GOAL: $(BINARY)

$(BINARY):
	cargo build --release
	chown -R $$(id -u):$$(id -g) target

build-armv8:
	# add linker config
	echo "[target.aarch64-unknown-linux-gnu]" > /usr/local/cargo/config
	echo "linker = \"aarch64-linux-gnu-gcc\"" >> /usr/local/cargo/config
	cat /usr/local/cargo/config
	# add architecture
	dpkg --add-architecture arm64
	apt-get update
	apt-get install -y curl git build-essential
	apt-get install -y libc6-arm64-cross libc6-dev-arm64-cross aarch64-linux-gnu-gcc
	# we use sqlite
	apt-get install -y libsqlite3-0:arm64 libsqlite3-dev:arm64
	rustup default stable
	rustup target add x86_64-unknown-linux-gnu
	rustup target add aarch64-unknown-linux-gnu
	export PKG_CONFIG_PATH="/usr/lib/aarch64-unknown-linux-gnu/pkgconfig"
	export PKG_CONFIG_ALLOW_CROSS="true"
	# build
	cargo build --release --target aarch64-unknown-linux-gnu
	chown -R 1000:1000 target

build-lowest-glibc:
	# buster at this point is on glibc 2.28
	docker run -v $(shell pwd):/usr/src/pihole_restore -w /usr/src/pihole_restore -t rust:buster make
	

build-lowest-glibc-arm:
	# buster at this point is on glibc 2.28
	docker run -v $(shell pwd):/usr/src/pihole_restore -w /usr/src/pihole_restore -t rust:buster make build-armv8

test: test-clean build-lowest-glibc
	mkdir -p test/pihole
	mkdir -p test/dnsmasq
	docker run --name pihole -d -v $(shell pwd)/test/pihole:/etc/pihole -v $(shell pwd)/test/dnsmasq:/etc/dnsmasq.d pihole/pihole:latest
	sleep 20
	docker cp ./target/release/pihole_restore pihole:./
	docker cp ./test/pi-hole_backup.tar.gz pihole:./
	docker exec -e RUST_LOG=debug -t pihole ./pihole_restore -f pi-hole_backup.tar.gz
	docker inspect pihole | grep IPAddress
	docker logs pihole 2>/dev/null | grep "Assigning random password"

test-clean:
	-docker stop pihole
	-docker rm pihole
	mkdir -p ./test/archive
	-sudo mv ./test/pihole ./test/archive/pihole-$(shell date +%Y-%m-%d_%H%M)
	-sudo mv ./test/dnsmasq ./test/archive/dnsmasq-$(shell date +%Y-%m-%d_%H%M)

clean:
	rm -rf target
