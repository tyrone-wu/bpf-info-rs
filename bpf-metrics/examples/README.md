## Example Usages

This directory contain example usages of `bpf-metrics` with various monitoring providers.

Since this library requires root privileges, you must first build the binary so that it can be executed with `sudo`.

```bash
# Build binary
cargo build --example $name

# Execute with root privileges
sudo ./target/debug/examples/$name
```
