extern crate futures;
extern crate futures_io;
extern crate futures_mio;
extern crate futures_tls;

use std::net::ToSocketAddrs;

use futures::Future;
use futures_mio::Loop;
use futures_tls::ClientContext;

pub fn fetch<S: Into<String>>(url: S) -> String {
    let url = url.into();
    let request_msg = format!("GET / HTTP/1.0\r\n Host: {}\r\n\r\n", url);
    let mut lp = Loop::new().unwrap();
    let addr = format!("{}:443", url).to_socket_addrs().unwrap().next().unwrap();

    let socket = lp.handle().tcp_connect(&addr);

    let tls_handshake = socket.and_then(move |socket| {
        let cx = ClientContext::new().unwrap();
        cx.handshake(&url, socket)
    }).boxed();

    let request = tls_handshake.and_then(move |socket| {
        futures_io::write_all(socket, request_msg)
    }).boxed();
    let response = request.and_then(|(socket, _)| {
        futures_io::read_to_end(socket, Vec::new())
    }).boxed();

    let data = lp.run(response).unwrap();
    String::from_utf8_lossy(&data).into_owned()
}

#[test]
fn it_should_not_panic() {
  fetch("www.rust-lang.org");
}
