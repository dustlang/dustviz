# Getting Started

Build DustViz object output from the Dust compiler workspace:

```bash
CARGO_HOME=/tmp/cargo-home cargo run -p dust -- obj ../dustviz/src --out-dir /tmp/dustviz_obj
```

Expected successful output includes an object file for `main.ds`.
