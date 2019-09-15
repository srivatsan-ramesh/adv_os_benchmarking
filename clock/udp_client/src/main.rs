use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    {
        let mut socket = UdpSocket::bind("127.0.0.1:8081")?;

        // Receives a single datagram message on the socket. If `buf` is too small to hold
        // the message, it will be cut off.
        let mut buf = [0; 4];
        for i in 0..1000 {
            let (amt, src) = socket.recv_from(&mut buf)?;
            socket.send_to(& mut buf, "127.0.0.1:8082")?;
        }
    } // the socket is closed here
    Ok(())
}