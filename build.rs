// Copyright (c) 2026 Greg, l1acy

fn main() {
    let token = std::env::var("BRAIN_CRAB_TOKEN")
        .expect("переменная BRAIN_CRAB_TOKEN должна быть задана при сборке");

    println!("cargo:rustc-env=BRAIN_CRAB_TOKEN={token}");
    println!("cargo:rerun-if-env-changed=BRAIN_CRAB_TOKEN");
}
