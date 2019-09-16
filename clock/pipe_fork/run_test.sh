cargo build --release
i="0"
while [ $i -lt $1 ]; 
do
	taskset -c 1 ./target/release/pipe_fork
	i=$(($i + 1))
done
