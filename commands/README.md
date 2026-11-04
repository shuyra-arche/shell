# commands

Five Shell command resource images extracted from frozen Source revision
`94e114614bec53424ac55e640fc80cf19a3e7107`:

`emit`, `upper`, `fail`, `spin`, `workers`.

They are resource images in Shell's existing catalogue, not separate
`UnitFile`s. They live in the independent nested `commands/` Cargo workspace
inside the existing public Shell repository. The Shell root workspace is
intentionally unchanged and does not include `commands/`. Shell's API, client
and catalogue are not copied here.

## Runtime behavior preserved

| Binary | Shell resource/ports | Behavior |
| --- | --- | --- |
| `emit` | `emit`; writes `records` | Writes each boot argument followed by `\n`. With `--repeat <count> <text>`, writes `text` exactly `count` times, accepting `count <= 1048576`. Boot, port, write or close failure returns reason `1`; invalid repeat arguments return reason `2`. |
| `upper` | `upper`; reads `source`, writes `result` | Copies `source` to `result` with ASCII uppercasing until EOF, then closes both ports. Any boot, port or copy failure returns reason `1`. |
| `fail` | `fail`; no ports | Returns the first argument parsed as an exit reason, or `17` when absent/unparseable. Boot failure returns reason `1`. |
| `spin` | `spin`; no ports | Takes the boot grant once, then calls `env::room::starve()` forever. Boot failure returns reason `1`. |
| `workers` | `workers`; writes `ticks` | Spawns one worker task that increments an atomic counter, then repeatedly writes the counter and `\n`, parking for 20 milliseconds with `env::room::park(20)` between writes. Any boot, port, spawn, write or park failure returns reason `1`. |

The port and resource names above are unchanged from the source. Their Shell
catalogue bindings remain owned by Shell.

## Dependency and layout

`commands/Cargo.toml` is the final nested-workspace runtime manifest. It pins
`shell-client` to the fixed prior public Shell commit
`b910582fc9dc0fd732f279d94f2cfca89f4c8c8f` from
`https://github.com/shuyra-arche/shell.git`. This is deliberately a Git
dependency, not a local path, even though the surrounding Shell checkout
contains a nearby `client/` tree. The sibling tree must not be substituted for
the pinned provider.

The remaining direct pins are also unchanged:

- `pipe-client` at `4fce76372beb6c5ab8567232516f156b3d1032eb`
- `env` at `d3000253cd5d13d466e91fafd4dbb27a5ec5aaae`
- `execution` at `1b333a28bc4c34beaea3f7986a9bb6e8ab2db414`
- `mold` at `f52501f4c91b686eded8b4bcd743b4e96084b7f9`

- `Cargo.toml` - final `shell-command-images` package with five explicit bins.
- `build.rs` - forwards `DEP_EXECUTION_LINKER_SCRIPT` and tracks each bin source;
  no private linker/runtime.
- `.cargo/config.toml` - RISC-V target flags for static relocation, medium code
  model and frame pointers; no QEMU runner.
- `rust-toolchain.toml` - Source's pinned `nightly-2026-09-04`.
- `declaration/` - pure standalone `no_std` descriptor workspace using public
  `unit::ImageBuild`; no Shell dependency, UnitFile, ports or client.
- `provenance/source-hashes.json` - portable source paths, hashes and history.
- `tests/test_migration.py` - standard-library preservation and manifest contract.
- `VALIDATION.md` - preparation history plus the remaining final evidence.

## Build location and config discovery

Cargo config is discovered from the current working directory, not from the
directory containing `--manifest-path`. Therefore `commands/.cargo/config.toml`
and its RISC-V flags are active only when the invocation runs from inside
`commands/` (or a descendant).

From the repository root, the runtime manifest is selected explicitly:

```sh
cargo +nightly-2026-09-04 check \
  --manifest-path commands/Cargo.toml \
  --target riscv64gc-unknown-none-elf --bins
```

That root invocation does not load `commands/.cargo/config.toml`. For genuine
native builds, enter the nested workspace first so the target flags are active:

```sh
cd commands
cargo +nightly-2026-09-04 build \
  --target riscv64gc-unknown-none-elf --bins
cargo +nightly-2026-09-04 build --release \
  --target riscv64gc-unknown-none-elf --bins
```

These commands are for Root to run; the worker did not run Cargo, native builds
or QEMU.

## Library-only preparation checks

`commands/declaration/` is a separate workspace. Its tests exercise only public
`unit::ImageBuild` coordinates and do not launch a command or substitute a
Shell client. From `commands/`:

```sh
python3 tests/test_migration.py
cargo +nightly-2026-09-04 test --manifest-path declaration/Cargo.toml \
  --target x86_64-unknown-linux-gnu --locked --offline
cargo +nightly-2026-09-04 check --manifest-path declaration/Cargo.toml \
  --target riscv64gc-unknown-none-elf --lib --locked --offline
cargo +nightly-2026-09-04 check --manifest-path declaration/Cargo.toml \
  --target riscv64gc-unknown-none-elf --lib --release --locked --offline
```

These are descriptor/library checks, not command-binary validation. Root owns
the actual test and build results.

The descriptors use the provider-relative manifest `commands/Cargo.toml`. In a
later integration tree that selects a different Shell checkout prefix, the
effective descriptor manifest must be
`<selected-shell-checkout>/commands/Cargo.toml`; image names, bin names,
package `shell-command-images`, features, target and profile remain unchanged.

## Native validation and runtime boundary

Root verified the pinned Shell provider and completed real debug/release builds
of all five RISC-V binaries, actual ELF/linker audits, source preservation and
host contract checks. Both runtime and descriptor locks retain Source's
external dependency versions. See `VALIDATION.md` for actual results and the
historical preparation checkpoint. Catalogue integration, QEMU and actual
command behavior were not executed.
