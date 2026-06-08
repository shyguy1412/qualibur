use std::collections::HashMap;

use smol::{io::AsyncWriteExt, net::TcpStream};

mod hello_world;
mod nested;
mod some;

#[resty::endpoint(GET)]
async fn get_main(_headers: HashMap<String, Box<[u8]>>, mut stream: TcpStream) {
    let _ = stream.write("Base".as_bytes()).await;
    let _ = stream.close().await;
}
