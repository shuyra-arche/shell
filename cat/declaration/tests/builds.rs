use std::path::{Path, PathBuf};

use shell_cat_image_declaration::{BUILDS, CAT};

fn repository_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("cat/declaration sits two levels below the repository root")
        .to_path_buf()
}

fn section_names(manifest: &str, header: &str) -> Vec<String> {
    let mut names = Vec::new();
    let mut inside = false;
    for line in manifest.lines() {
        let line = line.trim();
        if line.starts_with('[') {
            inside = line == header;
            continue;
        }
        if !inside {
            continue;
        }
        if let Some(rest) = line.strip_prefix("name") {
            let value = rest.trim_start().strip_prefix('=').unwrap_or("").trim();
            names.push(value.trim_matches('"').to_string());
        }
    }
    names
}

#[test]
fn builds_are_exactly_the_cat_image() {
    assert_eq!(BUILDS.len(), 1);
    for build in BUILDS {
        assert_eq!(build.image, "cat");
        assert_eq!(build.manifest, "cat/command/Cargo.toml");
        assert_eq!(build.package, "cat");
        assert_eq!(build.bin, "prog-cat");
        assert!(build.features.is_empty());
        assert_eq!(build.target, "riscv64gc-unknown-none-elf");
        assert_eq!(build.profile, "release");
        assert!(build.default_features);
    }
    assert_eq!(CAT.image, BUILDS[0].image);
    assert_eq!(CAT.manifest, BUILDS[0].manifest);
    assert_eq!(CAT.package, BUILDS[0].package);
    assert_eq!(CAT.bin, BUILDS[0].bin);
    assert_eq!(CAT.features, BUILDS[0].features);
    assert_eq!(CAT.target, BUILDS[0].target);
    assert_eq!(CAT.profile, BUILDS[0].profile);
    assert_eq!(CAT.default_features, BUILDS[0].default_features);
}

#[test]
fn manifest_resolves_to_the_cat_package_under_the_repository_root() {
    let manifest_path = repository_root().join(CAT.manifest);
    let manifest = std::fs::read_to_string(&manifest_path)
        .unwrap_or_else(|error| panic!("cannot read {}: {error}", manifest_path.display()));
    assert_eq!(section_names(&manifest, "[package]"), [CAT.package]);
    assert_eq!(section_names(&manifest, "[[bin]]"), [CAT.bin]);
    let package_root = manifest_path
        .parent()
        .expect("the declared manifest path has a package directory");
    assert!(package_root.join("src/lib.rs").is_file());
    assert!(package_root.join("src/main.rs").is_file());
}
