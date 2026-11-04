# Lisp crate extraction

Status: `lisp/` is present at the repository root as a workspace member. This
document is the procedure for import, ownership, and verification. Test results
belong in the executor's generated audit evidence, not here.

## Ownership

- The `lisp` package at `lisp/` is owned as a workspace member. It owns the
  language: `source`, `syntax`, `diagnostic`, `library`, and `runtime` (value,
  heap, environment, machine, native). Its public surface is exactly the
  current re-exports in `lisp/src/lib.rs`.
- The root `Cargo.toml` `[workspace.package]` remains the sole source of the
  version and edition. `lisp/Cargo.toml` inherits them with
  `version.workspace` and `edition.workspace`. `default-members` remain
  `["api", "declaration"]`.
- `api`, `client`, and `declaration` are unchanged. The six commands and their
  program runtime are unchanged.

## Boundaries

- Dependency direction: `shell-server -> lisp -> logos`. No
  `api`/`client`/`declaration`/`commands` crate depends on `lisp`.
- `logos = "=0.16.1"` with `default-features = false` and
  `features = ["export_derive"]` is the sole external dependency of `lisp`.
- `#![no_std]`, `extern crate alloc`, and `#![forbid(unsafe_code)]` are
  preserved.
- Shell's native host and `stdlib.lisp` remain Shell runtime, not part of this
  crate and not part of this slice. Source keeps building and running its own
  copies until a separate intake slice.

## History and baseline

- Imported tree: Source baseline
  `94e114614bec53424ac55e640fc80cf19a3e7107`, path `crates/lisp/`.
- Imported content: `Cargo.toml`, `README.md`, `src/**`, and `tests/**` — 20
  files including the manifest, byte-identical to the Source baseline. The 18
  test functions are in the single file `lisp/tests/language.rs`.
- Four original Source commits were retained and grafted onto verified Shell
  main `f4ab6a2dc3924959ac5096305b582cfb479569bd`, on branch
  `codex/shell-lisp-library`.
- There was no standalone Lisp repository. The earlier local Lisp candidate
  and group, and the Shell-server groups and checkpoints, are archived.
- The command-nested-workspace branch from PR2 `effeb09c…` is separate; this
  branch does not change root workspace semantics beyond adding the member.

## Root integration surface

- `Cargo.toml`: `"lisp"` added to `members` only. `default-members`,
  `[workspace.package]`, and `[profile.dev]` are unchanged.
- `README.md`: the crate list, build commands, and deferred boundaries describe
  the fourth crate and that it has no consumer in this repository yet.
- `lisp/**`: no edits. No added facade, features, tests, or substitutes.
- `Cargo.lock`: seeded and resolved by Cargo from the Source versions, with
  existing pins preserved. It is not hand-edited.
- `rust-toolchain.toml`, `AGENTS.md`, and `commands/` are unchanged.

## Verification procedure

The executor runs these and records exact commands and outputs in the audit
docs. Do not edit this procedure to claim results.

1. Resolve the lock from the actual graph, then use `--locked` for the rest.
   Adding a member changes `Cargo.lock`; the `--locked` commands below require
   the regenerated lock to be committed.
2. `cargo test --locked -p lisp --target x86_64-unknown-linux-gnu` — the 18
   language tests in `lisp/tests/language.rs`.
3. `cargo check --locked -p lisp --target riscv64gc-unknown-none-elf`.
4. `cargo test --locked` — the existing API and declaration host tests,
   unchanged.
5. `cargo metadata --locked` shows `lisp` as a member and shows no new edges
   from `api`, `client`, or `declaration`.
6. External consumer: a separate throwaway crate depending on this repository
   by full-SHA git URL
   `https://github.com/shuyra-arche/shell.git`, then `cargo build`.
7. Anonymous checkout of the branch, repeating the `lisp` test to show no local
   path assumptions.

## Package identity and the future server dependency

- Within one checkout of this repository, Cargo resolves a dependency on `lisp`
  from a sibling member by path `../lisp`. A future in-repo `shell-server` uses
  that path dependency. It does not use a separate Git dependency on this same
  repository for its own member.
- A Git consumer of this repository follows the sibling path within the same
  Git source. It does not need to git-pin `lisp` separately.
- Avoid duplicate path and Git copies, or differing revisions, of `lisp` in one
  build graph when its public types must interoperate.
- The exact commit SHA for this provider is recorded after the integrating
  commit exists. The repository URL is
  `https://github.com/shuyra-arche/shell.git`.
- Server integration and Source intake are separate later PRs. Source/intake/
  QEMU work is deferred.

## Known failure modes

- If `lisp` is not in `members`, `cargo test -p lisp` reports it is not found
  in the workspace. No `exclude` may overlap `lisp`.
- `--locked` fails until `Cargo.lock` is regenerated and committed.
