fn main() {
    // On Windows, link against required system libraries for git2
    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-lib=dylib=Advapi32");
        println!("cargo:rustc-link-lib=dylib=Crypt32");
    }
}
