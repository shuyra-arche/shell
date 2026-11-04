# Validation

All results below were produced on pinned `nightly-2026-09-04` (`rustc
1.100.0-nightly (a69a63265 2026-09-03)`) with `--locked --offline`. Full
commands, logs, and hashes: `provenance/validation.json`,
`provenance/dependency-and-elf.json`.

| Check | Result |
| --- | --- |
| Cat debug + release native builds | pass |
| `prog-cat` ELF (both profiles) | static ELF64 `ET_EXEC`, machine 243, load base `0x10000`, executable `_start` entry, unique `_start`/`clean_ret`, no dynamic segment/interpreter/unresolved symbols, gzip roundtrip |
| Existing 23 Cat host tests | pass |
| Descriptor 2 host tests | pass |
| Descriptor RISC-V debug + release library builds | pass |
| Root SDK host tests | 13 pass (rerun) |
| Runtime dependency graph | 27 packages, 24 external entries matching baseline |
| Descriptor dependency graph | 12 packages, 11 external entries matching baseline |
| Canonical identities | unique; no Source/programs/composition/Lisp entries |
| Root SDK tracked base files | 23 byte-identical |
| Cat runtime and 23 tests | byte-identical to prior candidate |

Failure history: the first root SDK offline run exited 101 because the `unit`
Git checkout was absent from the cache. `cargo fetch --locked` warmed the
cache and the SDK rerun passed 13 tests. The initial attempt is not counted as
a pass.

The fresh independent DeepSeek reviewer approved frozen code/config snapshot
`2144aea5c14b3313fef496d92aecdbdf5417fc1607b632a0161e93e7d2f4fc1d`
with no confirmed P0/P1/P2 blocker. See `provenance/review.md` and the separate
group receipt.

Public commit `db92567f447db9ba19584cb5e85fdf94ca6b8bb4` was anonymously
checked out. Its reviewed code/config hashes match exactly; both Cat native
builds, 23 Cat tests, 2 descriptor tests and 13 SDK tests passed again. A fresh
consumer resolved `shell-cat-image-declaration` from that immutable Git
revision and ran host coordinate assertions. Its 13-package closure excludes
the Cat runtime and Shell client. The public worktree stayed clean. Reports
and actual logs are in `provenance/public-verification.json`.

The later server head `7a2f99e268784693975b5e244b28795be50c218c` was compared
against the base before branch creation. Its root/Lisp/server changes contain
no `cat/` path; the receipt is in `provenance/prepublication-coordination.json`.
This adds no Cat dependency on that branch or PR #2/#3. Only our new branch
was created; main and peer refs were not updated. Draft PR delivery follows this
validated snapshot. No merge or force push is part of this work.

`runtime_acceptance` is false and no QEMU execution was performed.
