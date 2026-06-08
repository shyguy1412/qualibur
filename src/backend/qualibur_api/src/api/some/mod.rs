use std::collections::HashMap;

use smol::{io::AsyncWriteExt, net::TcpStream};

#[resty::endpoint(GET)]
async fn get_hello_world(_headers: HashMap<String, Box<[u8]>>, mut stream: TcpStream) {
    let _ = stream.write("mod".as_bytes()).await;
    let _ = stream.close().await;
}
