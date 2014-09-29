#![feature(macro_rules)]
#![feature(phase)]

#[phase(plugin, link)] extern crate log;

use server::Server;
mod server;

#[deriving(Clone)]
struct FierroServer;

impl server::Server for FierroServer {
}

fn main() {
    FierroServer.serve("127.0.0.1", 8090);
}
