# Command image validation

Validated on 2026-11-04. The formerly missing Shell dependency is resolved by
the verified public commit `b910582fc9dc0fd732f279d94f2cfca89f4c8c8f`.
Command and descriptor checks ran from `commands/`, so its Cargo target flags
were active. The unchanged SDK host regression ran from the Shell repository
root with an explicit host target. All used the original pinned
`nightly-2026-09-04`.

| Final local check | Actual result |
| --- | --- |
| Portable source/manifest/history/build-tracking contract | 6 passed, exit 0 |
| Actual resource descriptor host tests | 3 passed, exit 0 |
| Unchanged Shell API/declaration host regression | 13 passed, exit 0 |
| Five real command binaries, RISC-V debug/release | 10 ELFs, both builds exit 0 |
| Descriptor RISC-V library debug/release | Both exit 0 |
| Runtime locked/offline metadata | exit 0 |
| Actual ELF, artifact and nominal identity audit | exit 0 |
| Immutable Source comparison and existing provider file comparison | All match |

The final runtime lock has 25 packages. All 22 external entries retain Source's
versions, sources and checksums. The only source changes are the command package
and Shell API/client at the verified fixed Git revision. Canonical env, pipe,
resource, execution, mold and Shell identities are unique; no sqware, programs,
composition, Lisp or Shell server dependency is present. The separate descriptor
lock has 12 packages, with its 11 external entries also retaining Source's
versions, sources and checksums.

The artifact audit uses real Cargo executable JSON, not guessed target paths.
Each ELF is static RISC-V ELF64 ET_EXEC, with no interpreter/dynamic segment,
minimum load address `0x10000`, an executable entry equal to its unique `_start`,
unique `clean_ret`, `save_args`, allocator/deallocator/reallocator/error-handler
and panic symbols, and no unresolved symbols. Each is nonempty, within Shell's
16 MiB image bound, and passes gzip roundtrip. Root retained all ten binaries,
hashes and program headers as evidence. This checks linkage, not runtime behavior.

Earlier failures remain in evidence. Initial lock generation selected
bitflags 2.13.2 and unicode-ident 1.0.27; the closure audit rejected those unrelated
upgrades. Root seeded the original Source lock and let Cargo normally resolve
the changed package identities, preserving bitflags 2.13.1 and unicode-ident
1.0.24, then rebuilt both profiles. An offline Shell host run initially lacked
cached bitflags 2.13.1; locked online fetch followed by offline rerun passed.
The first ELF diagnostic incorrectly required the unused alloc-zeroed shim in
optimized binaries; all five release ELFs omit it while all debug ELFs retain
one. The corrected diagnostic still requires each actual core runtime symbol
exactly once and forbids duplicate optional shims. Production code and test
assertions were not changed to hide these diagnostic/setup failures.

The same independent reviewer accepted the final native candidate for a Draft
PR after the documented corrections and descriptor lock parity checks.
Publication and independent public-checkout results are recorded separately.
No QEMU, catalogue integration, scheduler/port flow, or command exit behavior
was executed. Source byte equality and actual native linkage do not establish
those runtime results. The earlier preparation record follows for provenance.

## Preparation checkpoint (historical)

Validated on 2026-11-04 against frozen Source
`94e114614bec53424ac55e640fc80cf19a3e7107`. This checkpoint prepared five
command images; it did not build command binaries or verify runtime behavior.

Root independently ran these checks after the worker's fixes:

| Check | Actual result |
| --- | --- |
| Standard-library Python migration contract | 6 passed, exit 0 |
| Declaration host tests, explicit x86_64 target | 3 passed, exit 0 |
| Declaration RISC-V library build, debug | exit 0 |
| Declaration RISC-V library build, release | exit 0 |
| Declaration locked/offline metadata | exit 0 |
| Five command entry syntax parses | exit 0 |
| Exact comparison against immutable Source Git objects | All five match after only the two recorded entry substitutions |
| Frozen Source files and shared Source checkout | Hashes unchanged; both checkouts clean |

Native preparation checks used the original pinned `nightly-2026-09-04`, with
explicit targets and `--locked --offline`. An earlier host pass used the
preinstalled `nightly-2026-10-09`; the table above refers to the final pinned
preparation checks. The descriptor lock contains public immutable Git pins and
no `shell-client`.

The Rust tests checked actual public `unit::ImageBuild` values. The Python suite
checked exact source-byte reconstruction, fixed dependency and target metadata,
the then-current manifest gate, and explicit per-bin build-script tracking.
Neither suite substitutes a Shell client or executes command behavior.

Root also directly compared each adapted file to its original Git object,
independently of the checked-in hash metadata. Each complete file permits only
`extern crate programs;` to `extern crate execution;` and `#[programs::entry]`
to `#[mold::entry]`; all other bytes match. Ports, exit codes, argument handling,
atomics, task ownership and sleep behavior therefore remain source-identical.

## Current public Shell placement

The work now lives in the existing public Shell clone, private branch
`codex/shell-command-images`, under `commands/`, based on accepted immutable
commit `b910582fc9dc0fd732f279d94f2cfca89f4c8c8f`. No new unrelated public
repository is needed.

Root imported the two old isolated commits linearly under `commands/` with
`git am --directory=commands --committer-date-is-author-date`, preserving
authors, dates and messages. The final history maps original Source commit
`35b3d73e13f695c935665efa6d79b12265af98e7` to prefix-imported
`bdf4f0e7893fcd0c28fc091fe62d1cc85d8ae400`; the old isolated filtered
checkpoint `d3848262c01eefbe03526959a7eecfb75380eebf` remains historical
archive only. The prepared checkpoint
`16a6bb6740f9f3c3994b89d34eabfdb157a452c5` maps to prefix-imported
`8d8d39a44a7540dd76848ae404840aee343ea462`.

The descriptors use the provider-relative manifest `commands/Cargo.toml`.
Later integration must prefix that with the actual selected Shell checkout:
`<selected-shell-checkout>/commands/Cargo.toml`, preserving all other
provider-owned coordinates. Shell retains the original catalogue, resource
bindings and ports. These images do not become UnitFiles; `cat` and `shell` are
outside this extraction.

## Genuine DeepSeek group

This PR has a new independent dsh 0.2.1-rc.4 group, routed through
`opencode-go` to `deepseek-v4.1-flash`:

- namer: `session-2373d4e0-73bc-4384-bab8-15c40d2050dd`
- sole implementation worker: `session-ebfc94bf-f307-4706-878e-24d379e86903`
- independent reviewer: `session-19f68f81-367a-4f54-b067-eb191cc443dd`

Routing and permission states are extracted from actual persisted request
context, not inferred from role labels. Namer/reviewer use read-only policy;
worker uses workspace-write. Bash, PowerShell and job tools are disabled;
filesystem policy and approval handling remain active. Root performs native
checks, normal lock resolution, provenance audit and local Git delivery.

The initial final-review pass required per-bin source tracking in `build.rs`,
the explicit host target in the README, and the correct 20-millisecond unit for
`park(20)`. The same worker fixed these; the rejected pass remains in evidence.
The same reviewer accepted preparation after inspecting the fixes and Root's
evidence. That was preparation acceptance; the final native review is separate.

## Verified provider revision and remaining gates

The prior missing-provider blocker is resolved. The public Shell PR1 draft head
is exactly `b910582fc9dc0fd732f279d94f2cfca89f4c8c8f`. Root's verification
evidence is
`/workspace/evidence/shell-command-publication/shell-provider-verification.json`.
It records that the provider `api/` and `client/` production Rust bytes equal
frozen Source `94e114`, including unchanged `Boot::take`, `Boot::args`,
`Boot::port`, `copy` and `write_all`, and that the pipe, env, execution and
resource identities align.

`commands/Cargo.toml` now pins `shell-client` to that immutable Git revision.
There is no `Cargo.toml.in`, marker or local client path in this workspace.

Root completed the real debug/release command builds, runtime lock and artifact
audits listed above. Catalogue integration and actual command behavior remain
unverified. No QEMU run, integration pass or runtime behavior is claimed here.
