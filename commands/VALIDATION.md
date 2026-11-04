# Preparation checkpoint validation

Validated on 2026-11-04 against frozen Source
`94e114614bec53424ac55e640fc80cf19a3e7107`. This checkpoint prepares five
command images; it does not release them or consume an unfinished Shell client.

## Completed checks

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

Native final checks used the original pinned `nightly-2026-09-04`, installed in
this environment, with explicit targets and `--locked --offline`. An earlier
host pass used the preinstalled `nightly-2026-10-09`; the table above refers to
the final pinned-toolchain checks. The descriptor lock was resolved normally;
it contains 12 packages, public immutable Git pins, and no `shell-client`.

The Rust tests check actual public `unit::ImageBuild` values. The Python suite
checks exact source-byte reconstruction, fixed dependency and target metadata,
the pending-manifest gate, and explicit per-bin build-script tracking. Neither
suite substitutes a Shell client or executes command behavior.

Root also directly compared each adapted file to its original Git object,
independently of the checked-in hash metadata. Each complete file permits only
`extern crate programs;` to `extern crate execution;` and `#[programs::entry]`
to `#[mold::entry]`; all other bytes match. Ports, exit codes, argument handling,
atomics, task ownership and sleep behavior therefore remain source-identical.

## Isolation and integration boundary

Work is isolated on `codex/shell-command-images` in a new repository. The
filtered source commit preserves the original author/date/message, with its
mapping recorded in `PROVENANCE.md`. No Source, Shell branch, or Vanllia
integration branch was edited. No remote, push, merge, force push, provider
publication, or QEMU run is part of this checkpoint.

The descriptors' `manifest: "Cargo.toml"` is relative to this provider's root.
Later integration owns the selected checkout prefix and must set the effective
manifest to `<selected-provider-checkout>/Cargo.toml`, preserving all other
provider-owned coordinates. Shell retains the original catalogue, resource
bindings, and ports. These images do not become UnitFiles; `cat` and `shell`
are outside this extraction.

## Genuine DeepSeek group

This next PR has a new independent dsh 0.2.1-rc.4 group, routed through
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
Final review is retained separately with the completed native results.
The same reviewer accepted preparation after inspecting the fixes and Root's
evidence; runtime acceptance and publication remain blocked.

## Required dependency and remaining gates

`shell-client` must come from a reviewed immutable revision of the independently
extracted Shell provider. At the last checkpoint, `git ls-remote` succeeded for
`https://github.com/shuyra-arche/shell.git` and returned no refs. No usable
provider revision has been supplied.

The root remains `Cargo.toml.in` with exactly one `@SHELL_PROVIDER_REV@` marker;
there is no root `Cargo.toml` or runtime `Cargo.lock`. A 40-hex string alone is
not evidence of provider compatibility. Before materialization, verify the
actual provider revision and its preserved `Boot::take`, `Boot::args`,
`Boot::port`, `copy`, and `write_all` contracts and dependency identities.

After that dependency is confirmed: materialize the runtime manifest, resolve
its real lock, independently build all five RISC-V ELFs in debug/release, audit
artifact coordinates and runtime/linker symbols, and repeat locked/offline
checks. Final runtime ELF linkage, behavior, catalogue integration and release
remain unverified. Publication stays blocked on those gates. QEMU remains
outside this delegated task.

Raw command exits, stdout/stderr, original snapshots/hashes, history map, role
turns, review findings and provenance audits are preserved in the private
handoff evidence archive.
