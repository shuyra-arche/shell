# Provenance

## Source identity

- Baseline snapshot: `/workspace/cat-baseline`
- Source repository: `https://github.com/anreonyr/sqware.git`
- Baseline revision: `94e114614bec53424ac55e640fc80cf19a3e7107`
- Source path: `programs/src/user/cat/main.rs`
- Source SHA-256: `d0e6fecf9c281245fc27412579bcdb36406dd4c43f8cdcb96018ba6fdc041aca`

Destination:

- Repository: `shuyra-arche/cat`
- Package/library: `cat`
- Binary and image: `prog-cat` / `cat`
- Main path: `command/src/main.rs`
- Role: `shell-command`
- Composition owner / owner: `shuyra-arche/shell`

The baseline is frozen source, not a new upstream history. No license is
invented; this snapshot adds no license text.

## Production mapping

- `programs/src/user/cat/main.rs` is mapped to `command/src/main.rs` and
  `command/src/run.rs`.
- The baseline `#[programs::entry] fn main() -> Result<(), env::Reason>` is
  replaced by `#[mold::entry] fn main() -> Result<(), env::Reason>`, which
  delegates to `cat::run()`. The direct `programs` dependency is removed.
- The baseline private identity function `run() -> Result<(), ()>` is renamed
  `terminal()` and kept near-verbatim in `command/src/run.rs`.
- The new public `cat::run() -> Result<(), env::Reason>` contains the original
  Shell selector, Boot/port/copy calls, and fallback to the identity path.
  Shell-selected failures return reason 1; Terminal failures map to reason 1.
- `command/src/lib.rs` is `no_std` plus `alloc` and reexports `run`.

The selector, port names and directions, `false` copy flag, greeting string,
verbatim echo loop, EOF/Interrupt/Suspend drain-once behavior, and error
mapping are unchanged.

## Migration / supply / ports

The existing migration baseline describes:

- Image `cat`, binary `prog-cat`, source `programs/src/user/cat/main.rs`
- Target repository `shuyra-arche/cat`, target path `command/src/main.rs`
- Role `shell-command`, composition owner `shuyra-arche/shell`

Shell retains the existing supply binding and port declaration. The exact
Shell-owned supply metadata is:

- Source key: `system/initrd`
- Resource key: `cat`
- Slot mark: `shell-cat`
- Permission: `FETCH`

Cat consumes:

- `source`: `pipe_client::Direction::Read`
- `copy`: `pipe_client::Direction::Write`
- Copy third flag: `false`

No Unit, Declaration crate, BUILD crate, composition entry, or Deployment is
created by this package.

## Shell pipe mode

- Selector: first Boot argument owner; any raw pie with matching owner and
  `shell_api::MARK`.
- The outer selector uses only owner and mark. Pie entries that are dead,
  non-Pole, or have another vestor still select Shell when owner and mark
  match.
- Boot: `shell_client::Boot::take`.
- Ports: `source` Read, `copy` Write.
- Copy: `shell_client::copy(source, copy, false)`.
- Any failure after selection: reason 1, no Terminal fallback.

The Shell provider is verified at
`https://github.com/shuyra-arche/shell/pull/1`, immutable revision
`b910582fc9dc0fd732f279d94f2cfca89f4c8c8f`. The coordinator's detached, clean
clone matched all eight frozen API/client/declaration source and manifest files
byte-for-byte (`docs/shell-provider-verification.json`). Both
`shell-api` and `shell-client` are pinned to that revision in
`command/Cargo.toml`. The unchanged `client/src/lib.rs` SHA-256 is
`51154dc20d2b9f652caef4febb1ab640ccc8923cde0024891230acde705b3614`.

## Terminal/identity mode

- IO: `terminal_client::Io::injected(first argument owner)`.
- Resolver pie: first raw pie whose mark equals `Grant::Resolve.mark()`.
- Direct face: `Face::direct(entry.owner, Grant::Resolve, entry.token)`.
- Resolve call: `Wire::Resolve(unit::self_id())`, `Wait::AtMost(1000)`.
- Greeting: `cat: task={self_id} principal={authority}:{slot}\n`.
- Data: echoed byte-for-byte.
- EOF, Interrupt, Suspend: drain once, then succeed.
- Failures: injected IO, pie lookup, direct face, call, non-binding reply,
  greeting write, read, data write, or drain map to reason 1.

There is no added cleanup, retry, owner/kind filter, or fallback after a
selected Shell failure.

## Runtime dependency revisions

These are the immutable revisions from the actual `programs/Cargo.toml`:

- `mold`: `f52501f4c91b686eded8b4bcd743b4e96084b7f9`
- `env`: `d3000253cd5d13d466e91fafd4dbb27a5ec5aaae`
- `resource`: `dbc96834f50f094b5c386e54a0a2ae5560dcd7f8`
- `execution`: `1b333a28bc4c34beaea3f7986a9bb6e8ab2db414`
- `pipe-client`: `4fce76372beb6c5ab8567232516f156b3d1032eb`
- `system-api`: `9e5d63ca536a176583cf1b0054b4ecf7ddd88257`
- `system-client`: `5a792ff2d39901a970925b1a6d63206adf6c16d8`
- `terminal-client`: `c447c0761835339a39efdf2ddd8101ebf80db15a`
- `shell-api`: `b910582fc9dc0fd732f279d94f2cfca89f4c8c8f`
- `shell-client`: `b910582fc9dc0fd732f279d94f2cfca89f4c8c8f`

`unit::self_id()` resolves through the `unit` module reexported by `env`; no
separate direct `unit` crate dependency is declared. No direct `pipe-api`
dependency is declared because the source uses the `pipe_client` path.

## Toolchain distinction

- Frozen source declaration in the baseline: `nightly-2026-09-04`.
- Actual available validation toolchain: `nightly-2026-10-09`.

`rust-toolchain.toml` records the available validation toolchain. It does not
claim the frozen source declaration was already that toolchain.

## Native validation

The coordinator ran locked/offline RISC-V release and debug builds for package
`cat`, binary `prog-cat`:

- Release build: exit 0 in 20.445 s; static `ET_EXEC`, machine 243, no
  interpreter, `_start`/`clean_ret` present; SHA-256
  `679e525696d90b09a2220ee6ece0a72a7701fada6a2f1f326b64e46e1b482b94`
- Debug build: exit 0 in 13.499 s; static `ET_EXEC`, machine 243, no
  interpreter, `_start`/`clean_ret` present

The native dependency graph has 27 packages. All 24 non-Shell external lock
entries match the baseline versions, sources, and checksums; canonical ABI/API
type identities are unique; no Source/programs/Lisp dependency is present.
Shipped evidence is in `docs/validation.json` and
`docs/dependency-and-elf.json`; raw build/test logs ship under `docs/logs/`.

## Host tests

`tests/Cargo.toml` is a separate dependency-free host workspace. Its library
compiles the actual `command/src/run.rs` using
`#[path = "../command/src/run.rs"]` and crate-self aliases for `env`,
`execution`, `resource`, `pipe_client`, `shell_api`, `shell_client`,
`system_api`, `system_client`, and `terminal_client`. The API members are
reexported at the test-crate root; no production provider is copied.

Fixture data is split by type. Thread-local scripts record ordering,
arguments, and failures so parallel tests remain isolated. Coverage includes:

- Shell selection with exact source Read/copy Write ports and
  `uppercase: false`
- Outer selector ignoring alive/kind/vestor, then Boot failure -> reason 1
- Wrong owner and wrong mark falling back to Terminal
- Selected-Shell Boot/port/copy failures without Terminal fallback
- Missing args, injected IO, lookup, direct face, call, binding, greeting,
  read, write, and drain failures
- Identity greeting, self ID, `AtMost(1000)`, direct grant fields
- Verbatim arbitrary and empty data echo
- EOF, Interrupt, and Suspend drain once with no further read

The final locked/offline host command passed all 23 tests with 0 failures:
`cargo test --locked --offline --manifest-path tests/Cargo.toml --target
x86_64-unknown-linux-gnu`. The first run failed on the projection path outside
the repository; the second failed on the test `Binding.current` shape. Both
fixture issues were corrected without changing production behavior or
weakening assertions. See `VERIFICATION.md` for the command, evidence, and
limits.

## Finalization gates

1. Completed: immutable Shell revision
   `b910582fc9dc0fd732f279d94f2cfca89f4c8c8f` verified with eight-file frozen
   source equivalence.
2. Completed: final root and command `Cargo.toml` files with both Shell
   packages pinned to that revision.
3. Completed: native dependency resolution and lockfile; 27 packages, with all
   24 non-Shell external entries matching the baseline.
4. Completed: locked/offline RISC-V release and debug static ELF builds with
   `DEP_EXECUTION_LINKER_SCRIPT` and `mold` entry linkage.
5. Completed: source behavior review and 23-pass/0-fail dependency-free host
   test run.
6. Delivery intent: Draft PR only (`draft_pr: true`, `merge: false`), status
   `validated-for-draft-pr`; no PR existence is claimed yet.
7. Unperformed/out of scope: Integration/Vanllia IMAGE BUILD manifest mapping,
   runtime validation, and QEMU. No new Cat Unit mapping is needed; Shell keeps
   the existing supply binding and port declaration.
