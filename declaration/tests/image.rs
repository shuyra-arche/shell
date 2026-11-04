use shell_declaration::{BUILD, PROGRAM};

#[test]
fn build_declares_the_shell_program_image() {
    assert_eq!(BUILD.image, PROGRAM.loader.image);
    assert_eq!(BUILD.manifest, "server/entry/Cargo.toml");
    assert_eq!(BUILD.package, "shell-entry");
    assert_eq!(BUILD.bin, "prog-shell");
    assert_eq!(BUILD.target, "riscv64gc-unknown-none-elf");
    assert_eq!(BUILD.profile, "release");
    assert!(BUILD.features.is_empty());
    assert!(BUILD.default_features);

    let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join(BUILD.manifest);
    let canonical = manifest
        .canonicalize()
        .expect("declared manifest path resolves");
    assert!(canonical.is_file(), "missing declared manifest: {}", canonical.display());
}
