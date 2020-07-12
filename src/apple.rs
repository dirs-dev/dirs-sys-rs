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
    /// PPDs folder, Library/Printers/PPDs
    PrinterDescription = 20,
    /// Public folder, ~/Public
    SharedPublic = 21,
    /// Preference Panes, Library/PreferencePanes
    PreferencePanes = 22,
    /// User scripts folder for calling application, ~/Library/Application Scripts/code-signing-id
    ApplicationScripts = 23,
    /// Trash folder
    Trash = 102,
}

/// Domain for path to return, dirs currently mostly deals with user dirs so likely want UserDomain
#[repr(u64)]
pub enum SearchPathDomainMask {
    /// Looks up directory in user's domain, so ~
    UserDomain = 1,
    /// Local system domain, which is folders typically found in /Library
    LocalDomain = 2,
    /// Publically available locations on the local network
    NetworkDomain = 4,
    /// Read only system locations, /System (may be completely unavailable on newer systems?)
    SystemDomain = 8,
    /// Looks up directories in all of the current domains and future ones apple may add
    AllDomains = 65535,
}

/// Returns first path found on macOS/iOS systems given the requested type of path, within given domain
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
