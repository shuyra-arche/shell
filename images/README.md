# shell-images-declaration

Aggregate image-build declaration for the Shell image.

## Ownership

- The Shell entry build is owned by `../declaration` (`shell_declaration::BUILD`).
- Cat image metadata is owned by `../cat/declaration` (`CAT`).
- The five command images are owned by `../commands/declaration`
  (`EMIT`, `UPPER`, `FAIL`, `SPIN`, `WORKERS`).
- This package only re-exports those constants in canonical order:
  shell, cat, emit, upper, fail, spin, workers.

No image-build coordinates, ports, catalogues or bindings are copied here.

## SDK path contract

The real builder resolves the Shell SDK through the in-repository provider
paths. The historical `b910582` extraction pin is recorded only as
provenance metadata and is not a runtime dependency.

## Builder working directory

The real builder must run from the repository root so package manifests such
as `cat/command/Cargo.toml` and `commands/Cargo.toml` resolve as declared.

## Host/native validation intentions

Run the host checks with the pinned `nightly-2026-09-04` toolchain:

```
cargo test --locked --manifest-path images/Cargo.toml --test builds --target x86_64-unknown-linux-gnu
```

The native image descriptor library is compiled for the declared bare-metal
target:

```
cargo build --locked --manifest-path images/Cargo.toml --target riscv64gc-unknown-none-elf
```

The real binaries are built from the preserved workspaces from the repository
root, debug and release (release by adding `--release`):

```
cargo build --locked --manifest-path server/entry/Cargo.toml --package shell-entry --bin prog-shell --target riscv64gc-unknown-none-elf
cargo build --locked --manifest-path commands/Cargo.toml --bins --target riscv64gc-unknown-none-elf
cargo build --locked --manifest-path cat/command/Cargo.toml --bin prog-cat --target riscv64gc-unknown-none-elf
```

The actual real image-builder consumes all seven `BUILDS` with an explicit
repository-root argument and root-target flags, resolving each declared
manifest relative to that root. Integration runtime, IPC, SMP, QEMU, Source and
Vanllia acceptance remain unverified and are not claimed here.
