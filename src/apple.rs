use objc::rc::autoreleasepool;
use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};
use std::ffi::CStr;
use std::os::raw::c_char;
use std::path::PathBuf;

// We need to link to Foundation to access the NSFileManager class
#[link(name = "Foundation", kind = "framework")]
extern "C" {}

/// Type of directory to lookup from macOS/iOS
#[repr(u64)]
pub enum SearchPathDirectory {
    /// Applications directory, depending on domain. /Applications or ~/Applications typically
    Application = 1,
    AdminApplication = 4,
    /// Library folder, can be /Library (system) or ~/Library (user)
    Library = 5,
    /// Location of usere's home directories, typically /Users
    Users = 7,
    /// Documentation, not sure if used...
    Documentation = 8,
    /// Documents folder, typically ~/Documents
    Documents = 9,
    AutosavedInformation = 11,
    /// User's desktop folder, typically ~/Desktop
    Desktop = 12,
    /// Caches folder, Library/Caches
    Caches = 13,
    /// Applicatino support, Library/Application Support
    ///
    /// Typical home of non-userdefaults app data & settings
    ApplicationSupport = 14,
    /// Downloads folder, ~/Downloads
    Downloads = 15,
    /// Movies folder, ~/Movies
    Movies = 17,
    /// Music folder, ~/Music
    Music = 18,
    /// Pictures folder, ~/Pictures
    Pictures = 19,
    PrinterDescription = 20,
    SharedPublic = 21,
    PreferencePanes = 22,
    ApplicationScripts = 23,
    Trash = 102,
}

#[repr(u64)]
pub enum SearchPathDomainMask {
    UserDomain = 1,
    LocalDomain = 2,
    NetworkDomain = 4,
    SystemDomain = 8,
    AllDomains = 65535,
}

/// Returns first path found on macOS/iOS systems given the requested type of path, within the domain
///
/// Even if a path is returned, it may not exist yet and require creation
pub fn path_for_dir(dir: SearchPathDirectory, domain: SearchPathDomainMask) -> Option<PathBuf> {
    let mut result = None;
    autoreleasepool(|| {
        let cls = class!(NSFileManager);
        unsafe {
            let obj: *mut Object = msg_send![cls, defaultManager];
            let url: *mut Object = msg_send![obj, URLForDirectory:dir inDomain:domain as u64 appropriateForURL:0 create:false error:0];
            if !url.is_null() {
                let path: *mut Object = msg_send![url, path];
                let s: *const c_char = msg_send![path, UTF8String];
                let c_str = CStr::from_ptr(s);
                match c_str.to_str() {
                    Err(error) => {
                        println!("Error getting home dir string: {}", error);
                    }
                    Ok(string) => result = Some(PathBuf::from(string.to_owned())),
                };
            } else {
                println!("Failed to get dir");
            }
        }
    });
    result
}
/// Returns user's home directory, or sandbox if called within sandboxed app
pub fn home_dir() -> Option<PathBuf> {
    unsafe {
        let mut result = None;
        autoreleasepool(|| {
            let cls = class!(NSFileManager);
            let obj: *mut Object = msg_send![cls, defaultManager];
            let url: *mut Object = msg_send![obj, homeDirectoryForCurrentUser];
            let path: *mut Object = msg_send![url, path];
            let s: *const c_char = msg_send![path, UTF8String];
            let c_str = CStr::from_ptr(s);
            match c_str.to_str() {
                Err(error) => {
                    println!("Error getting home dir string: {}", error);
                }
                Ok(string) => result = Some(PathBuf::from(string.to_owned())),
            };
        });
        result
    }
}
