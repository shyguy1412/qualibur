resty::api_module!(api);

// mod api {
//     use resty::{Endpoint, Route};

//     mod hello_world;
//     const HELLO_WORLD_ENDPOINT: Endpoint = Endpoint {
//         get: Some(&hello_world::get_hello_world),
//         put: None,
//         post: None,
//         delete: None,
//         option: None,
//         trace: None,
//     };

//     const HELLO_WORLD_SEGMENT: Route =
//         Route::Segment("hello_world", &Route::Endpoint(HELLO_WORLD_ENDPOINT));

//     const Router: &[Route] = &[HELLO_WORLD_SEGMENT];
// }

use std::{
    net::{Ipv4Addr, SocketAddrV4},
    process::ExitCode,
};

fn main() -> ExitCode {
    const ADDR: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 3333);

    resty::bind(ADDR);

    println!("Listening on port 3333");

    std::thread::available_parallelism()
        .map(|n| 0..n.into())
        .unwrap_or(0..1)
        .map(|_| resty::spawn_thread())
        .map(|thread| thread.join())
        .for_each(drop);

    return ExitCode::SUCCESS;
}
