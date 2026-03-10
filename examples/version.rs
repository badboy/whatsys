fn main() {
    println!("{:?}", whatsys::kernel_version());
    #[cfg(target_os = "windows")]
    println!("{:?}", whatsys::windows_build_number());
    #[cfg(target_os = "macos")]
    println!("{:?}", whatsys::macos_version());
}
