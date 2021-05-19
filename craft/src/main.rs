mod server;
mod types;

use std::net::Ipv4Addr;

use crate::server::*;

fn main() {
	let rt = tokio::runtime::Runtime::new().expect("tokio");
	
	println!("yeah");
	
	rt.block_on(async {
		let mut server = McServer::bind((Ipv4Addr::new(127, 0, 0, 1), 25565)).await.expect("server!");
		server.do_it().await;
	});
}

//extra read types: define McRead, impl it forall things that impl asyncread or whatnot
//cribbed from a minecraft varint library: impl<R> VarIntRead for R where R: std::io::Read {
//(but i'll want to tokio it a bit)