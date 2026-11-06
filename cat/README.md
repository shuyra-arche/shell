# Cat

Cat is a Shell resource image. `command/` builds package `cat`, binary
`prog-cat`; `declaration/` exports image coordinates; `tests/` is an independent
host workspace. Runtime dependencies use sibling Shell API/client paths.

When the Shell owner and mark select a startup grant, Cat copies `source` to
`copy`; failure returns reason 1 without Terminal fallback. Otherwise it
resolves identity and uses Terminal. EOF, interrupt and suspend drain pending I/O.

From the repository root:

```sh
cargo test --locked --manifest-path cat/tests/Cargo.toml --target x86_64-unknown-linux-gnu
cargo build --locked --manifest-path cat/command/Cargo.toml --bin prog-cat --target riscv64gc-unknown-none-elf --release
```

The runtime document binds `prog-cat` to the existing Shell Cat slot; legacy
image declarations retain image name `cat`. Preserve that mapping when choosing
a build path. See the [root README](../README.md) and [attribution](PROVENANCE.md).
