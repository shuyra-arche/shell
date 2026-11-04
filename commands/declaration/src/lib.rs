#![no_std]
//! Image-build declarations for the five Shell command resources in this repo.

use unit::ImageBuild;

pub const EMIT: ImageBuild = ImageBuild {
    image: "emit",
    manifest: "commands/Cargo.toml",
    package: "shell-command-images",
    bin: "emit",
    features: &[],
    target: "riscv64gc-unknown-none-elf",
    profile: "release",
    default_features: true,
};

pub const UPPER: ImageBuild = ImageBuild {
    image: "upper",
    manifest: "commands/Cargo.toml",
    package: "shell-command-images",
    bin: "upper",
    features: &[],
    target: "riscv64gc-unknown-none-elf",
    profile: "release",
    default_features: true,
};

pub const FAIL: ImageBuild = ImageBuild {
    image: "fail",
    manifest: "commands/Cargo.toml",
    package: "shell-command-images",
    bin: "fail",
    features: &[],
    target: "riscv64gc-unknown-none-elf",
    profile: "release",
    default_features: true,
};

pub const SPIN: ImageBuild = ImageBuild {
    image: "spin",
    manifest: "commands/Cargo.toml",
    package: "shell-command-images",
    bin: "spin",
    features: &[],
    target: "riscv64gc-unknown-none-elf",
    profile: "release",
    default_features: true,
};

pub const WORKERS: ImageBuild = ImageBuild {
    image: "workers",
    manifest: "commands/Cargo.toml",
    package: "shell-command-images",
    bin: "workers",
    features: &[],
    target: "riscv64gc-unknown-none-elf",
    profile: "release",
    default_features: true,
};

pub const BUILDS: &[ImageBuild] = &[EMIT, UPPER, FAIL, SPIN, WORKERS];
