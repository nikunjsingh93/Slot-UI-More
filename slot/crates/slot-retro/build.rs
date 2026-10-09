fn main() {
    println!("cargo:rerun-if-changed=src/core_log.c");
    cc::Build::new()
        .file("src/core_log.c")
        .compile("slot_core_log");
}
