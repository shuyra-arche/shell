use shell_command_images_declaration::{BUILDS, EMIT, FAIL, SPIN, UPPER, WORKERS};

const NAMES: [&str; 5] = ["emit", "upper", "fail", "spin", "workers"];

#[test]
fn builds_are_exactly_the_five_ordered_resource_images() {
    assert_eq!(BUILDS.len(), NAMES.len());
    for (build, name) in BUILDS.iter().zip(NAMES) {
        assert_eq!(build.image, name);
        assert_eq!(build.bin, name);
        assert_eq!(build.manifest, "Cargo.toml");
        assert_eq!(build.package, "shell-command-images");
        assert!(build.features.is_empty());
        assert_eq!(build.target, "riscv64gc-unknown-none-elf");
        assert_eq!(build.profile, "release");
        assert!(build.default_features);
    }
}

#[test]
fn exported_constants_match_the_ordered_slice() {
    let constants = [EMIT, UPPER, FAIL, SPIN, WORKERS];
    assert_eq!(constants.len(), BUILDS.len());
    for (constant, build) in constants.iter().zip(BUILDS) {
        assert_eq!(constant.image, build.image);
        assert_eq!(constant.bin, build.bin);
        assert_eq!(constant.manifest, build.manifest);
        assert_eq!(constant.package, build.package);
        assert_eq!(constant.features, build.features);
        assert_eq!(constant.target, build.target);
        assert_eq!(constant.profile, build.profile);
        assert_eq!(constant.default_features, build.default_features);
    }
}

#[test]
fn coordinates_are_unique_and_exclude_cat_and_shell() {
    let mut images: [&str; 5] = [""; 5];
    let mut bins: [&str; 5] = [""; 5];
    for (index, build) in BUILDS.iter().enumerate() {
        images[index] = build.image;
        bins[index] = build.bin;
    }
    images.sort_unstable();
    bins.sort_unstable();
    for pair in images.windows(2) {
        assert_ne!(pair[0], pair[1]);
    }
    for pair in bins.windows(2) {
        assert_ne!(pair[0], pair[1]);
    }
    assert!(BUILDS.iter().all(|build| build.image != "cat"));
    assert!(BUILDS.iter().all(|build| build.image != "shell"));
    assert!(BUILDS.iter().all(|build| build.bin != "cat"));
    assert!(BUILDS.iter().all(|build| build.bin != "shell"));
}
