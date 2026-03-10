/* Based on code from sysinfo: https://crates.io/crates/sysinfo
 * Original licenses: MIT
 * Original author: Guillaume Gomez
 * License file: https://github.com/GuillaumeGomez/sysinfo/blob/master/LICENSE
 */

fn get_system_info(name: &[u8]) -> Option<String> {
    let mut size = 0;

    // Call first to get size
    //
    // SAFETY:
    // * It's a syscall
    // * We control the input. `name` is guaranteed to be a non-null slice.
    let string_buf = unsafe {
        // Retrieve the size of the string (including null terminator)
        if libc::sysctlbyname(
            name.as_ptr().cast(),
            std::ptr::null_mut(),
            &mut size,
            std::ptr::null_mut(),
            0,
        ) != 0
            || size <= 1
        {
            // exit early if we did not update the size
            return None;
        }

        let mut buff = Vec::new();
        buff.resize(size, 0);

        if libc::sysctlbyname(
            name.as_ptr().cast(),
            buff.as_mut_ptr().cast(),
            &mut size,
            std::ptr::null_mut(),
            0,
        ) != 0
        {
            // If command fails return default
            return None;
        }

        buff.pop(); // remove null terminator
        buff
    };

    String::from_utf8(string_buf).ok()
}

/// Get the version of the currently running kernel.
///
/// Returns `None` if an error occured.
pub fn kernel_version() -> Option<String> {
    get_system_info(b"kern.osrelease\0")
}

/// Retrieve the OS version information.
///
/// Note that this only works on macOS 10.13.4+.
///
/// Based on
/// <https://github.com/rust-minidump/minidump-writer/blob/main/src/mac/streams/system_info.rs>
/// Also under [MIT license](https://github.com/rust-minidump/minidump-writer/blob/94305066631b93eba768050e362e7a4bed40de1e/LICENSE).
pub fn macos_version() -> Option<String> {
    get_system_info(b"kern.osproductversion\0")
}
