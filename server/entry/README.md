# Shell entry

The `shell-entry` package builds `prog-shell`, the freestanding RISC-V entry
for `shell-server`. Execution supplies startup, allocation, panic handling and
the linker script; `build.rs` forwards `DEP_EXECUTION_LINKER_SCRIPT`.

`unit.toml` is the requested runtime policy. The retained Rust image declaration
is `shell_declaration::BUILD`. Run from the repository root so image link flags
and provider-relative paths apply:

```sh
cargo build --locked --manifest-path server/entry/Cargo.toml -p shell-entry --bin prog-shell --target riscv64gc-unknown-none-elf --release
```

The entry must lie inside an executable `PT_LOAD`; the minimum load base is
`0x10000`. Image construction does not prove startup. See the
[root README](../../README.md) for all builds and tests.
