extern crate cc;

use std::env;

fn rmain() -> Option<()> {
    // Missing? No idea what's going on.
    let target = env::var("TARGET").ok()?;
    // Not set? Definitely not windows then.
    let target_os = target.split('-').nth(2)?;

    if target_os == "windows" {
        let mut builder = cc::Build::new();
        builder.file("c/windows.c");
        builder.compile("info");
    }

    Some(())
}

fn main() {
    let _ = rmain();
}
