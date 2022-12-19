#[cfg(target_os = "windows")]
fn main() {
    let kernel_version = whatsys::kernel_version().unwrap();
    let build_number = whatsys::windows_build_number().unwrap();

    println!("Kernel: {}", kernel_version);
    println!("Build Number: {}", build_number);

    #[cfg(feature = "manifest")]
    {
        assert_eq!("10.0", kernel_version);
        assert!(build_number >= 20000, "Expected 20000+, got {build_number}");
    }

    #[cfg(not(feature = "manifest"))]
    {
        assert_eq!("6.2", kernel_version);
        assert!(build_number < 20000, "Expected <20000, got {build_number}");
    }
}

#[cfg(not(target_os = "windows"))]
fn main() {
    eprintln!("Windows-only example");
}
