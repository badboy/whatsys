use std::env;

fn rmain() -> Option<()> {
    let target_os = env::var("CARGO_CFG_TARGET_OS").ok()?;
    if target_os == "windows" && env::var("CARGO_FEATURE_MANIFEST").is_ok() {
        embed_resource::compile("windows-example-manifest.rc");
    }

    Some(())
}

fn main() {
    let _ = rmain();
}
