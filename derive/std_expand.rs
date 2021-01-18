enabled schema
#![feature(prelude_import)]
#![doc = " A simple example"]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
use jsonrpc_core::futures::future::{self, Future, FutureResult};
use jsonrpc_core::{Error, IoHandler, Result};
use jsonrpc_core_client::transports::local;
use jsonrpc_derive::rpc;
mod rpc_impl_Rpc {
    use jsonrpc_core as _jsonrpc_core;
    use super::*;
}
struct RpcImpl;
impl Rpc for RpcImpl {
    fn protocol_version(&self) -> Result<String> {
        Ok("version1".into())
    }
    fn add(&self, a: u64, b: u64) -> Result<u64> {
        Ok(a + b)
    }
    fn call(&self, _: u64) -> FutureResult<String, Error> {
        future::ok("OK".to_owned())
    }
    fn notify(&self, a: u64) {
        {
            ::std::io::_print(::core::fmt::Arguments::new_v1(
                &["Received `notify` with value: ", "\n"],
                &match (&a,) {
                    (arg0,) => [::core::fmt::ArgumentV1::new(
                        arg0,
                        ::core::fmt::Display::fmt,
                    )],
                },
            ));
        };
    }
}
fn main() {
    let mut io = IoHandler::new();
    io.extend_with(RpcImpl.to_delegate());
    let fut = {
        let (client, server) = local::connect::<gen_client::Client, _, _>(io);
        let schema = gen_client::Client::gen_schema();
        let j = serde_json::to_string_pretty(&schema).unwrap();
        {
            ::std::io::_print(::core::fmt::Arguments::new_v1(
                &["", "\n"],
                &match (&j,) {
                    (arg0,) => [::core::fmt::ArgumentV1::new(
                        arg0,
                        ::core::fmt::Display::fmt,
                    )],
                },
            ));
        };
        client
            .add(5, 6)
            .map(|res| {
                ::std::io::_print(::core::fmt::Arguments::new_v1(
                    &["5 + 6 = ", "\n"],
                    &match (&res,) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                ));
            })
            .join(server)
    };
    fut.wait().unwrap();
}
