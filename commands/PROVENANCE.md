# Provenance

This repository is a filtered extraction of five existing Shell command resource
images from the frozen Source tree. It is not a new command implementation.

## Source coordinates

- Source repository: https://github.com/anreonyr/sqware.git
- Source revision: `94e114614bec53424ac55e640fc80cf19a3e7107`
- Sole introducing commit for the five entries:
  [`35b3d73e13f695c935665efa6d79b12265af98e7`](https://github.com/anreonyr/sqware/commit/35b3d73e13f695c935665efa6d79b12265af98e7)
- Filtered commit in this repository:
  `d3848262c01eefbe03526959a7eecfb75380eebf`
- History map (one nonzero mapping):
  `35b3d73e13f695c935665efa6d79b12265af98e7` -> `d3848262c01eefbe03526959a7eecfb75380eebf`

The filtered commit preserves the original author, dates and message:

- Author/committer: `anreonyr <anreonyr@gmail.com>`
- Authored/committed at: `2026-10-09T18:50:20Z`
- Message: `feat(shell): integrate explicit pipes, job control and login`

No author or attribution is invented here. The machine-readable copy of this
metadata is [`provenance/source-hashes.json`](provenance/source-hashes.json).

## File mapping and original hashes

| Command | Source path | Path in this repo | Original SHA-256 |
| --- | --- | --- | --- |
| `emit` | `programs/src/user/emit/main.rs` | `emit/src/main.rs` | `381e351118c2a50beeb997d4f4c8b6eb9744afb4a66c5bc9db819431ef094cdb` |
| `upper` | `programs/src/user/upper/main.rs` | `upper/src/main.rs` | `c684fc4c06d2626de4e8972687d6eaaa46eddaa052f259d2a0e54d942e2be54b` |
| `fail` | `programs/src/user/fail/main.rs` | `fail/src/main.rs` | `4f359d398365b6bbb8db178fdc5c7607df7b203bf70e5e9a09e8266e1f8432d8` |
| `spin` | `programs/src/user/spin/main.rs` | `spin/src/main.rs` | `d4c3733a5d3daf0c6115e1a60498cec795e98972040e4c3acf536d4b446596c2` |
| `workers` | `programs/src/user/workers/main.rs` | `workers/src/main.rs` | `a8ed27571e312137bef13dce03ae706e3cc0e990c826206eccadc6856a0774d5` |

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
and `link.ld`. The root `build.rs` consumes `DEP_EXECUTION_LINKER_SCRIPT` and
passes the published linker script with `cargo::rustc-link-arg`; it does not
carry a private linker script or runtime.

These five entries remain resource images in Shell's existing catalogue. No
`UnitFile`, lifecycle, port/catalogue struct, registration or Shell API/client
implementation was copied into this repository. The standalone
`declaration` package describes only `unit::ImageBuild` coordinates.

## License

No license file or license identifier is invented or added by this extraction.
No new licensing claim is made; use remains governed by the source repository's
existing terms.
