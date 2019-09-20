#!/bin/bash
cargo build --release
sizes=(4 16 64 256 1024 4096 16384 65536 262144 524288)
for size in ${sizes[@]}; do
	echo "Running for size: "$size
	./target/release/pipe_fork $size >> output_$size
	echo "Min Value = "$(cat output_$size | sort -nr | tail -n 1)
done
