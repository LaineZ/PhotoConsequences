#[cfg(target_os = "linux")]
pub fn messagebox(title: &str, message: &str) {
    messagebox_x11::msgbox(title, message);
}

#[cfg(target_os = "windows")]
pub fn messagebox(title: &str, message: &str) {
    extern crate msgbox;
    use msgbox::IconType;
    
    msgbox::create(title, message, msgbox::IconType::Error).unwrap();
}

#[cfg(target_os = "macos")]
pub fn messagebox(title: &str, message: &str) {
    extern crate msgbox;
    use msgbox::IconType;

    msgbox::create(title, message, msgbox::IconType::Error).unwrap();
}
