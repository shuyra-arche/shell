# Shell image declarations

`shell-images-declaration` exports Shell, Cat, Emit, Upper, Fail, Spin and
Workers builds in that order. Coordinates are owned by `../declaration`,
`../cat/declaration` and `../commands/declaration`; this package aggregates them.
Builders receive the repository root so provider-relative manifests and root
RISC-V flags apply. Shell SDK dependencies resolve through sibling paths.

```sh
cargo test --locked --manifest-path images/Cargo.toml --test builds --target x86_64-unknown-linux-gnu
```

See the [root README](../README.md) for prerequisites and executable builds.
The historical `audit.py` verifies the fixed consolidation checkpoint; it is
not a general current-tree validation command.
