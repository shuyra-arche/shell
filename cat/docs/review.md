## Final doc closure — same reviewer/session

**P2 documentation defects: cleared.** No code/config blocker.

- `README.md:4-5` now states validated for Draft PR delivery, merge not authorized; `:66-68` confirms native lock, both locked/offline RISC-V builds and 23-pass/0-fail host run **have passed**; `:111-116` points to `docs/validation.json`, `docs/dependency-and-elf.json`, `docs/shell-provider-verification.json`, `docs/logs/`, states `draft_pr: true`/`merge: false`, no PR claimed, runtime/Vanllia/QEMU unperformed. The earlier false "not ready/not already passed" prose is gone.
- `PROVENANCE.md:114-115` now carries the actual `b910582f…c8c8f` for both Shell packages; `:143-144` shipped evidence under `docs/` incl. `docs/logs/`; `:190-191` delivery intent, no PR existence claimed.
- `provenance.json:3-10` `state`/`delivery` = validated-for-draft-pr, `pr_claimed: false`; `:36` comparison evidence now `docs/shell-provider-verification.json`; `:100-101` actual Shell revs; `:121-212` log/evidence paths under `docs/`; validation results unchanged (23/0; release/debug ET_EXEC machine 243, entry `0x135ca`/`0x21c52`); finalization gates `:216-224` correctly avoid claiming publication.
- `docs/validation.json:18,35,52` log paths corrected to `docs/logs/` with hashes; `:56-57` `qemu_runs: 0`, `runtime_acceptance: false`.

**Snapshot integrity:** unchanged `6bd71b88…a74b58a87`; `docs/validated-code.json` binds only code/config (manifests, lock, `.cargo/config.toml`, toolchain, `command/*`, tests), which excludes the corrected docs, so doc edits do not disturb the bound hash. No source/tests/manifests/lock change.

**Non-blocking wording nit:** `provenance.json:105-109` `out_of_scope` values are `false`; consider `"unperformed"` to avoid “false = performed” misreading. Not a blocker.

**Publication blocker is external, not code:** GitHub org repo creation returned 403 “Resource not accessible by integration”; canonical repo GET 404 and search empty — no repository, branch, README or PR exists. Publication must stop until the target repository is created/made accessible; do not retry/bypass. Consequently the bootstrap graft was **not** done: `source-history.json` records the seven actual local filtered commits on `source-history`; they must not be described as grafted or published, and the future publisher must attach them to the eventual repository base preserving authors/dates/messages.

**Runtime, Integration/Vanllia image mapping and QEMU remain unperformed and unclaimed.** Approval stands for the immutable validated code/config snapshot only, for Draft PR delivery, no merge.