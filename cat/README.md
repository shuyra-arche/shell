# cat

The Cat resource image, kept in the existing Shell repository by the user's
explicit choice. Historical note: an earlier attempt to create a standalone
Cat repository was denied by the hosting service (403); that record is
historical only and will never be retried. The statements below describe this
tree as currently observed, not global behavior outside it.

## Layout

- `command/` — runtime crate `cat` with bin `prog-cat`. Source, manifest,
  build script, and dependency pins are unchanged.
- `declaration/` — independent descriptor workspace
  `shell-cat-image-declaration`, exporting `CAT: unit::ImageBuild` and
  `BUILDS = &[CAT]` on `unit` rev
  `90396f0fda376e0693fb1b9553aa21864d241057`. It carries build coordinates
  only: image `cat`, provider-relative manifest `cat/command/Cargo.toml`,
  package `cat`, bin `prog-cat`, empty features, target
  `riscv64gc-unknown-none-elf`, profile `release`,
  `default_features = true`. No UnitFile, ports, catalogue, or Boot metadata
  is duplicated here.
- `tests/` — 23 host tests covering the runtime workspace; unchanged.

`cat/` is a nested runtime workspace (`members = ["command"]`,
`exclude = ["tests", "declaration"]`); `declaration/` has its own `[workspace]`.
The root SDK workspace and `commands/` are untouched.

## Preserved behavior

- Selector: owner from the first argument matched against `resource::raw::pies()`
  by owner and `shell_api::MARK`. On match it takes `shell_client::Boot::take()`,
  ports `"source"` as Read and `"copy"` as Write, then
  `shell_client::copy(source, copy, false)` and returns; errors map to `1` with
  no fallback.
- Terminal: identity `Resolve` via `Grant::Resolve.mark()` pie and
  `Face::direct`, `AtMost(1000)`, greeting
  `"cat: task={} principal={}:{}\n"`, echo, and
  `Read::Eof | Read::Interrupt | Read::Suspend` draining `io` once before
  `Ok(())`; errors map to `1`.

## Ownership and binding

The ownership source key is the single literal `system/initrd`; resource
`cat`, slot/mark `shell-cat`, permission `FETCH`, Shell-owned. The runtime
selects by Shell owner plus mark. The descriptor names build coordinates only
and duplicates no UnitFile, ports, catalogue, or Boot metadata. The root SDK
`bindings_align_with_commands_and_fetch_initrd` contract passes.

## Build configuration and relocation

Native commands run with cwd `cat/` so `cat/.cargo/config.toml` flags load.
The descriptor manifest path `cat/command/Cargo.toml` is provider-relative;
later integration prepends the chosen Shell checkout prefix once and retains
image `cat`, package `cat`, bin `prog-cat`, target, profile, and features
verbatim. There is no dependency on PR2 `effeb09c170f0259c3d79687754b02aa41f615fe`
or on PR3 `b72bbfca6455159a9bf9a0b250877b7e660e8c52` or on any server branch.
Checked 2026-11-04: PR3 (branch `codex/shell-lisp-library`, based on main
`f4ab6a2dc3924959ac5096305b582cfb479569bd`) changes root
`Cargo.toml`/`Cargo.lock`/`README` plus `docs/lisp*` and `lisp/`, with zero
`cat/` paths; PR2's compare is all `commands/`, zero `cat/`; no published
server branch or PR was observed. Cat writes stay under `cat/` only, leaving
the root workspace/SDK, `commands/`, `lisp/`, and server untouched. Work is on
private branch `codex/shell-cat-image` from main `f4ab6a2d…`; heads must be
rechecked before publication.

## Pins and toolchain

Runtime: `mold f52501f4c91b686eded8b4bcd743b4e96084b7f9`,
`env d3000253cd5d13d466e91fafd4dbb27a5ec5aaae`,
`resource dbc96834f50f094b5c386e54a0a2ae5560dcd7f8`,
`execution 1b333a28bc4c34beaea3f7986a9bb6e8ab2db414`,
`pipe-client 4fce76372beb6c5ab8567232516f156b3d1032eb`,
`shell-api`/`shell-client b910582fc9dc0fd732f279d94f2cfca89f4c8c8f`,
`system-api 9e5d63ca536a176583cf1b0054b4ecf7ddd88257`,
`system-client 5a792ff2d39901a970925b1a6d63206adf6c16d8`,
`terminal-client c447c0761835339a39efdf2ddd8101ebf80db15a`; descriptor `unit`
as above. Toolchain `nightly-2026-09-04`
(`rustc 1.100.0-nightly (a69a63265 2026-09-03)`).

## Validation

Static RISC-V debug/release `prog-cat` ELFs and all host suites pass; exact
results, commands, logs, and graph comparisons are in
`provenance/validation.json` and `provenance/dependency-and-elf.json`.
The fresh independent review approved the frozen code/config snapshot; see
`provenance/review.md`. A public checkout repeated all 38 host tests and both
Cat ELF builds. A separate consumer resolved the declaration from the exact
public Git commit and exercised its coordinates; see
`provenance/public-verification.json`. `runtime_acceptance` remains false;
no QEMU run or merge was performed.
