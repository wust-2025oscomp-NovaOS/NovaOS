rm -rf ex4.img
#dd if=/dev/zero of=ex4.img bs=1M count=8192
dd if=/dev/zero of=ex4.img bs=1M count=512
#mkfs.ext4 ./ex4.img
mkfs.ext4 -b 4096 -m 1 ./ex4.img
