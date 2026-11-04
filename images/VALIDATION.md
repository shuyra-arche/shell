# Consolidation verification

The integration branch retains the exact PR #6 entry/Lisp/server head
`1165fa40a2ed3f32cbd6bc1966bb70041cdcc600`, PR #2 commands head
`effeb09c170f0259c3d79687754b02aa41f615fe`, and PR #5 Cat head
`df651c37d99f15e26cf7d76a24f0a63a4b58df58` as parents of the union checkpoint
`d84eaa8b995af065cc978872b22520ae4f8dc1d9`. Its Draft PR targets main at
`f4ab6a2dc3924959ac5096305b582cfb479569bd`. No existing PR or branch is merged,
moved or overwritten. The original source baseline remains
`anreonyr/sqware@94e114614bec53424ac55e640fc80cf19a3e7107`.

The new provider changes only internal SDK dependency coordinates, their two
locks, the matching extraction-coordinate assertion and its historical fixture,
the image inventory, verification tools, and explanatory documentation. Existing
entry/server/Lisp/SDK code and all original Rust tests are byte-identical. All
non-Shell dependency versions, sources and checksums remain unchanged. The old
`b910582` pin is tested as historical evidence; the live migration assertion now
requires exactly `../client`. No dependency patch or source override is used.

The new inventory lock is seeded from the reviewed root lock and then pruned
by Cargo. Its initial unseeded resolution selected newer `bitflags` and
`unicode-ident` versions; the Source-projection audit rejected that attempt.
The corrected lock retains the baseline versions, and the inventory checks
are rerun against it. This correction does not change a validation threshold.

The generic ELF audit initially assumed every compiler allocation wrapper was
retained. Release removes the unused `alloc_zeroed` wrapper in six resource
images; Shell retains it. The corrected check requires the eight core runtime
symbols in every image and at most one copy of every present wrapper, with the
single Execution heap and no unresolved symbols. It records wrapper absence
explicitly rather than requiring unused code to survive optimization.

Run with `nightly-2026-09-04`, the RISC-V target, Python 3.11+, `nm` and
`llvm-objcopy`. Commands below run from the repository root; add `--offline`
after dependencies are cached.

```sh
cargo test --locked --target x86_64-unknown-linux-gnu -p lisp -p shell-api -p shell-declaration
cargo test --locked --manifest-path server/Cargo.toml --target x86_64-unknown-linux-gnu -p shell-tests -p shell-adapter-tests
cargo test --locked --manifest-path cat/tests/Cargo.toml --target x86_64-unknown-linux-gnu
cargo test --locked --manifest-path commands/declaration/Cargo.toml --target x86_64-unknown-linux-gnu
cargo test --locked --manifest-path cat/declaration/Cargo.toml --target x86_64-unknown-linux-gnu
cargo test --locked --manifest-path images/Cargo.toml --target x86_64-unknown-linux-gnu
python3 commands/tests/test_migration.py
cargo build --locked --manifest-path images/Cargo.toml --target riscv64gc-unknown-none-elf
cargo build --locked --manifest-path images/verification/Cargo.toml --target x86_64-unknown-linux-gnu
```

Run `images/verification/target/x86_64-unknown-linux-gnu/debug/shell-images-builder-proof`
with the absolute repository root, an absolute build-work directory, and
`debug` or `release`. Save each invocation's standard output in a log. The
verifier calls actual `image_builder::executables` at fixed public revision
`17f294bae58ef5526a18fd6a88a88cc41850d4a6`, using the seven exported descriptors.
Debug overrides only their profile to `dev`; release uses them unchanged.
The builder runs Cargo at the provider root, loading the root RISC-V flags.

With a clean Source checkout at the baseline above, run:

```sh
python3 images/audit.py --source /path/to/source-baseline \
    --builder-log /path/to/debug.log --builder-log /path/to/release.log \
    --output /path/to/stripped-images
```

This compares original Git blobs, original migration methods, manifest fields,
lock projections and package identities, and audits fourteen actual ELFs for
static RISC-V executable layout, entry, runtime uniqueness, unresolved symbols,
strip preservation, the loader's real 16 MiB limit, and gzip round trips. Earlier
slice-wide freeze scripts remain historical proofs bound to their reviewed
commits; their assertions are not relaxed to accommodate this new composition.

The separate verification tool has a locked host-tool graph and is not part of
any runtime closure. Runtime graphs have 25 commands, 27 Cat, and 43 server
packages; their Shell packages use this checkout. The pure inventory graph has
17 packages, with no Execution, Lisp, server or client runtime dependency.

Cross-checkout byte reproducibility is not claimed: absolute source paths and
Cargo identities can affect ELF bytes and layout. Source identity, locked
resolution, successful rebuilds, and the same executable contracts are checked
separately. Results are recorded by the executor in the PR and audit evidence.

Static builds and host tests do not establish actual boot/launch handshake,
interactive Shell/Lisp/terminal behavior, command IPC, SMP or frame reclamation.
No QEMU, shared Source change, Vanllia intake, PR merge or force push is performed.
Downstream intake must pin the final immutable provider head, apply the provider
manifest prefix once, retain the caller's RISC-V flags and use the original
`shell-declaration::PROGRAM` and six command bindings.
