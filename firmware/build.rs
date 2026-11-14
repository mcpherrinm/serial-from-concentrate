//! This build script generates `memory.x` for the selected chip and puts it
//! where the linker can find it, then sets up the linker scripts.
//!
//! `memory.x` is the chip's `MEMORY` block (generated here) followed by the
//! chip's boot/metadata `SECTIONS` (from `link-rp2040.x` or `link-rp235x.x`).

use std::env;
use std::fs;
use std::path::PathBuf;

/// Flash size declared to the linker, shared by all boards.
///
/// Every board we support has at least 2 MiB of flash, so under-declaring
/// keeps one definition. Raise this (or make it per-board) when we need
/// more space for code or on-flash storage.
const FLASH_KIB: u32 = 2048;

fn main() {
    let rp2040 = env::var_os("CARGO_FEATURE_RP2040").is_some();
    let rp235x = env::var_os("CARGO_FEATURE_RP235X").is_some();

    let memory_x = match (rp2040, rp235x) {
        (true, false) => format!(
            "MEMORY {{
    /* The RP2040 boot ROM loads the second-stage bootloader from the first 256 bytes of flash. */
    BOOT2 : ORIGIN = 0x10000000, LENGTH = 0x100
    FLASH : ORIGIN = 0x10000100, LENGTH = {FLASH_KIB}K - 0x100
    /* SRAM0-SRAM3, striped. */
    RAM   : ORIGIN = 0x20000000, LENGTH = 256K
    /* SRAM4 and SRAM5 are directly mapped. */
    SRAM4 : ORIGIN = 0x20040000, LENGTH = 4K
    SRAM5 : ORIGIN = 0x20041000, LENGTH = 4K
}}

{}",
            include_str!("link-rp2040.x")
        ),
        (false, true) => format!(
            "MEMORY {{
    /* No boot2 on the RP2350; the boot ROM finds embassy-rp's IMAGE_DEF in .start_block. */
    FLASH : ORIGIN = 0x10000000, LENGTH = {FLASH_KIB}K
    /* SRAM0-SRAM7, striped. */
    RAM   : ORIGIN = 0x20000000, LENGTH = 512K
    /* SRAM8 and SRAM9 are directly mapped. */
    SRAM8 : ORIGIN = 0x20080000, LENGTH = 4K
    SRAM9 : ORIGIN = 0x20081000, LENGTH = 4K
}}

{}",
            include_str!("link-rp235x.x")
        ),
        // board/mod.rs emits a friendlier compile_error! for this case.
        _ => String::new(),
    };

    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    fs::write(out.join("memory.x"), memory_x).unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=link-rp2040.x");
    println!("cargo:rerun-if-changed=link-rp235x.x");

    println!("cargo:rustc-link-arg-bins=--nmagic");
    println!("cargo:rustc-link-arg-bins=-Tlink.x");
    // embassy-rp only provides link-rp.x (boot2 placement) for the RP2040.
    if rp2040 {
        println!("cargo:rustc-link-arg-bins=-Tlink-rp.x");
    }
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");
}
