#![cfg(windows)]

use winapi::shared::minwindef;
use winapi::shared::minwindef::{BOOL, DWORD, LPVOID};
use windows::Win32::Foundation::HINSTANCE;

pub mod hooks;

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(
    dll_module: HINSTANCE,
    call_reason: DWORD,
    reserved: LPVOID)
    -> BOOL
{
    const DLL_PROCESS_ATTACH: DWORD = 1;
    const DLL_PROCESS_DETACH: DWORD = 0;

    match call_reason {
        DLL_PROCESS_ATTACH => unsafe {
            hooks::demo_init();
        },
        DLL_PROCESS_DETACH => (),
        _ => ()
    }
    minwindef::TRUE
}
