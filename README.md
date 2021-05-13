# tls-tunnel

### Rust TLS over TCP tunnel

This simple crate establishes a TLS connection to a specified target and creates a TCP server that allows you to tunnel TCP to the target over a standard TLS connection.

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
$ tls-tunnel httpbin.org:443
```

The server will automatically choose a port to listen on. It defaults to port `1025` and will increment by one until it finds an available port to bind to.
