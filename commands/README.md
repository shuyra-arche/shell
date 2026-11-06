# Shell commands

This independent workspace builds `emit`, `upper`, `fail`, `spin` and `workers`
as Shell resource images, not runtime Units. The client is the sibling
`../client` path dependency. `declaration/` owns image coordinates; the Shell
catalogue owns ports and bindings. Execution supplies startup and linking.

| Image | Behavior |
| --- | --- |
| `emit` | Writes arguments or repeated text to `records` |
| `upper` | Copies `source` to `result` with ASCII uppercasing |
| `fail` | Returns a selected exit reason, defaulting to 17 |
| `spin` | Consumes its boot grant and repeatedly calls `starve` |
| `workers` | Spawns a counter worker and writes periodic `ticks` |

From the repository root:

```sh
python3 commands/tests/test_migration.py
cargo build --locked --manifest-path commands/Cargo.toml --bins --target riscv64gc-unknown-none-elf --release
cargo test --locked --manifest-path commands/declaration/Cargo.toml --target x86_64-unknown-linux-gnu
```

See the [root README](../README.md) for prerequisites and runtime validation.
Source attribution remains in [PROVENANCE.md](PROVENANCE.md).
