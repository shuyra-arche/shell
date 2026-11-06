# Shell server

This independent workspace contains the `shell-server` library, Lisp REPL,
native host, startup handshake and job/pipe adapters. It consumes sibling
`lisp`, `api` and `declaration` packages by path. `entry/` supplies `prog-shell`
and owns executable linking through Execution metadata.

From the repository root:

```sh
cargo test --locked --manifest-path server/Cargo.toml -p shell-tests -p shell-adapter-tests --target x86_64-unknown-linux-gnu
cargo build --locked --manifest-path server/entry/Cargo.toml -p shell-entry --bin prog-shell --target riscv64gc-unknown-none-elf --release
```

`server/Cargo.lock` is the effective lock. Locks inside member test directories
are original source fixtures ignored by Cargo. See the [root README](../README.md)
for toolchain and runtime boundaries, and [attribution](docs/provenance.md).
