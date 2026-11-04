# Cat independent provider verification

Source: https://github.com/anreonyr/sqware.git at
`94e114614bec53424ac55e640fc80cf19a3e7107`. No runtime acceptance is claimed.

## Shell provider verification

The coordinator anonymously checked out
`shuyra-arche/shell@b910582fc9dc0fd732f279d94f2cfca89f4c8c8f` and compared
all eight API/client/declaration sources and manifests byte-for-byte with the
specified Source baseline. All match. In particular the original Boot/port/
copy implementation, including pipe EOF handling, is unchanged. Both direct
Shell dependencies use this full immutable revision. See
`docs/shell-provider-verification.json`.

## Actual commands

With installed toolchain `nightly-2026-10-09`, all exited 0:

```sh
cargo build --locked --offline --release --package cat --bin prog-cat
cargo build --locked --offline --package cat --bin prog-cat
cargo test --locked --offline --manifest-path tests/Cargo.toml --target x86_64-unknown-linux-gnu
```

The first two commands use `.cargo/config.toml`'s
`riscv64gc-unknown-none-elf` target. Actual results:

- Release and debug produced static ET_EXEC RISC-V ELF executables, machine
  243, with no interpreter. Release `_start` equals ELF entry `0x135ca`;
  `clean_ret` is present at `0x10000`.
- Final host run: **23 passed, 0 failed**. Tests project actual
  `command/src/run.rs` against recording fixtures and exercise Cat's call
  order, arguments, output bytes, error mapping and termination.
- Native graph: 27 packages. All 24 non-Shell external lock entries equal
  baseline versions/sources/checksums. Canonical ABI/API identities are
  unique; Source, programs, composition and Lisp are absent.

`docs/validation.json` records command exits/timings/log hashes.
`docs/dependency-and-elf.json` records graph checks and ELF hashes.

Two earlier host attempts failed compiling fixtures: an incorrect source
projection path, then incorrect identity nesting. The same worker fixed both
without changing production behavior or weakening assertions. Raw logs remain
in the private handoff evidence.

Fixtures do not execute real IPC, pipe transport, identity services, kernel
resource lifetime or the RISC-V image. ELF construction and inspection are
not runtime validation. No QEMU was run.

## Source parity and history

Production imports/bodies match the original Cat file after only the
entry/function renames and blank-line normalization. Extracted run.rs SHA-256:
`f479b4d3c7329619a69b8aa8998363bfde9ef5858b9aad78c60a1261e395d9e8`.

Seven relevant source commits were filtered to `command/src/main.rs`.
Their authors, author/committer dates and messages were checked against the
original objects. `source-history.json` records the map. Shared Source and
the frozen baseline working trees remain clean.

## Independent group

This PR uses a fresh genuine `opencode-go / deepseek-v4.1-flash` group through
a scoped direct API harness, not dsh:

- Namer: `cat-namer-9c4108ac-1381-437e-83cc-7e0b9da1b221`
- Sole code worker: `cat-worker-3f552dbd-50b3-4efd-9fe9-657264e1c1d9`
- Reviewer: `cat-reviewer-f3694293-6dad-4a6c-a502-845cd4bdab03`

Initial review confirmed behavior parity and meaningful tests. Final native
review approved code/config snapshot
`6bd71b887775eb4763e16191aaacd18a7f39c33b81e9b1b9d490d98a74b58a87`.
All P2 documentation findings were corrected and cleared by the same reviewer.
See `docs/review.md`. Actual session/request-model records, model-reported
response metadata and tool actions are retained privately; private model
state is excluded from deliverables.

Publication is blocked by GitHub integration permissions: organization
repository creation returned HTTP 403, `Resource not accessible by integration`.
Subsequent canonical repository lookup returned 404 and repository search was
empty. No repository, README, branch or PR was created. The bootstrap graft
was not performed; the seven filtered source commits are retained locally.
The future publisher must attach this history to the eventual repository base
without losing authors, dates or messages. This checkpoint claims no PR.

Vanllia image mapping and runtime validation are separate tasks. This work
changes no shared Source, Shell or Vanllia and performs no merge, force push
or QEMU execution.
