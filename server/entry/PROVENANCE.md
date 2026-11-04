# Source and ownership

The entry body comes from `programs/src/user/shell/main.rs` at Source baseline
`94e114614bec53424ac55e640fc80cf19a3e7107`. Its genuine path history was
filtered into `server/entry/src/main.rs` and overlaid linearly on Shell server
PR #4 (`7a2f99e268784693975b5e244b28795be50c218c`), retaining the original
author, committer, dates and message:

| Source commit | Shell imported commit |
| --- | --- |
| `35b3d73e13f695c935665efa6d79b12265af98e7` | `57be49f24cbbcf9bfbd3c675d0b1c3fc553367a9` |

Git copy detection can follow this ten-line template to Account boilerplate;
that unrelated unit history was not imported. The later adaptation changes
only four paths in the original entry body. `programs::entry` originally
reexported `mold::entry`, and `programs::debug` reexported `system_support::debug`.
The same `run()` result still prints the diagnostic and returns reason `86`.

The entry owns executable linking and the `run()` call. Execution owns `_start`,
the allocator, panic handler, entry lifecycle, and link script. System support
owns debug-channel output. Shell server owns parsing, launch, job state, Lisp
native operations and the REPL. Lisp has no reverse dependency on the server.
The declaration's existing `PROGRAM` remains the unit identity and catalogue
authority; `BUILD` only adds image build coordinates and reads its image name
from `PROGRAM`. No second unit descriptor, parser, runtime or command binding
is introduced. Link-metadata absence fails the build explicitly.

The entry's direct runtime pins are in `Cargo.toml`; its additional transitive
dependencies, all matching the Source baseline lock, are:

| Package | Provider | Revision or version |
| --- | --- | --- |
| `system-support` | `shuyra-arche/system` | `5a792ff2d39901a970925b1a6d63206adf6c16d8` |
| `hub-api` | `shuyra-arche/hub` | `cccef3f325198c8390007ae88b064bae0f21f93e` |
| `fdt` | crates.io | `0.1.5` (checksum in `server/Cargo.lock`) |

`hub-api` belongs to System support's dependency closure. It is unrelated to
the command-image workspaces in PR #2 or #5.

New PR group `shell-entry-image-20261104` uses genuine DeepSeek
`deepseek-v4.1-flash` namer, worker and reviewer sessions through the configured
OpenCode Go API. They inspect supplied source and executor evidence; they do
not execute commands or access files themselves. This is not a claim of an
interactive dsh CLI invocation. The worker supplies production changes and
corrections; the executor applies them, performs builds and records the evidence.
Role receipts and reviewed code hashes are recorded in `review.json`.

Command PR #2 (`effeb09c170f0259c3d79687754b02aa41f615fe`) and Cat PR #5
(`df651c37d99f15e26cf7d76a24f0a63a4b58df58`) are separate resource-image slices.
This branch starts from PR #4 and changes neither resource workspace. Combining
them later must retain the root RISC-V build flags, the original six bindings,
and the single Shell API identity. Source and Vanllia intake are separate work.
