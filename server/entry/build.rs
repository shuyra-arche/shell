//! Passes the execution link script to the entry binary.

fn main() {
    let linker_script = std::env::var("DEP_EXECUTION_LINKER_SCRIPT")
        .expect("execution build metadata DEP_EXECUTION_LINKER_SCRIPT is missing");
    println!("cargo::rustc-link-arg-bin=prog-shell=-T{linker_script}");
    println!("cargo::rerun-if-changed={linker_script}");
    println!("cargo::rerun-if-changed=src/main.rs");
}
