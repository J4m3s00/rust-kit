use std::process::Command;

fn main () {
    Command::new("cmake")
    .args(["-S", "../user_interface/", "-B", "../user_interface/build/"])
    .output()
    .expect("Failed to execute cmake");
    println!("cargo:rerun-if-changed=../user_interface/src/main.c");
}