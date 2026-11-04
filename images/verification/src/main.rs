use std::collections::BTreeSet;
use std::path::PathBuf;

fn main() {
    let mut args = std::env::args().skip(1);
    let root = PathBuf::from(args.next().expect("provider root"));
    let work = PathBuf::from(args.next().expect("build work directory"));
    let profile = match args.next().as_deref() {
        Some("debug" | "dev") => "dev",
        None | Some("release") => "release",
        _ => panic!("expected debug or release"),
    };
    let declared = shell_images_declaration::BUILDS;
    assert_eq!(declared.len(), 7);
    assert!(declared.iter().all(|build| build.profile == "release"));
    let builds: Vec<_> = declared.iter()
        .map(|build| image_builder::ImageBuild { profile, ..*build }).collect();
    let artifacts = image_builder::executables(&root, &work, &builds)
        .expect("real seven-image descriptor-driven build");
    assert_eq!(artifacts.len(), declared.len());
    let expected: BTreeSet<_> = declared.iter().map(|build| build.image).collect();
    let actual: BTreeSet<_> = artifacts.iter().map(|artifact| artifact.0).collect();
    assert_eq!(expected, actual);
    println!("{}", serde_json::json!({
        "profile": profile,
        "max_image": system_api::loader::MAX_IMAGE,
        "builds": builds.iter().map(|build| serde_json::json!({
            "image": build.image, "manifest": build.manifest,
            "package": build.package, "bin": build.bin, "target": build.target,
            "profile": build.profile, "features": build.features,
            "default_features": build.default_features,
        })).collect::<Vec<_>>(),
        "artifacts": artifacts.iter().map(|artifact| serde_json::json!({
            "image": artifact.0, "executable": artifact.1,
        })).collect::<Vec<_>>(),
    }));
}
