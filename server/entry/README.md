# Shell entry binary

`shell-entry` is the `no_std`, `no_main` RISC-V program image for the Shell
server. It is an entry-only package in the `server/` workspace: it depends on
`shell-server` by path plus pinned `env`, `execution`, `mold`, and
`system-support`, and these dependencies do not enter the server library
closure. The binary is `prog-shell`; tests, benches, and doctests are disabled
because the target is a freestanding program validated by the image builder.

## Source

`src/main.rs` is the original ten-line entry source with exactly four
substitutions: `extern crate shell_server` (formerly `programs`), the
`#[mold::entry]` attribute, `shell_server::run()`, and
`system_support::debug::put`. The return type `Result<(), env::Reason>`, the
`map_err` closure, the diagnostic message, and the exit reason `86` are
unchanged. No façade, adapter, or new function is introduced.

## Runtime and link script

`build.rs` reads `DEP_EXECUTION_LINKER_SCRIPT`, the `cargo::metadata` emitted
by the pinned `execution` build script, and forwards it as
`cargo::rustc-link-arg-bin=prog-shell=-T<path>`. It also reruns when the link
script or `src/main.rs` changes. Startup and link layout (image base `0x10000`,
`ENTRY(_start)`) come from `execution`; they are not copied into this tree.
`system_support::debug::put` is the same debug-channel code as the original
`programs::debug::put`.

## Image declaration

`shell_declaration::BUILD` in `declaration/src/build.rs` is the only image
declaration. It names:

- image `crate::PROGRAM.loader.image` (the value declared by the unit file);
- manifest `server/entry/Cargo.toml`, package `shell-entry`, bin `prog-shell`;
- target `riscv64gc-unknown-none-elf`, profile `release`, no features, default
  features on.

The repository-root `.cargo/config.toml` supplies the RISC-V-only codegen
flags (static relocation, forced frame pointers, medium code model). The image
builder sets Cargo's working directory to the supplied provider root, so give
it the repository root to resolve `BUILD.manifest` and load the root flags.
There is no separate `server/` workspace config. The host target is unaffected,
and the existing command workspace configs are untouched.

## Build

Build through `unit::ImageBuild` so the declared coordinates and profile are
used. The builder takes the repository root as its root argument;
`BUILD.manifest` is relative to it, and the builder sets Cargo's `current_dir`
to that root so the root `.cargo/config.toml` RISC-V flags apply. The same ELF
can be built directly with the locked command:

```sh
cargo build --locked --manifest-path server/entry/Cargo.toml -p shell-entry \
    --bin prog-shell --target riscv64gc-unknown-none-elf --release
```

The raw Cargo ELF is the build artifact. Packaging is the image-builder
pipeline's slim helper calling `llvm-objcopy --strip-all`; `unit::ImageBuild`
has no `slim()` method. Keep and report both sizes. Validate the slim ELF's
`PT_LOAD` layout (minimum base `0x10000`) and that `e_entry` equals `_start`
inside an executable `PT_LOAD` (not necessarily exactly `0x10000`).

The host test `declaration/tests/image.rs` checks that `BUILD.image` equals
`PROGRAM.loader.image`, that the declared manifest exists at its real path,
and that the package/bin/target/profile coordinates are connected. It does not
spawn Cargo.

## Deferred

The executor report records that both debug and release RISC-V ELFs were built
and passed `image_builder::executables` with `BUILD` against the repository
root; that command was not rerun for this README edit. Source and Vanllia
catalogue intake, runtime, interactive shell, SMP, and QEMU verification remain
deferred. Command bindings and `PROGRAM` remain authored in `shell-declaration`;
this entry does not duplicate them.
