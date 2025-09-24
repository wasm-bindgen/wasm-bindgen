fn main() {
    // For `rstest` to pick up new tests.
    println!("cargo::rerun-if-changed=tests/reference");
}
