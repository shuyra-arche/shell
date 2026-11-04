use std::path::{Path, PathBuf};

use shell_cat_image_declaration::CAT;
use shell_command_images_declaration::{EMIT, FAIL, SPIN, UPPER, WORKERS};
use shell_declaration::{BUILD, COMMANDS};
use shell_images_declaration::BUILDS;
use unit::ImageBuild;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("images package must live under the repository root")
        .to_path_buf()
}

fn assert_same_build(actual: &ImageBuild, expected: &ImageBuild) {
    assert_eq!(actual.image, expected.image);
    assert_eq!(actual.manifest, expected.manifest);
    assert_eq!(actual.package, expected.package);
    assert_eq!(actual.bin, expected.bin);
    assert_eq!(actual.features, expected.features);
    assert_eq!(actual.target, expected.target);
    assert_eq!(actual.profile, expected.profile);
    assert_eq!(actual.default_features, expected.default_features);
}

#[test]
fn aggregate_matches_existing_constants_field_by_field() {
    assert_eq!(BUILDS.len(), 7);
    assert_same_build(&BUILDS[0], &BUILD);
    assert_same_build(&BUILDS[1], &CAT);
    assert_same_build(&BUILDS[2], &EMIT);
    assert_same_build(&BUILDS[3], &UPPER);
    assert_same_build(&BUILDS[4], &FAIL);
    assert_same_build(&BUILDS[5], &SPIN);
    assert_same_build(&BUILDS[6], &WORKERS);
}

#[test]
fn aggregate_order_is_canonical() {
    let names: Vec<&str> = BUILDS.iter().map(|build| build.image).collect();
    assert_eq!(names, ["shell", "cat", "emit", "upper", "fail", "spin", "workers"]);
}

#[test]
fn image_identities_are_unique() {
    let mut names: Vec<&str> = BUILDS.iter().map(|build| build.image).collect();
    names.sort_unstable();
    let count = names.len();
    names.dedup();
    assert_eq!(names.len(), count, "image identities must be unique");
}

#[test]
fn six_command_resources_correspond_to_existing_commands() {
    let command_builds = &BUILDS[1..];
    assert_eq!(command_builds.len(), COMMANDS.len());
    for (build, command) in command_builds.iter().zip(COMMANDS.iter()) {
        assert_eq!(build.image, command.name);
    }
}

#[test]
fn declared_manifests_resolve_from_repository_root() {
    let root = repo_root();
    for build in BUILDS {
        let manifest = root.join(build.manifest);
        assert!(
            manifest.is_file(),
            "declared manifest does not resolve: {}",
            manifest.display()
        );
    }
}
