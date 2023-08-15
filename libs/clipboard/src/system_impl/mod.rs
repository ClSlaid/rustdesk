//! System-specific clipboard implementation

// TODO: Add support for Wayland
// TODO: Add support for mobile platforms

#[cfg(target_os = "macos")]
/// Local clipboard implementation for macOS using Objective-C API
pub mod macos;
#[cfg(target_os = "windows")]
/// Local clipboard implementation for Windows using win32 API
pub mod win32;
#[cfg(target_os = "linux")]
/// Local clipboard implementation for Linux using x11 API (won't consider BSDs)
pub mod x11;
