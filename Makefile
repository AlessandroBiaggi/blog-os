target := x86_64-blog_os.json
system := qemu-system-x86_64

build: $(target)
	cargo build

bootimage: $(target)
	cargo bootimage

boot: bootimage
	$(system) -drive format=raw,file=target/x86_64-blog_os/debug/bootimage-blog_os.bin

run:
	cargo run

test:
	cargo test
