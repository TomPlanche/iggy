# SDK lifecycle repros

Three small programs show `disconnect()` and `shutdown()` problems in the Rust SDK. The crate uses `apache/iggy` master at commit `5d8129e95`, without local changes.

Start a server with the default root credentials on the default ports. Run this command from the root of an `apache/iggy` checkout, on Linux, because the server uses `io_uring`:

```bash
cargo run --bin iggy-server -- --with-default-root-credentials --fresh
```

The programs connect to `127.0.0.1`. If the server runs in a container, publish ports 8090 and 8092 to the host.

Then run each program from this directory:

| Program | Expected | Actual on `5d8129e95` |
| --- | --- | --- |
| `cargo run --bin ws_connect_after_shutdown` | `connect()` after `shutdown()` returns `ClientShutdown` for TCP and WebSocket | TCP returns `ClientShutdown`, WebSocket returns `Ok(())` |
| `cargo run --bin heartbeat_undoes_disconnect` | `get_me()` fails at both times | `get_me()` fails right after `disconnect()`, then succeeds 7 s later, because the heartbeat reconnects and signs in again |
| `cargo run --bin ping_reconnects_after_disconnect` | Open question: must a direct `ping()` after an explicit `disconnect()` reconnect an auto-login client? | `get_me()` fails, `ping()` reconnects and signs in, then `get_me()` succeeds |

`ping_reconnects_after_disconnect` waits 500 ms before `disconnect()`. Without this wait, the first heartbeat tick runs at the same time as `disconnect()`, and the output changes from run to run.
