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
group receipt. Public checkout verification and publication are the next
steps; no PR submission is claimed by this local validation snapshot.
`runtime_acceptance` is false and no QEMU execution was performed.
