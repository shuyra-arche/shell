#![no_std]
//! Image-build declaration for the Cat resource in this repo.

use unit::ImageBuild;

pub const CAT: ImageBuild = ImageBuild {
    image: "cat",
    manifest: "cat/command/Cargo.toml",
    package: "cat",
    bin: "prog-cat",
    features: &[],
    target: "riscv64gc-unknown-none-elf",
    profile: "release",
    default_features: true,
};

pub const BUILDS: &[ImageBuild] = &[CAT];
