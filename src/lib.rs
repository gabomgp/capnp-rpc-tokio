extern crate capnp;
extern crate capnp-futures;

use capnp::Error;
use capnp::private::capability::{ClientHook, ServerHook};
use std::cell::RefCell;
use std::rc::Rc;

/// Code generated from [rpc.capnp]
/// (https://github.com/sandstorm-io/capnproto/blob/master/c%2B%2B/src/capnp/rpc.capnp).
pub mod rpc_capnp {
  include!(concat!(env!("OUT_DIR"), "/rpc_capnp.rs"));
}

/// Code generated from [rpc-twoparty.capnp]
/// (https://github.com/sandstorm-io/capnproto/blob/master/c%2B%2B/src/capnp/rpc-twoparty.capnp).
pub mod rpc_twoparty_capnp {
  include!(concat!(env!("OUT_DIR"), "/rpc_twoparty_capnp.rs"));
}