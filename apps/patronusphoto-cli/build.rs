//! Windows only: embed the app icon and version info (VERSIONINFO) into `patronusphoto-cli.exe`.
//!
//! On every other target this does nothing, even when `PATRONUSPHOTO_REQUIRE_WINRES` is set. A
//! missing resource compiler is a warning, so a cross-compile from macOS or Linux still links,
//! unless `PATRONUSPHOTO_REQUIRE_WINRES=1` (set by the release packaging script) turns it into a
//! build error.

/// Embeds the Windows resources and delay-loads the print-only system DLLs (#8272).
fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../assets/app-icon/patronusphoto.ico");
    println!("cargo:rerun-if-env-changed=PATRONUSPHOTO_REQUIRE_WINRES");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return Ok(());
    }
    // Printing to PDF (winprint's `PdfCreateRenderer`) imports Windows.Data.Pdf.dll, and its
    // printer options (the Print Ticket API) import prntvpt.dll; stripped Windows images (NTLite,
    // some N/LTSC builds) lack them. A load-time import stops the exe from starting at all
    // ("windows.data.pdf.dll was not found", #5884; "prntvpt.dll", #8272), so load them on first
    // use; the print commands check for them first and report an error instead.
    if std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc") {
        println!("cargo:rustc-link-arg-bins=/DELAYLOAD:windows.data.pdf.dll");
        println!("cargo:rustc-link-arg-bins=/DELAYLOAD:prntvpt.dll");
        println!("cargo:rustc-link-arg-bins=delayimp.lib");
    }
    let mut res = winresource::WindowsResource::new();
    // FileVersion / ProductVersion strings and the numeric FILEVERSION / PRODUCTVERSION are left
    // to `WindowsResource::new`, which reads the Cargo package version. A pre-release such as
    // `1.2.3-rc.1` stays in the strings; the numeric fields are `1.2.3.0`.
    res.set_icon("../../assets/app-icon/patronusphoto.ico")
        .set("ProductName", "Patronus Photo")
        .set("FileDescription", "Patronus Photo command-line interface")
        .set("CompanyName", "Patronus")
        .set("LegalCopyright", "Copyright (c) the PhotoCraft authors. MIT OR Apache-2.0.")
        .set("OriginalFilename", "patronusphoto-cli.exe")
        .set("InternalName", "patronusphoto-cli");
    if let Err(err) = res.compile() {
        if std::env::var_os("PATRONUSPHOTO_REQUIRE_WINRES").is_some() {
            return Err(std::io::Error::other(format!(
                "embedding Windows resources into patronusphoto-cli.exe failed: {err}. \
                 Install the Windows SDK resource compiler (rc.exe) or mingw-w64 windres and rebuild."
            )));
        }
        println!("cargo:warning=patronusphoto-cli.exe built without icon/version resources: {err}");
    }
    Ok(())
}
