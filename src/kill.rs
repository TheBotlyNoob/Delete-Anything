use std::ffi::CStr;
use windows::Win32::{
    Foundation::CloseHandle,
    System::{
        Diagnostics::ToolHelp::{
            CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32,
            TH32CS_SNAPPROCESS,
        },
        Threading::{OpenProcess, TerminateProcess, PROCESS_TERMINATE},
    },
};

// https://stackoverflow.com/a/7956651 in Rust
pub fn by_name(filename: impl AsRef<str>) -> crate::Result<()> {
    let snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)? };

    let mut process_entry = PROCESSENTRY32 {
        dwSize: std::mem::size_of::<PROCESSENTRY32>() as u32,
        ..PROCESSENTRY32::default()
    };

    let mut res = unsafe { Process32First(snapshot, &mut process_entry).as_bool() };

    while res {
        let process_name = unsafe {
            CStr::from_ptr(process_entry.szExeFile.as_ptr() as _)
                .to_string_lossy()
                .to_lowercase()
        };
        if process_name == filename.as_ref().to_lowercase() {
            println!("[+] Killing {process_name}...");

            if let Ok(process_handle) =
                unsafe { OpenProcess(PROCESS_TERMINATE, None, process_entry.th32ProcessID) }
            {
                unsafe { TerminateProcess(process_handle, 9) };
                unsafe { CloseHandle(process_handle) };
            }
        }

        res = unsafe { Process32Next(snapshot, &mut process_entry).as_bool() };
    }
    unsafe { CloseHandle(snapshot) };

    Ok(())
}
