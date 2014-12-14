use std::io::{Listener, Acceptor};
use std::io::net::tcp::TcpListener;
use std::io::BufferedStream;


pub trait Server: Send + Clone {
    fn handle_client<R: Reader + Writer>(&self, mut stream: BufferedStream<R>, peer: String) {
        loop {
            let input: String = match stream.read_line() {
                Err(err) => {
                    error!("read error: {}", err);
                    return;
                },
                Ok(str) => str,
            };
            println!("Peer {} sent {}", peer, input);
        };
    }

    fn serve(self, host: &str, port: u16) {
        let mut acceptor = match TcpListener::bind((host, port)).listen() {
            Err(err) => {
                error!("bind to {}:{} failed ({})", host, port, err);
                return;
            },
            Ok(acceptor) => acceptor,
        };
        debug!("bound to {}:{}", host, port);
        loop {
            let mut stream = match acceptor.accept() {
                Err(err) => {
                    debug!("accept failed: {}", err);
                    continue;
                }
                Ok(sock) => sock,
            };
            let peer = format!("{}", stream.peer_name().ok().unwrap());
            debug!("accepted connection from: {}", peer);
            let bstream = BufferedStream::new(stream);
            let self_clone = self.clone();
            spawn(proc() { self_clone.handle_client(bstream, peer) });
        };
    }


}
