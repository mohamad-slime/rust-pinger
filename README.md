
# rust-pinger — minimal async ping

Super-small async ICMP ping tool written in Rust (Tokio + surge-ping).

Usage

```powershell
# defaults: host = google.com, payload = 32
cargo run -- <host> <payload_size>

# examples
cargo run -- google.com
cargo run -- 1.1.1.1 64
```

Behavior

- Sends 4 pings (1 per second) to the first IP returned by DNS for the given host.
- Prints round-trip duration per packet.
- Rejects very large payloads (safety cap in code).

Notes

- On Windows you usually need to run PowerShell as Administrator so the program can send ICMP packets.
- If no args are provided the program pings `google.com` with a 32-byte payload.

Want it even simpler? I can add a `--help` flag, better CLI parsing, or a tiny examples script.

