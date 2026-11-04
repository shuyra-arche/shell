# Provenance

- Repository: `shuyra-arche/shell`, base
  `f4ab6a2dc3924959ac5096305b582cfb479569bd`. The historical candidate
  checkpoint is `27aaf941597ddfa3e512178acd7285e445de76fb`; the imported tip
  in this repo is `db0a67a20264e60842d0b00fde1c1c8548fa5508` under prefix
  `cat/`.
- Original source:
  `https://github.com/anreonyr/sqware.git` @
  `94e114614bec53424ac55e640fc80cf19a3e7107`, path
  `programs/src/user/cat/main.rs`, sha256
  `d0e6fecf9c281245fc27412579bcdb36406dd4c43f8cdcb96018ba6fdc041aca`,
  imported as `cat/command/src/main.rs`.
- Import state per `provenance/import-history.json`: all candidate files
  byte-equal; all base files unchanged (23 tracked); authors,
  author/committer dates, and messages preserved. The report maps the seven
  original commits plus the candidate checkpoint (eight commits total) to
  their imported Shell commit IDs.
- Frozen code/config snapshot sha256
  `2144aea5c14b3313fef496d92aecdbdf5417fc1607b632a0161e93e7d2f4fc1d`,
  recorded with per-file hashes in `provenance/validated-code.json`.
- Runtime and the 23 host tests are byte-identical to the prior candidate;
  root SDK files are identical to Shell main. `shell-api`/`shell-client`
  `b910582fc9dc0fd732f279d94f2cfca89f4c8c8f` is retained verbatim.
- Cat lives in this existing Shell repository by the user's explicit choice.
  The earlier denied standalone-repository creation (403) is a historical
  note only and will never be retried.
- PR2 `effeb09c170f0259c3d79687754b02aa41f615fe` is unchanged; its compare is
  all `commands/`, zero `cat/`. Lisp PR3
  `b72bbfca6455159a9bf9a0b250877b7e660e8c52` (branch
  `codex/shell-lisp-library`, based on main
  `f4ab6a2dc3924959ac5096305b582cfb479569bd`) is public with that exact head;
  its compare changes root `Cargo.toml`/`Cargo.lock`/`README` plus `docs/lisp*`
  and `lisp/`, zero `cat/` paths. No published server branch or PR was
  observed. Neither PR nor the server is a dependency; all Cat writes stay
  under `cat/`, leaving root workspace/SDK, `commands/`, `lisp/`, and server
  untouched. Work remains on private branch `codex/shell-cat-image` from main
  `f4ab6a2d…`; heads must be rechecked before publication.

Evidence: `provenance/coordination.json`, `provenance/import-history.json`,
`provenance/validation.json`, `provenance/dependency-and-elf.json`,
`provenance/validated-code.json`.
