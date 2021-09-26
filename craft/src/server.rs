use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;
use tokio::net::ToSocketAddrs;

pub struct McServer {
	listener: TcpListener,
}

impl McServer {
	pub async fn bind(addr: impl ToSocketAddrs) -> std::io::Result<McServer> {
		let listener = TcpListener::bind(addr).await?;

		Ok(McServer { listener })
	}

	pub async fn do_it(&mut self) {
		loop {
			match self.listener.accept().await {
				Ok((mut socket, addr)) => {
					println!("new connection from: {:?}", addr);

					loop {
						let mut data = vec![0; 1024];

						match socket.read(&mut data).await {
							Ok(0) => {
								println!("end of stream");
								break;
							},
							Ok(n) => {
								debug_data(&data[..n]);
							},
							Err(e) => {
								println!("socket read error, {:?}", e)
							},
						}
					}
				},
				Err(e) => {
					println!("connection accept error, {:?}", e)
				},
			}
		}
	}
}

fn debug_data(data: &[u8]) {
	print!("{}\t", data.len());

	for x in data {
		print!("{:x} ", *x);
	}
	print!("\t");
	for x in data {
		let c = *x as char;
		if c.is_ascii_graphic() || c == ' ' {
			print!("{}", c);
		} else {
			print!(".");
		}
	}
	println!();
}
