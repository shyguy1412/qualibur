#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
mod api {
    use std::collections::HashMap;
    use smol::{io::AsyncWriteExt, net::TcpStream};
    mod hello_world {
        use std::collections::HashMap;
        use smol::{io::AsyncWriteExt, net::TcpStream};
        #[used]
        #[unsafe(link_section = "linkme_ROUTES")]
        static get_hello_world_route: (
            &'static [&'static str],
            ::resty::Handler,
            ::resty::HttpMethod,
        ) = {
            #[allow(clippy::no_effect_underscore_binding)]
            unsafe fn __typecheck(_: ::resty::linkme::__private36::Void) {
                #[allow(clippy::ref_option_ref)]
                let __new = || -> fn() -> &'static (
                    &'static [&'static str],
                    ::resty::Handler,
                    ::resty::HttpMethod,
                ) { || &get_hello_world_route };
                unsafe {
                    ::resty::linkme::DistributedSlice::private_typecheck(
                        ::resty::ROUTES,
                        __new(),
                    );
                }
            }
            (&["hello_world"], &get_hello_world, ::resty::HttpMethod::GET)
        };
        pub fn get_hello_world(
            _headers: HashMap<String, Box<[u8]>>,
            stream: TcpStream,
        ) -> ::smol::Task<()> {
            async fn get_hello_world(
                _headers: HashMap<String, Box<[u8]>>,
                mut stream: TcpStream,
            ) {
                let _ = stream.write("Hello World".as_bytes()).await;
                let _ = stream.close().await;
            }
            ::resty::task(get_hello_world(_headers, stream))
        }
    }
    mod nested {
        mod path {
            use std::collections::HashMap;
            use smol::{io::AsyncWriteExt, net::TcpStream};
            #[used]
            #[unsafe(link_section = "linkme_ROUTES")]
            static get_hello_world_route: (
                &'static [&'static str],
                ::resty::Handler,
                ::resty::HttpMethod,
            ) = {
                #[allow(clippy::no_effect_underscore_binding)]
                unsafe fn __typecheck(_: ::resty::linkme::__private36::Void) {
                    #[allow(clippy::ref_option_ref)]
                    let __new = || -> fn() -> &'static (
                        &'static [&'static str],
                        ::resty::Handler,
                        ::resty::HttpMethod,
                    ) { || &get_hello_world_route };
                    unsafe {
                        ::resty::linkme::DistributedSlice::private_typecheck(
                            ::resty::ROUTES,
                            __new(),
                        );
                    }
                }
                (&["nested", "path"], &get_hello_world, ::resty::HttpMethod::GET)
            };
            pub fn get_hello_world(
                _headers: HashMap<String, Box<[u8]>>,
                stream: &mut TcpStream,
            ) -> ::smol::Task<()> {
                async fn get_hello_world(
                    _headers: HashMap<String, Box<[u8]>>,
                    stream: &mut TcpStream,
                ) {
                    let _ = stream.write("nested".as_bytes()).await;
                    let _ = stream.close().await;
                }
                ::resty::task(get_hello_world(_headers, stream))
            }
        }
    }
    mod some {
        use std::collections::HashMap;
        use smol::{io::AsyncWriteExt, net::TcpStream};
        #[used]
        #[unsafe(link_section = "linkme_ROUTES")]
        static get_hello_world_route: (
            &'static [&'static str],
            ::resty::Handler,
            ::resty::HttpMethod,
        ) = {
            #[allow(clippy::no_effect_underscore_binding)]
            unsafe fn __typecheck(_: ::resty::linkme::__private36::Void) {
                #[allow(clippy::ref_option_ref)]
                let __new = || -> fn() -> &'static (
                    &'static [&'static str],
                    ::resty::Handler,
                    ::resty::HttpMethod,
                ) { || &get_hello_world_route };
                unsafe {
                    ::resty::linkme::DistributedSlice::private_typecheck(
                        ::resty::ROUTES,
                        __new(),
                    );
                }
            }
            (&["some", ""], &get_hello_world, ::resty::HttpMethod::GET)
        };
        pub fn get_hello_world(
            _headers: HashMap<String, Box<[u8]>>,
            stream: TcpStream,
        ) -> ::smol::Task<()> {
            async fn get_hello_world(
                _headers: HashMap<String, Box<[u8]>>,
                mut stream: TcpStream,
            ) {
                let _ = stream.write("mod".as_bytes()).await;
                let _ = stream.close().await;
            }
            ::resty::task(get_hello_world(_headers, stream))
        }
    }
    #[used]
    #[unsafe(link_section = "linkme_ROUTES")]
    static get_main_route: (
        &'static [&'static str],
        ::resty::Handler,
        ::resty::HttpMethod,
    ) = {
        #[allow(clippy::no_effect_underscore_binding)]
        unsafe fn __typecheck(_: ::resty::linkme::__private36::Void) {
            #[allow(clippy::ref_option_ref)]
            let __new = || -> fn() -> &'static (
                &'static [&'static str],
                ::resty::Handler,
                ::resty::HttpMethod,
            ) { || &get_main_route };
            unsafe {
                ::resty::linkme::DistributedSlice::private_typecheck(
                    ::resty::ROUTES,
                    __new(),
                );
            }
        }
        (&[""], &get_main, ::resty::HttpMethod::GET)
    };
    pub fn get_main(
        _headers: HashMap<String, Box<[u8]>>,
        stream: TcpStream,
    ) -> ::smol::Task<()> {
        async fn get_main(_headers: HashMap<String, Box<[u8]>>, mut stream: TcpStream) {
            let _ = stream.write("Base".as_bytes()).await;
            let _ = stream.close().await;
        }
        ::resty::task(get_main(_headers, stream))
    }
}
use std::{
    net::{Ipv4Addr, SocketAddrV4},
    process::ExitCode,
};
fn main() -> ExitCode {
    const ADDR: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 3333);
    {
        ::std::io::_print(format_args!("START"));
    };
    resty::bind(ADDR);
    {
        ::std::io::_print(format_args!("LISTENING ON PORT 3333\n"));
    };
    std::thread::available_parallelism()
        .map(|n| 0..n.into())
        .unwrap_or(0..1)
        .map(|_| resty::spawn_thread())
        .map(|thread| thread.join())
        .for_each(drop);
    return ExitCode::SUCCESS;
}
