#![no_std]
//! Aggregated image-build declaration for the Shell image.

pub use shell_declaration::BUILD;

pub const BUILDS: &[unit::ImageBuild] = &[
    shell_declaration::BUILD,
    shell_cat_image_declaration::CAT,
    shell_command_images_declaration::EMIT,
    shell_command_images_declaration::UPPER,
    shell_command_images_declaration::FAIL,
    shell_command_images_declaration::SPIN,
    shell_command_images_declaration::WORKERS,
];
