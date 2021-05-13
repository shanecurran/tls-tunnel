# tls-tunnel

### Rust TLS over TCP tunnel

[![Crates.io][crates-badge]][crates-url]
[![docs.rs][docs-badge]][docs-url]
[![MIT licensed][mit-badge]][mit-url]

[crates-badge]: https://img.shields.io/crates/v/tls-tunnel.svg
[crates-url]: https://crates.io/crates/tls-tunnel
[docs-badge]: https://docs.rs/tls-tunnel/badge.svg
[docs-url]: https://docs.rs/tls-tunnel
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/shanecurran/tls-tunnel/blob/master/LICENSE

This simple crate establishes a TLS connection to a specified target and creates a TCP server that allows you to communicate with the TLS server over a raw, unencrypted TCP socket.

This is useful for environments where TLS connections are not feasible at runtime. Some examples:

- Using a CONNECT proxy from a language that doesn't support CONNECT-over-TLS.
- Legacy runtimes which do not support HTTPS but do support HTTP.
- Interoperability of legacy VoIP, IRC or FTP clients which do not support TLS with servers which do.

#### Install

Simply download and compile the Rust binary using `cargo install`:

```bash
$ cargo install tls-tunnel
```

#### Run

Pass the target host and port as an argument when running using `cargo` or the compiled binary. If not provided, the tunnel will default to `httpbin.org:443` for demo purposes.

```bash
$ tls-tunnel https://httpbin.org
```

The server will automatically choose a port to listen on. It defaults to port `1025` and will increment by one until it finds an available port to bind to.
