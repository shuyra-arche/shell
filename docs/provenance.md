# Shell interface provenance

The source is [anreonyr/sqware](https://github.com/anreonyr/sqware) at
[`94e114614bec53424ac55e640fc80cf19a3e7107`](https://github.com/anreonyr/sqware/tree/94e114614bec53424ac55e640fc80cf19a3e7107).
The destination `shuyra-arche/shell` was checked before import and had no refs.
An empty commit establishes `main` as the review base; implementation is on
`codex/shell-interfaces` and is proposed through a Draft PR.

## Historical import

`git-filter-repo` 2.47.0 retained these paths through the stated source baseline:

| Source path | Destination path |
| --- | --- |
| `programs/src/user/shell/api/` | `api/` |
| `programs/src/user/shell/client/` | `client/` |
| `programs/src/user/shell/declaration/` | `declaration/` |
| `programs/tests/shell/contracts.rs` | `api/tests/contracts.rs` |

Ten relevant commits retain their original author, committer, dates, messages
and surviving ancestry. Filtering changes commit IDs and prunes unrelated
commits. [source-history.tsv](source-history.tsv) records every surviving
original-to-filtered commit pair. The filtered tip is
`f2cab3a17527ffe1ca362a77908f6a5d456e7e23`, corresponding to the last relevant
source change, `c96f4a4b5a6795fab899c95bf11c350c20ba2572`.
The reviewed branch joins this filtered ancestry to the empty review base.
No source or remote history is rewritten.

All five production Rust files and all three package manifests are imported
without content changes. The existing protocol test is retained, with only
`use env as abi;` added to restore its former test-package dependency alias.
New workspace configuration, tests and documentation make the slice usable
independently. AGENTS.md is copied from the same source baseline.

## Scope and follow-up

The published interface slice contains `shell-api`, `shell-declaration` and
`shell-client`. Only the API and declaration are selected by default for host
tests. The client consumes target runtime services and is compiled for
`riscv64gc-unknown-none-elf`.

The Shell server, Lisp interpreter (`crates/lisp`), and implementations of
`cat`, `emit`, `upper`, `fail`, `spin` and `workers` remain in sqware. They belong
in this same Shell repository when their runtime dependencies are extracted.
Their common `programs` entry/startup/allocator/panic and target link support
must be examined then. `cat` additionally uses Terminal and System identity;
`workers` uses Execution task spawning and shared atomics.

The source Lisp package currently has a single declared dependency:
`logos = "=0.16.1"` with default features disabled and `export_derive` enabled.
The interpreter and its language tests need an explicit ownership decision and
an independently checked package before the Shell native host/adapters can
leave their local source paths. This interface PR neither creates a Lisp
repository nor changes that package's API, dependencies or ownership.

Command migration does not inherently require Lisp: after their shared image
runtime support is separated from `programs`, the six command implementations
can consume the published Shell client in this repository. Lisp is a prerequisite
for moving the current Shell REPL/native host, rather than a new dependency of
those commands. Keep those two follow-up boundaries separate.

Vanllia Draft PR #3 at `1b619f996869c9d9f22e5cdededf4e127fd37554` still selects
sqware's `shell-declaration` at the source baseline above and still builds the
Shell/command images through legacy source. This PR does not change that pin
or claim runtime independence. A later integration PR can pin this reviewed
public interface revision before taking on command/server/Lisp migration.
Cross-repository consumers must select one public Shell API revision to avoid
creating distinct Rust type identities for the same interface.
