//! Windows registry read helpers for Dassault Abaqus.

pub const APP_KEY: &str = "Software\\Dassault\\Dassault Abaqus";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Dassault Abaqus";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Dassault Abaqus") },
        RegValue { name: "Version",     value: String::from("v1.8.21") },
        RegValue { name: "Channel",     value: String::from("lts") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Dassault Abaqus") },
        RegValue { name: "Publisher",       value: String::from("Dassault") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Dassault Abaqus\\uninstall.exe\"") },
    ]
}
