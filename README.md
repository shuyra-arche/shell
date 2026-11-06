# Shell

Shell provides Shuyra Vanllia's interactive Lisp environment, startup protocol,
job and pipe adapters, and six command images.

## Layout

| Path | Responsibility |
| --- | --- |
| `api/`, `client/`, `declaration/` | Wire codecs, startup client and catalogue compatibility declarations |
| `lisp/` | System-independent `no_std + alloc` language library |
| `server/` | Independent workspace for the REPL, native host and job/pipe adapters |
| `server/entry/` | `shell-entry` package and `prog-shell` executable |
| `commands/` | Independent workspace for `emit`, `upper`, `fail`, `spin`, `workers` |
| `cat/` | Independent workspace for the `prog-cat` executable |
| `images/` | Aggregate declarations for all seven images |
| `tests/unit-document/` | Independent runtime-document contract tests |

Vanllia discovers `server/entry/unit.toml`. Shell is a requested Unit: packaging
it does not start it. Its runtime dependencies are Pipe and Terminal; its supply
bindings include all six command images. `prog-cat` is bound to the existing
Shell Cat slot. Root workspace defaults include API and declaration only.

## Build and test

Install Rust through rustup; `rust-toolchain.toml` selects `nightly-2026-09-04`
and the RISC-V target. Run from the repository root so `.cargo/config.toml`
applies the image link flags. The first run needs network access.

```sh
cargo test --locked
cargo test --locked -p lisp --target x86_64-unknown-linux-gnu
cargo test --locked --manifest-path server/Cargo.toml -p shell-tests -p shell-adapter-tests --target x86_64-unknown-linux-gnu
cargo test --locked --manifest-path tests/unit-document/Cargo.toml --target x86_64-unknown-linux-gnu
cargo test --locked --manifest-path images/Cargo.toml --test builds --target x86_64-unknown-linux-gnu
cargo test --locked --manifest-path cat/tests/Cargo.toml --target x86_64-unknown-linux-gnu
python3 commands/tests/test_migration.py
cargo build --locked --manifest-path server/entry/Cargo.toml -p shell-entry --bin prog-shell --target riscv64gc-unknown-none-elf --release
cargo build --locked --manifest-path commands/Cargo.toml --bins --target riscv64gc-unknown-none-elf --release
cargo build --locked --manifest-path cat/command/Cargo.toml --bin prog-cat --target riscv64gc-unknown-none-elf --release
```

Host tests and ELF builds do not establish interactive or SMP runtime behavior.
Use Vanllia's selected deployment for product validation. Historical import
audits such as `images/audit.py` compare fixed migration checkpoints; run them
on their corresponding historical checkout, not as current-tree CI checks.

## Interface constraints

Wire version 3 and existing encode/decode asymmetries are covered by
`api/tests/wire.rs`. `Boot::take` consumes the boot owner and grant, imports
ports and maps failures to `()`; it does not promise transactional rollback.
Pipe copy retries partial writes and closes the write port before the read port
at EOF. Preserve these ownership and failure contracts when changing clients.

Attribution and source-history mappings remain in the component provenance files.
Follow the [Shuyra maintenance standard](https://github.com/shuyra-arche/vanllia/blob/main/CONTRIBUTING.md).
