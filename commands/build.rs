fn main() {
    let ld = std::env::var("DEP_EXECUTION_LINKER_SCRIPT")
        .expect("execution did not publish DEP_EXECUTION_LINKER_SCRIPT");
    println!("cargo::rustc-link-arg=-T{ld}");
    println!("cargo::rerun-if-changed={ld}");
    println!("cargo::rerun-if-changed=emit/src/main.rs");
    println!("cargo::rerun-if-changed=upper/src/main.rs");
    println!("cargo::rerun-if-changed=fail/src/main.rs");
    println!("cargo::rerun-if-changed=spin/src/main.rs");
    println!("cargo::rerun-if-changed=workers/src/main.rs");
}
