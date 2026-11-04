use std::path::PathBuf;

fn main() {
    let mut args = std::env::args().skip(1);
    let root = PathBuf::from(args.next().expect("provider root"));
    let work = PathBuf::from(args.next().expect("work directory"));
    let profile = match args.next().as_deref() {
        Some("debug" | "dev") => "dev",
        None | Some("release") => "release",
        _ => panic!("expected debug or release"),
    };
    let declared = shell_declaration::BUILD;
    assert_eq!(declared.image, shell_declaration::PROGRAM.loader.image);
    let build = image_builder::ImageBuild { profile, ..declared };
    let artifacts = image_builder::executables(&root, &work, &[build])
        .expect("real ImageBuild-driven Cargo executable");
    assert_eq!(artifacts.len(), 1);
    assert_eq!(artifacts[0].0, build.image);
    println!("{}", serde_json::json!({
        "image": build.image, "manifest": build.manifest,
        "package": build.package, "bin": build.bin, "target": build.target,
        "declared_profile": declared.profile, "built_profile": build.profile,
        "default_features": build.default_features, "features": build.features,
        "executable": artifacts[0].1, "max_image": system_api::loader::MAX_IMAGE,
    }));
}
