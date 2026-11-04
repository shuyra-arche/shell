## Review result — Cat-in-Shell PR (branch `codex/shell-cat-image`)

**Verdict: no confirmed P0/P1/P2 blocker.** Approved scope is the frozen code/config snapshot `2144aea5c14b3313fef496d92aecdbdf5417fc1607b632a0161e93e7d2f4fc1d` (`cat/provenance/validated-code.json`) committed as a new branch from main `f4ab6a2d…` plus a **Draft** PR in Shell. Not a merge; no runtime acceptance.

**Behavior (both modes) — verified against actual source**
- `cat/command/src/run.rs:9-24`: selector = first arg owner + `shell_api::MARK` only, no `alive/kind/vestor` narrowing; `Boot::take` → `port("source", Read)` → `port("copy", Write)` → `copy(..., false)`; every selected-path failure maps to `1usize` and returns — no fallback.
- `run.rs:26-47`: independent Terminal/identity path: `Io::injected(owner)`, first `Grant::Resolve.mark()` pie, `Face::direct`, `Wait::AtMost(1000)`, exact greeting string.
- `run.rs:48-56`: echo verbatim; `Eof|Interrupt|Suspend` → `drain()` once → `Ok(())`; all errors → reason `1`.
- Body is the baseline program with `main`→`run`, `run`→`terminal` only (`/workspace/cat-baseline/programs/src/user/cat/main.rs:13-59`); hashes in `import-history.json:94-95` and `validated-code.json:17` agree.

**Descriptor / coordinates**
- `cat/declaration/src/lib.rs:6-17`: only `CAT: unit::ImageBuild` + `BUILDS`; image `cat`, manifest `cat/command/Cargo.toml`, package `cat`, bin `prog-cat`, empty features, riscv64gc target, release, default_features. Matches `cat/command/Cargo.toml:2,15`; no UnitFile/ports/catalogue/Boot duplication; no Shell dependency (`declaration/Cargo.lock:91-97`).
- `cat/declaration/tests/builds.rs:33-68` checks every coordinate and resolves manifest/package/bin/files against the repo root — real, non-vacuous.

**Tests / evidence**
- 23 host tests include production source via `#[path="../command/src/run.rs"]` (`cat/tests/lib.rs:14-15`), not a copy; fixtures shadow crates only. Full event-sequence assertions, no `#[ignore]`, no weakened asserts; no-fallback and drain-once paths covered (`cases/failures.rs:35-122`, `cases/shell.rs:130-147`, `cases/terminal.rs:111-181`).
- `cat-host.log:5-30` = 23 passed; descriptor 2 passed; SDK 13 passed after cache warm-up; the 101 offline failure is preserved and explained (`validation.json:61-95,211-220`); `runtime_acceptance=false`, `qemu_runs=0` — honest.
- Pins: `cat/command/Cargo.toml:27-28` shell `b910582…` (merged interfaces head, `coordination.json:13,53`); no `path=` SDK substitutions; toolchain consistent with root (`cat/rust-toolchain.toml`, `rust-toolchain.toml:2`).
- Scope: all writes under `cat/`; root workspace unchanged (`Cargo.toml:2`) and root SDK test still expects only base commands (`declaration/tests/commands.rs:9`); PR2/PR3 compare lists show zero `cat/` overlap (`coordination.json:73-138`).

**Non-blocking notes (P3)**
1. `cat/declaration/tests/builds.rs:5-11` + README `52-55`: manifest is repo-root-relative; a future checkout prefix relocation silently misresolves unless the prefix is prepended — re-verify in the integration PR.
2. Fixtures use stand-in marks `0x53`/`0x52` (`fixtures/shell_api.rs:1`, `fixtures/grant.rs:9`) vs real `env::Mark::of("shell-launch")` (`api/src/lib.rs:6`); distinct, so selector tests stay meaningful, but real mark semantics are unproven until runtime.
3. `fixtures/execution/boot/args.rs:7-10` `Box::leak`s per call (test-only leak).

**Limits**: I did not recompute SHA-256s, replay git history, or build; hash/equality claims rest on the recorded evidence above. Commit must be created from exactly the tree matching `validated-code.json`; public checkout and consumer/provider checks remain conditional on post-publication verification.