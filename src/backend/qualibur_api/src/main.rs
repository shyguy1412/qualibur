use resty::{Request, Response, Router};

use std::{
    fmt::Display,
    net::{Ipv4Addr, SocketAddrV4},
    process::ExitCode,
    sync::LazyLock,
    thread,
};

// #[resty::use_path_routing("./nested/api")]
#[resty::use_manual_routing]
static ROUTER: LazyLock<Router>;

#[resty::endpoint(
    Router(ROUTER),
    Path("/"),
    Method(GET),
    Header("Content-Type", "text/html; charset=utf-8")
)]
async fn get_hello_world<'a>(_req: &mut Request<'a>, res: &mut Response<'a, MyResponse>) {
    let count = 1;
    let _ = res
        .send(&MyResponse(format!(
            "Hello World! (request number: {count})"
        )))
        .await
        .inspect_err(log_error);
}

#[resty::endpoint(
    Router(ROUTER),
    Path("/"),
    Method(POST),
    Header("Content-Type", "text/html; charset=utf-8"),
    // Header("Content-Length", "0")
)]
async fn get_hello_world<'a>(
    req: &mut Request<'a, MyResponse>,
    res: &mut Response<'a, MyResponse>,
) {
    // let count = 1;
    let _ = res
        .send(req.body().await.as_ref().expect("Parse error"))
        .await
        .inspect_err(log_error);
}

pub fn log_error(err: &impl Display) {
    println!("{err}");
}

#[derive(serde::Serialize, serde::Deserialize, resty::Serialize, resty::Deserialize, Debug)]
#[serializer(crate::serialize)]
#[deserializer(crate::deserialize)]
struct MyResponse(String);

// const REQ: &[u8] = b"GET / HTTP/1.1\r\naccept: */*\r\naccept-encoding: gzip, compress, deflate, br\r\nuser-agent: oha/1.14.0\r\nhost: localhost:3333\r\n\r\n";

fn main() -> ExitCode {
    const ADDR: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 3333);

    resty::bind(ADDR, &ROUTER);

    println!("Listening on port 3333");

    let _: Vec<_> = std::thread::available_parallelism()
        .ok()
        .map(|n| 0..n.get())
        // .and_then(|_| None)
        .unwrap_or(0..1)
        .map(|_| resty::spawn_thread())
        .collect();

    // let Ok(mut stream) = TcpStream::connect(ADDR) else {
    //     panic!("Can not connect")
    // };

    // let a = stream.try_clone().expect("must work");

    // thread::spawn(move || {
    //     let bytes = &mut a.bytes();
    //     while let Some(Ok(byte)) = bytes.next() {
    //         // println!("{byte:?}")
    //     }
    // });

    // thread::spawn(move || {
    //     for _ in 0..=5 {
    //         let e = stream.write_all(REQ);
    //         println!("{e:?}");
    //     }
    // });

    thread::park();

    return ExitCode::SUCCESS;
}

fn serialize<T: serde::Serialize>(data: &T) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    Ok(serde_json::to_vec(&data)?)
}

fn deserialize<'a, T: serde::Deserialize<'a>>(
    data: &'a [u8],
) -> Result<T, Box<dyn std::error::Error>> {
    Ok(serde_json::from_slice(data).inspect_err(|e| println!("{e:?}"))?)
}
