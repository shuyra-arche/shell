# Shell interfaces

Interface-only extraction from `anreonyr/sqware` at
`94e114614bec53424ac55e640fc80cf19a3e7107`. This repository contains exactly
three crates:

- `api` (`shell-api`): the `Launch` and `Catalogue` wire codecs.
- `declaration` (`shell-declaration`): host-safe command and program declarations.
- `client` (`shell-client`): startup `Boot` and pipe copy helpers.

Server, Lisp, the six command implementations and their program runtime remain in
Source for later slices. This slice is interface availability, not runtime
independence or integration.

## Build and test

Locked host default test command (workspace default members are `api` and `declaration`):

```sh
cargo test --locked
```

Locked RISC-V build of all three crates:

```sh
cargo build --locked --workspace --target riscv64gc-unknown-none-elf
```

`rust-toolchain.toml` pins `nightly-2026-09-04` with the minimal profile,
`rust-src`, and `riscv64gc-unknown-none-elf`. The committed `Cargo.lock` fixes
the dependency graph. The imported contract test and the added wire and
declaration tests are the available host verification for this slice; they do
not establish runtime independence or integration acceptance.

## Frozen interface

- Wire `VERSION = 3` and `MAGIC = 0x7371776172656c73` (little-endian bytes
  `73 6c 65 72 61 77 71 73`). Codec layout and production behavior are unchanged.
- Known encode/decode asymmetries are preserved and pinned by
  `api/tests/wire.rs`, not repaired here.
- Dependencies remain full-SHA HTTPS GitHub pins:
  - `env` `d3000253cd5d13d466e91fafd4dbb27a5ec5aaae`
  - `pipe` `4fce76372beb6c5ab8567232516f156b3d1032eb`
  - `resource` `dbc96834f50f094b5c386e54a0a2ae5560dcd7f8`
  - `execution` `1b333a28bc4c34beaea3f7986a9bb6e8ab2db414`
  - `unit` `90396f0fda376e0693fb1b9553aa21864d241057`
  - `system` `9e5d63ca536a176583cf1b0054b4ecf7ddd88257`
- The only intra-repository dependency is `shell-client -> ../api`.
- No `source/sqware` path, local Git dependency, patch override, renamed API,
  changed codec, or production behavior change is part of this slice.

## Startup and client behavior

`Boot::take` consumes the first boot argument as the owner task id. It scans
`resource::raw::pies()` for a live pole owned and vested by that task with
`shell_api::MARK`, opens and retains the matching `Dock` for the view lifetime,
decodes the little-endian `u64` length followed by the `Launch` payload, imports
each declared endpoint as a `Port`, shuts the dock, and then ignores the result of
`env::pie::release(..., env::ReleaseMode::Revoke)` before returning args and
ports. Failures map to `()`.

This slice adds no transactional rollback. On an error after the dock is opened
or ports are imported, the early return performs no explicit rollback; cleanup
remains whatever the retained `Dock` and imported `Port` ownership already
provides, not an improved atomic unwind.

`shell_client::copy` reads with `env::Wait::Forever`; `Read::Pending` and
`Write::Pending` continue the loop, partial writes are retried from the unwritten
suffix by `write_all`, and EOF closes the write port before the read port.
Unexpected write outcomes, including zero-length progress, fail.

## Deferred boundaries

The six commands can move into this same repository once their shared
image/runtime support is independent; that migration does not require Lisp.
Lisp is instead a separate prerequisite for migrating the current REPL,
native-host, and server side, including Shell service adapters. `programs::entry`,
allocator, panic, and startup support also need independently reviewed slices.
`cat` additionally depends on Terminal and System identity; `workers` depends on
Execution and atomics. No new Lisp crate or repository is assumed.

Vanllia #3 composition remains pinned to the `sqware` `shell-declaration`. This
change does not modify Source or Vanllia.

## Future intake order

1. Publish and review the interface at a full SHA.
2. Pin host composition separately.
3. Move the six commands into this repository after their shared image/runtime
   support is independent; this step does not require Lisp.
4. Migrate the REPL/native-host/server side separately after isolating Lisp and
   its runtime support.
5. Avoid duplicate public API type identities while moving.
