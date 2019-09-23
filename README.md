# Benchmarking Interprocess Communication in Linux

## Steps to Execute

## Clock - Timestamp Counter
```cd clock_tsc
```cargo run --release

## Clock - Realtime Clock
``` cd clock_realtime
``` cargo run --release

## Pipe - Latency
``` cd pipe_fork
``` ./run_test.sh

## Pipe - Throughput
``` cd pipe_fork_throughput
``` ./run_test.sh

### TCP
1. Start the server
```bash
# Latency
./tcp_server/target/release/tcp_server <ip>:<port>
# Throughput
./tcp_server_t/target/release/tcp_server_t <ip>:<port>
```
<ip> should be `127.0.0.1` for local experiments and IP address of the host for remote experiments.

2. Start the client
```bash
# Latency
./tcp_client/target/release/tcp_client <ip>:<port>
# Throughput
./tcp_client_t/target/release/tcp_client_t <ip>:<port>
```
<ip>:<port> should be the server's IP and port for connection

### UDP
1. Start the server
```bash
# Latency
./udp_server/target/release/udp_server <ip1>:<port1> <ip2>:<port2>
# Throughput
./udp_server_t/target/release/udp_server_t <ip1>:<port1> <ip2>:<port2>
```
`For the UDP throughput experiment client should be started within 5 seconds after starrting the server else the operation will timeout.`

2. Start the client
```bash
# Latency
./udp_client/target/release/udp_client <ip2>:<port2> <ip1>:<port1>
# Throughput
./udp_client_t/target/release/udp_client_t  <ip2>:<port2> <ip1>:<port1>
```
`<ip1>` - Server IP Address (127.0.0.1 for local) 
`<ip2>` - Client IP address (127.0.0.1 for local)


