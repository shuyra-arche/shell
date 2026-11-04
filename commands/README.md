# shell-command-images

Five Shell command resource images extracted from Source revision
`94e114614bec53424ac55e640fc80cf19a3e7107`:

`emit`, `upper`, `fail`, `spin`, `workers`.

They are catalogue resources inside Shell, not separate `UnitFile`s. This repo
contains only their program bodies, resource-name-preserving build coordinates
and extraction checks. Shell's API/client/catalogue is deliberately not copied.

Actual Root checks, the integration manifest-prefix contract and remaining
gates are recorded in [VALIDATION.md](VALIDATION.md).

## Runtime behavior preserved

| Binary | Shell resource/ports | Behavior |
| --- | --- | --- |
| `emit` | `emit`; writes `records` | Writes each boot argument followed by `\n`. With `--repeat <count> <text>`, writes `text` exactly `count` times, accepting `count <= 1048576`. Boot, port, write or close failure returns reason `1`; invalid repeat arguments return reason `2`. |
| `upper` | `upper`; reads `source`, writes `result` | Copies `source` to `result` with ASCII uppercasing until EOF, then closes both ports. Any boot, port or copy failure returns reason `1`. |
| `fail` | `fail`; no ports | Returns the first argument parsed as an exit reason, or `17` when absent/unparseable. Boot failure returns reason `1`. |
| `spin` | `spin`; no ports | Takes the boot grant once, then calls `env::room::starve()` forever. Boot failure returns reason `1`. |
| `workers` | `workers`; writes `ticks` | Spawns one worker task that increments an atomic counter, then repeatedly writes the counter and `\n`, parking for 20 milliseconds with `env::room::park(20)` between writes. Any boot, port, spawn, write or park failure returns reason `1`. |

The port and resource names above are unchanged from the source. Their Shell
catalogue bindings remain owned by the independently extracted Shell provider.

## Layout

- `Cargo.toml.in` - fail-closed runtime manifest template with one
  `@SHELL_PROVIDER_REV@` marker and no root `Cargo.toml` while Shell is pending.
- `build.rs` - forwards `DEP_EXECUTION_LINKER_SCRIPT`; no private linker/runtime.
- `.cargo/config.toml` - RISC-V target flags for static relocation, medium code
  model and frame pointers; no forced host/test target and no QEMU runner.
- `declaration/` - standalone `no_std` workspace with the five
  `unit::ImageBuild` constants and ordered `BUILDS` slice. It has no Shell
  dependency and can be host-tested before Shell exists.
- `provenance/source-hashes.json` - portable original paths, hashes and history.
- `tests/test_migration.py` - standard-library preservation and manifest contract.

## Current blocker

No immutable revision for `https://github.com/shuyra-arche/shell.git` is
available. The root runtime manifest therefore remains `Cargo.toml.in` with one
visible `@SHELL_PROVIDER_REV@` marker. There is intentionally no root
`Cargo.toml`, no resolver lock, and no runtime build until a reviewed immutable
Shell revision exists. Do not replace this with a local Source path, a floating
branch, a fake full SHA, or a reimplemented/mock Boot client.

## Finalization, build and validation commands

This worker did not run Cargo, Git, QEMU or Python. The commands below are the
handoff sequence, not a claim that they have passed.

### Checks available while Shell is pending

`declaration/` is a separate workspace and does not need the Shell provider:

```sh
python3 tests/test_migration.py
cargo +nightly-2026-09-04 test --manifest-path declaration/Cargo.toml \
  --target x86_64-unknown-linux-gnu --locked --offline
cargo +nightly-2026-09-04 check --manifest-path declaration/Cargo.toml \
  --target riscv64gc-unknown-none-elf --lib --locked --offline
cargo +nightly-2026-09-04 check --manifest-path declaration/Cargo.toml \
  --target riscv64gc-unknown-none-elf --lib --release --locked --offline
```

`rust-toolchain.toml` pins Source's exact `nightly-2026-09-04` with the same
components and RISC-V target. Root has installed that toolchain; native proof
uses the pinned toolchain. The migration check reads `Cargo.toml.in` with
`tomllib` and does not build or import Shell. The worker did not run these
commands.

### After a reviewed immutable Shell revision exists

Replace the one marker, materialize the manifest, then generate the lock and
build the RISC-V binaries:

```sh
sed -i 's/@SHELL_PROVIDER_REV@/<full-40-hex-shell-provider-revision>/' Cargo.toml.in
mv Cargo.toml.in Cargo.toml
cargo +nightly-2026-09-04 generate-lockfile --manifest-path Cargo.toml
cargo +nightly-2026-09-04 check --manifest-path Cargo.toml --release \
  --target riscv64gc-unknown-none-elf --bins
cargo +nightly-2026-09-04 build --manifest-path Cargo.toml --release \
  --target riscv64gc-unknown-none-elf --bins
python3 tests/test_migration.py
```

After materialization the migration check permits a real `Cargo.toml` only when
the template is gone, the marker is absent and `shell-client.rev` is a full
40-hex revision. That proves manifest shape, not provider compatibility.
Original-byte reconstruction likewise cannot prove behavior against a future
Shell provider.

Runtime/QEMU behavior is out of scope for this checkpoint and was not run. Real
end-to-end validation requires the reviewed Shell provider, its catalogue and a
booted kernel; no mock client or invented Boot path is used as a substitute.
