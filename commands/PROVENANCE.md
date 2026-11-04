# Provenance

This `commands/` subtree of the Shell repository is a filtered extraction of
five existing Shell command resource images from the frozen Source tree. It is
not a new command implementation.

## Source coordinates

- Source repository: https://github.com/anreonyr/sqware.git
- Source revision: `94e114614bec53424ac55e640fc80cf19a3e7107`
- Sole introducing commit for the five entries:
  [`35b3d73e13f695c935665efa6d79b12265af98e7`](https://github.com/anreonyr/sqware/commit/35b3d73e13f695c935665efa6d79b12265af98e7)
- Final prefix-imported commit under `commands/`:
  `bdf4f0e7893fcd0c28fc091fe62d1cc85d8ae400`
- Historical isolated filtered archive checkpoint:
  `d3848262c01eefbe03526959a7eecfb75380eebf`
- Prepared checkpoint in isolated history:
  `16a6bb6740f9f3c3994b89d34eabfdb157a452c5`
- Final prefix-imported prepared commit:
  `8d8d39a44a7540dd76848ae404840aee343ea462`

Root imported the two old isolated commits linearly under `commands/` with
`git am --directory=commands --committer-date-is-author-date`. The final history
maps original source `35b3d73e13f695c935665efa6d79b12265af98e7` to the
prefix-imported `bdf4f0e7893fcd0c28fc091fe62d1cc85d8ae400`; the earlier isolated
filtered checkpoint `d3848262c01eefbe03526959a7eecfb75380eebf` remains a
historical archive checkpoint only. The preparation checkpoint
`16a6bb6740f9f3c3994b89d34eabfdb157a452c5` maps to prefix-imported
`8d8d39a44a7540dd76848ae404840aee343ea462`.

The import preserves the original author, dates and messages:

- Author/committer: `anreonyr <anreonyr@gmail.com>`
- Authored/committed at: `2026-10-09T18:50:20Z`
- Message: `feat(shell): integrate explicit pipes, job control and login`

No author or attribution is invented here. The machine-readable copy of this
metadata is [`provenance/source-hashes.json`](provenance/source-hashes.json).

## File mapping and original hashes

| Command | Source path | Path in `commands/` | Original SHA-256 |
| --- | --- | --- | --- |
| `emit` | `programs/src/user/emit/main.rs` | `emit/src/main.rs` | `381e351118c2a50beeb997d4f4c8b6eb9744afb4a66c5bc9db819431ef094cdb` |
| `upper` | `programs/src/user/upper/main.rs` | `upper/src/main.rs` | `c684fc4c06d2626de4e8972687d6eaaa46eddaa052f259d2a0e54d942e2be54b` |
| `fail` | `programs/src/user/fail/main.rs` | `fail/src/main.rs` | `4f359d398365b6bbb8db178fdc5c7607df7b203bf70e5e9a09e8266e1f8432d8` |
| `spin` | `programs/src/user/spin/main.rs` | `spin/src/main.rs` | `d4c3733a5d3daf0c6115e1a60498cec795e98972040e4c3acf536d4b446596c2` |
| `workers` | `programs/src/user/workers/main.rs` | `workers/src/main.rs` | `a8ed27571e312137bef13dce03ae706e3cc0e990c826206eccadc6856a0774d5` |

The paths above are relative to the nested `commands/` workspace; the
checked-in files under the public Shell prefix have the same bytes and hashes.

The referenced, but not copied, Shell declaration and composition coordinates
are recorded by hash in the same JSON file:

- `programs/src/user/shell/declaration/src/commands.rs`
  (`070ec4924376962ea4d2ccf882375ef0ee016a41d00f692074a88c79f3cd8e9e`)
- `crates/composition/src/image_builds.rs`
  (`2ee225d1870e3fcd7ce0f97dd3f9ee3bd7b431e496e528f297e78e29e2f33543`)

## Boundary adaptation

Every `main.rs` body, static, return type, resource name, port name and ordering
is otherwise byte-identical to the source. Only these two textual substitutions
were applied:

1. `extern crate programs;` -> `extern crate execution;`
2. `#[programs::entry]` -> `#[mold::entry]`

The public `execution` provider owns `_start`, the panic handler, the allocator
and `link.ld`. `commands/build.rs` consumes `DEP_EXECUTION_LINKER_SCRIPT` and
passes the published linker script with `cargo::rustc-link-arg`; it does not
carry a private linker script or runtime.

These five entries remain resource images in Shell's existing catalogue. No
`UnitFile`, lifecycle, port/catalogue struct, registration or Shell API/client
implementation was copied into `commands/`. The standalone `declaration`
package describes only `unit::ImageBuild` coordinates.

## License

No license file or license identifier is invented or added by this extraction.
No new licensing claim is made; use remains governed by the source repository's
existing terms.
