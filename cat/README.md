# cat — isolated command preparation

Confirmed destination: `shuyra-arche/cat`, package/library `cat`, binary
`prog-cat`, main path `command/src/main.rs`. This snapshot is validated for
Draft PR delivery; merge is not authorized.

No Unit, Declaration crate, composition entry, or Deployment lives here. Shell
keeps the existing supply binding and port declaration; Vanllia later maps the
external Cat image.

## Preserved behavior

- Shell pipe mode is selected when the first argument is an owner and any raw
  pie has that owner and `shell_api::MARK`. The outer selector checks only
  owner and mark; it does not add alive, kind, vestor, or other narrowing.
- Once selected, `Boot::take`, the `source` Read port, the `copy` Write port,
  or `shell_client::copy(source, copy, false)` failure returns reason 1. It
  never falls back to Terminal.
- Terminal/identity mode uses `Io::injected`, the first Resolve-mark pie,
  `Face::direct`, and `Resolve(unit::self_id())` with `AtMost(1000)`. It emits
  the original identity greeting, echoes data verbatim, and on EOF, Interrupt,
  or Suspend drains once and succeeds.
- Terminal failures propagate and map to reason 1. There is no added cleanup,
  retry, or other filtering.

The real Shell client is not reproduced here.

## Production mapping

- Baseline `programs/src/user/cat/main.rs` is `command/src/main.rs`.
- The baseline `#[programs::entry] fn main()` becomes `#[mold::entry] fn
  main()` and delegates to `cat::run()`; it no longer depends on `programs`.
- The baseline identity function `fn run() -> Result<(), ()>` is extracted
  near-verbatim as the private `terminal()` in `command/src/run.rs`.
- The new public `cat::run() -> Result<(), env::Reason>` owns both the Shell
  selector and the Terminal fallback, preserving the original reason-1
  mapping.
- `command/src/lib.rs` is `no_std` plus `alloc` and reexports `run`.

## Migration / image metadata

- Image: `cat`
- Binary: `prog-cat`
- Source: `programs/src/user/cat/main.rs`
- Target repository: `shuyra-arche/cat`
- Target path: `command/src/main.rs`
- Role: `shell-command`
- Composition owner / owner: `shuyra-arche/shell`
- Shell-owned supply: source `system/initrd`, resource `cat`, slot `shell-cat`,
  permission `FETCH`
- Shell command binding: `cat`
- `source` port direction: Read
- `copy` port direction: Write
- Copy third flag `uppercase`: `false`

## Verified Shell provider

- Pull request: `https://github.com/shuyra-arche/shell/pull/1`
- Verified immutable revision: `b910582fc9dc0fd732f279d94f2cfca89f4c8c8f`
- The coordinator's detached, clean clone matched all eight frozen
  API/client/declaration source and manifest files byte-for-byte.
- `client/src/lib.rs` SHA-256:
  `51154dc20d2b9f652caef4febb1ab640ccc8923cde0024891230acde705b3614`
- Evidence: `docs/shell-provider-verification.json`

The provider's own verification reports its host tests and RISC-V builds. Cat's
native dependency lock, locked/offline release and debug ELF builds, and final
23-pass/0-fail host run have also passed; see `docs/validation.json`.

## Toolchains

- Frozen source declaration: `nightly-2026-09-04`
- Actual available validation toolchain: `nightly-2026-10-09`

`rust-toolchain.toml` documents the available validation toolchain. It does
not claim the frozen source and validation toolchains are identical.

## Layout

- `command/src/main.rs`: `no_std`/`no_main`; `mold::entry` to `cat::run`.
- `command/src/lib.rs`: `no_std` plus `alloc`; reexports the extracted run.
- `command/src/run.rs`: extracted selector and Terminal implementation.
- `command/build.rs`: only `DEP_EXECUTION_LINKER_SCRIPT` and source tracking.
- `command/Cargo.toml`: final package manifest pinned to the verified Shell
  revision.
- `Cargo.toml`: workspace manifest; panic-abort profiles and `tests` excluded.
- `tests/`: dependency-free host workspace projecting the actual
  `command/src/run.rs` through `#[path]` and crate-self aliases.

`unit::self_id()` resolves through `env`'s reexported `unit` module; no
separate direct `unit` crate dependency is declared.

## Validation status

The root and command manifests and native lockfile are finalized. Native
dependency resolution passed with 27 packages; all 24 non-Shell external lock
entries match the baseline versions, sources, and checksums. Canonical ABI/API
type identities are unique, and there is no Source/programs/Lisp dependency.

Both `--locked --offline` RISC-V builds passed for package `cat`, binary
`prog-cat`:

- Release: 20.445 s; static `ET_EXEC`, machine 243, no interpreter; SHA-256
  `679e525696d90b09a2220ee6ece0a72a7701fada6a2f1f326b64e46e1b482b94`
- Debug: 13.499 s; static `ET_EXEC`, machine 243, no interpreter

Both ELFs contain the expected `_start`/`clean_ret`. The final locked/offline
host run passed 23 tests with 0 failures. Earlier projection-path and
fixture-nesting compile failures remain recorded in `VERIFICATION.md`.

Shipped validation references are `docs/validation.json`,
`docs/dependency-and-elf.json`, and `docs/shell-provider-verification.json`;
raw logs ship under `docs/logs/`. Delivery intent is Draft PR only
(`draft_pr: true`, `merge: false`); no PR is claimed yet. Runtime behavior,
Integration/Vanllia image work, and QEMU remain unperformed and out of scope
for this worker.
