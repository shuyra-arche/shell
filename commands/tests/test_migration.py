#!/usr/bin/env python3
"""Portable source-preservation and final-manifest contract for this extraction.

This check does not import or emulate Shell. It reverses only the two entry-point
substitutions applied at the extraction boundary, then compares the reconstructed
bytes with the recorded original SHA-256 values. A fail-closed template branch is
retained for historical preparation, but the final-state test requires the
materialized manifest and the actual immutable Shell revision.

The runtime manifest now expects the in-repository Shell provider paths. The
original revision pin is preserved only as factual extraction metadata and is
checked by a separate historical-SDK test.
"""

from __future__ import annotations

import hashlib
import json
import tomllib
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
NAMES = ("emit", "upper", "fail", "spin", "workers")

SOURCE_REVISION = "94e114614bec53424ac55e640fc80cf19a3e7107"
SOURCE_COMMIT = "35b3d73e13f695c935665efa6d79b12265af98e7"
FILTERED_COMMIT = "bdf4f0e7893fcd0c28fc091fe62d1cc85d8ae400"
ISOLATED_FILTERED_COMMIT = "d3848262c01eefbe03526959a7eecfb75380eebf"
PREPARED_ARCHIVE_COMMIT = "16a6bb6740f9f3c3994b89d34eabfdb157a452c5"
PREPARED_IMPORTED_COMMIT = "8d8d39a44a7540dd76848ae404840aee343ea462"
SOURCE_PATHS = {
    "emit": "programs/src/user/emit/main.rs",
    "upper": "programs/src/user/upper/main.rs",
    "fail": "programs/src/user/fail/main.rs",
    "spin": "programs/src/user/spin/main.rs",
    "workers": "programs/src/user/workers/main.rs",
}

EXECUTION_ENTRY = b"extern crate execution;"
PROGRAMS_ENTRY = b"extern crate programs;"
MOLD_ATTRIBUTE = b"#[mold::entry]"
PROGRAMS_ATTRIBUTE = b"#[programs::entry]"

MARKER = "@SHELL_PROVIDER_REV@"
SHELL_URL = "https://github.com/shuyra-arche/shell.git"
SHELL_REVISION = "b910582fc9dc0fd732f279d94f2cfca89f4c8c8f"
PINNED = {
    "pipe-client": (
        "https://github.com/shuyra-arche/pipe.git",
        "4fce76372beb6c5ab8567232516f156b3d1032eb",
    ),
    "env": (
        "https://github.com/shuyra-arche/env.git",
        "d3000253cd5d13d466e91fafd4dbb27a5ec5aaae",
    ),
    "execution": (
        "https://github.com/shuyra-arche/execution.git",
        "1b333a28bc4c34beaea3f7986a9bb6e8ab2db414",
    ),
    "mold": (
        "https://github.com/shuyra-arche/mold.git",
        "f52501f4c91b686eded8b4bcd743b4e96084b7f9",
    ),
}
EXPECTED_DEPENDENCIES = {"shell-client", *PINNED}


def load_toml(path: Path) -> dict:
    with path.open("rb") as handle:
        return tomllib.load(handle)


class MigrationContract(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        cls.provenance = json.loads(
            (ROOT / "provenance" / "source-hashes.json").read_text(encoding="utf-8")
        )

    def test_provenance_names_paths_and_hashes_are_complete(self) -> None:
        self.assertEqual(self.provenance["source_revision"], SOURCE_REVISION)
        history = self.provenance["history"]
        self.assertEqual(history["source_commit"], SOURCE_COMMIT)
        self.assertEqual(history["filtered_commit"], FILTERED_COMMIT)
        self.assertEqual(history["mapping"], {SOURCE_COMMIT: FILTERED_COMMIT})
        self.assertEqual(
            history["isolated_filtered_commit"], ISOLATED_FILTERED_COMMIT
        )
        self.assertEqual(
            history["isolated_mapping"],
            {SOURCE_COMMIT: ISOLATED_FILTERED_COMMIT},
        )
        self.assertEqual(history["prepared_archive_commit"], PREPARED_ARCHIVE_COMMIT)
        self.assertEqual(
            history["prepared_imported_commit"], PREPARED_IMPORTED_COMMIT
        )
        self.assertEqual(
            history["prepared_mapping"],
            {PREPARED_ARCHIVE_COMMIT: PREPARED_IMPORTED_COMMIT},
        )
        self.assertEqual(history["author"]["name"], "anreonyr")
        self.assertEqual(history["author"]["email"], "anreonyr@gmail.com")
        self.assertEqual(history["authored_at"], "2026-10-09T18:50:20Z")
        self.assertEqual(
            history["message"],
            "feat(shell): integrate explicit pipes, job control and login",
        )

        files = self.provenance["files"]
        self.assertEqual([entry["name"] for entry in files], list(NAMES))
        for entry in files:
            self.assertEqual(entry["source"], SOURCE_PATHS[entry["name"]])
            self.assertEqual(entry["path"], f"{entry['name']}/src/main.rs")
            self.assertRegex(entry["sha256"], r"\A[0-9a-f]{64}\Z")

    def test_source_bytes_reverse_only_the_entry_substitutions(self) -> None:
        for entry in self.provenance["files"]:
            with self.subTest(name=entry["name"]):
                current = (ROOT / entry["path"]).read_bytes()
                self.assertEqual(current.count(EXECUTION_ENTRY), 1)
                self.assertEqual(current.count(MOLD_ATTRIBUTE), 1)
                self.assertNotIn(PROGRAMS_ENTRY, current)
                self.assertNotIn(PROGRAMS_ATTRIBUTE, current)

                original = current.replace(EXECUTION_ENTRY, PROGRAMS_ENTRY, 1).replace(
                    MOLD_ATTRIBUTE, PROGRAMS_ATTRIBUTE, 1
                )
                digest = hashlib.sha256(original).hexdigest()
                self.assertEqual(
                    digest,
                    entry["sha256"],
                    f"{entry['path']} does not reconstruct to its original bytes",
                )

    def test_build_script_registers_all_five_bins_and_execution_linker_only(self) -> None:
        text = (ROOT / "build.rs").read_text(encoding="utf-8")
        self.assertEqual(
            text.count('std::env::var("DEP_EXECUTION_LINKER_SCRIPT")'), 1
        )
        self.assertEqual(text.count("std::env::var"), 1)
        self.assertEqual(text.count("cargo::rustc-link-arg=-T{ld}"), 1)
        self.assertEqual(text.count("rustc-link-arg"), 1)
        self.assertEqual(text.count("-T"), 1)
        self.assertNotIn("rustc-link-search", text)
        self.assertNotIn("link.ld", text)

        expected_reruns = [
            'println!("cargo::rerun-if-changed={ld}");',
            *[
                f'println!("cargo::rerun-if-changed={name}/src/main.rs");'
                for name in NAMES
            ],
        ]
        actual_reruns = [
            line.strip() for line in text.splitlines() if "rerun-if-changed" in line
        ]
        self.assertEqual(actual_reruns, expected_reruns)

    def _runtime_manifest(self) -> tuple[dict, bool]:
        template = ROOT / "Cargo.toml.in"
        materialized = ROOT / "Cargo.toml"

        if template.exists():
            text = template.read_text(encoding="utf-8")
            self.assertEqual(
                text.count(MARKER),
                1,
                "the pending template must expose exactly one Shell revision marker",
            )
            self.assertFalse(
                materialized.exists(),
                "Cargo.toml must not exist while the template is pending",
            )
            return tomllib.loads(text), True

        self.assertTrue(
            materialized.exists(),
            "expected Cargo.toml.in or a finalized Cargo.toml",
        )
        text = materialized.read_text(encoding="utf-8")
        self.assertNotIn(MARKER, text)
        return tomllib.loads(text), False

    def test_historical_sdk_pin_is_recorded(self) -> None:
        data = load_toml(ROOT / "provenance" / "sdk-at-extraction.toml")
        self.assertEqual(
            data,
            {"git": SHELL_URL, "rev": SHELL_REVISION},
            "historical SDK metadata must record the original Shell pin",
        )

    def test_runtime_manifest_coordinates_and_immutable_pins(self) -> None:
        data, pending = self._runtime_manifest()
        self.assertFalse(
            pending,
            "final state requires materialized commands/Cargo.toml with no marker",
        )

        workspace = data["workspace"]
        self.assertEqual(workspace["resolver"], "3")
        self.assertEqual(workspace["exclude"], ["declaration"])

        package = data["package"]
        self.assertEqual(package["name"], "shell-command-images")
        self.assertEqual(package["version"], "0.1.0")
        self.assertEqual(package["edition"], "2024")
        self.assertIs(package["publish"], False)
        self.assertIs(package["autobins"], False)
        self.assertIs(package["autotests"], False)
        self.assertNotIn("lib", data)
        for section in ("patch", "replace"):
            self.assertNotIn(section, data)

        bins = data["bin"]
        self.assertEqual([binary["name"] for binary in bins], list(NAMES))
        for binary, name in zip(bins, NAMES):
            self.assertEqual(binary["path"], f"{name}/src/main.rs")
            self.assertIs(binary["test"], False)
            self.assertIs(binary["bench"], False)
            self.assertIs(binary["doctest"], False)

        dependencies = data["dependencies"]
        self.assertEqual(set(dependencies), EXPECTED_DEPENDENCIES)
        self.assertEqual(dependencies["shell-client"], {"path": "../client"})

        for name, (url, revision) in PINNED.items():
            self.assertEqual(dependencies[name], {"git": url, "rev": revision})

        self.assertNotIn("build-dependencies", data)
        self.assertNotIn("dev-dependencies", data)
        for profile in ("dev", "release"):
            self.assertEqual(data["profile"][profile]["panic"], "abort")

    def test_toolchain_and_target_flags_are_preserved_without_a_runner(self) -> None:
        toolchain = load_toml(ROOT / "rust-toolchain.toml")["toolchain"]
        self.assertEqual(toolchain["channel"], "nightly-2026-09-04")
        self.assertEqual(
            set(toolchain["components"]), {"rust-src", "rustfmt", "clippy"}
        )
        self.assertIn("riscv64gc-unknown-none-elf", toolchain["targets"])

        config = load_toml(ROOT / ".cargo" / "config.toml")
        target = config["target"]["riscv64gc-unknown-none-elf"]
        self.assertEqual(
            target["rustflags"],
            [
                "-Crelocation-model=static",
                "-Cforce-frame-pointers=yes",
                "-Ccode-model=medium",
            ],
        )
        self.assertNotIn("runner", target)

    def test_declaration_is_standalone_and_uses_one_public_unit_pin(self) -> None:
        data = load_toml(ROOT / "declaration" / "Cargo.toml")
        self.assertEqual(data["workspace"]["resolver"], "3")

        package = data["package"]
        self.assertEqual(package["name"], "shell-command-images-declaration")
        self.assertEqual(package["version"], "0.1.0")
        self.assertEqual(package["edition"], "2024")
        self.assertIs(package["publish"], False)
        for section in ("patch", "replace"):
            self.assertNotIn(section, data)

        dependencies = data["dependencies"]
        self.assertEqual(set(dependencies), {"unit"})
        self.assertEqual(
            dependencies["unit"],
            {
                "git": "https://github.com/shuyra-arche/unit.git",
                "rev": "90396f0fda376e0693fb1b9553aa21864d241057",
            },
        )

        declaration = (ROOT / "declaration" / "src" / "lib.rs").read_text(
            encoding="utf-8"
        )
        for forbidden in ("UnitFile", "COMMANDS", "BINDINGS", "Command", "ports"):
            self.assertNotIn(forbidden, declaration)


if __name__ == "__main__":
    unittest.main()
