use crate::unit;

pub const BUILD: unit::ImageBuild = unit::ImageBuild {
    image: crate::PROGRAM.loader.image,
    manifest: "server/entry/Cargo.toml",
    package: "shell-entry",
    bin: "prog-shell",
    features: &[],
    target: "riscv64gc-unknown-none-elf",
    profile: "release",
    default_features: true,
};
