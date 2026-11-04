# Entry verification

Base: Shell server PR #4, `7a2f99e268784693975b5e244b28795be50c218c`.
The original source is `anreonyr/sqware` at
`94e114614bec53424ac55e640fc80cf19a3e7107`.

Use `nightly-2026-09-04`, the RISC-V target, Python 3.11+, `nm`, and
`llvm-objcopy`. Run the commands from the repository root. The host verification
tool is a separate workspace; its locked host-tool graph does not enter the
server runtime graph.

```sh
cargo test --locked --target x86_64-unknown-linux-gnu -p lisp -p shell-api -p shell-declaration
cargo test --locked --manifest-path server/Cargo.toml --target x86_64-unknown-linux-gnu -p shell-tests -p shell-adapter-tests
cargo build --locked --manifest-path server/entry/Cargo.toml --target riscv64gc-unknown-none-elf -p shell-entry --bin prog-shell
cargo build --locked --manifest-path server/entry/Cargo.toml --target riscv64gc-unknown-none-elf -p shell-entry --bin prog-shell --release
cargo build --locked --manifest-path server/entry/verification/Cargo.toml --target x86_64-unknown-linux-gnu
```

Invoke `server/entry/verification/target/x86_64-unknown-linux-gnu/debug/shell-entry-builder-proof`
with three arguments: the absolute repository root, an absolute output work
directory, and `debug` or `release`. Capture each invocation's standard output
in a separate log. It calls the real image-builder pinned above, resolving the
actual descriptor to Cargo artifacts with Cargo's working directory at the
provider root. `debug` changes only the descriptor profile to `dev`;
the release invocation uses the declared profile unchanged.

Then run, with a clean public Source checkout at the stated baseline:

```sh
python3 server/entry/audit.py --source /path/to/source-baseline \
    --builder-log /path/to/debug.log --builder-log /path/to/release.log \
    --output /path/to/stripped-images
```

The audit checks source substitutions, frozen runtime/SDK/catalogue files,
unique identities, pure Lisp, locked Source provenance, static RISC-V ELF load
layout, executable entry at `_start`, unique Execution runtime symbols, absence
of unresolved symbols, strip preservation of load bytes, the actual loader
16 MiB image bound, and a deterministic gzip round trip. It never executes the
RISC-V program. `server/docs/audit.py` is the preceding library extraction's
historical snapshot proof; its whole-declaration freeze targets that earlier
slice, before this intentional `BUILD` extension.

Executor results: 18 original Lisp tests, 13 original SDK tests, 1 new
declaration test, and 19 original server/adapter tests pass. Both native Cargo
builds and both real descriptor-driven builds pass with locked/offline inputs.
The runtime workspace has 43 packages, the entry closure 41, the unchanged
server library closure 37, and the unchanged pure Lisp closure 13. All 36
external lock entries match Source versions, sources and checksums. The only
new entry closure packages are `shell-entry`, `system-support`, `hub-api` and
`fdt`; no `programs` or `sqware` dependency exists.

Raw debug ELF: 13,447,152 bytes. Raw release ELF: 727,296 bytes. Both are
ELF64 little-endian RISC-V static ET_EXEC, with minimum load address `0x10000`;
their entry addresses are `0x56538` and `0x2c53e`, respectively. Entry addresses
are symbols inside executable load segments, not a fixed base-address claim.
Stripped sizes and load-byte hashes are in `verification-results.json`.

No Source/Vanllia intake, catalogue composition, command IPC, interactive
Shell, launch lifecycle, SMP, frame reclamation or QEMU runtime pass is claimed.
Command PR #2 and Cat PR #5 own their separate resources and workspaces.
