use pelite::pe32::{PeView, Pe};
use windows::Win32::Foundation::HINSTANCE;

fn sig2bytes(sig: &str) -> Vec<i32> {
    let mut bytes = Vec::new();
    for b in sig.split_whitespace() {
        if b.contains('?') {
            bytes.push(-1);
        } else {
            bytes.push(i32::from_str_radix(b, 16).unwrap());
        }
    }

    bytes
}

pub unsafe fn find_sig(module: HINSTANCE, sig: &str) -> Option<*const u8> {
    let module = module.0 as *const u8;
    let bytes = sig2bytes(sig);
    let module_view = PeView::module(module);
    let nt_headers = module_view.nt_headers();
    let image_size = nt_headers.OptionalHeader.SizeOfImage;

    for i in 0..(image_size as usize - bytes.len()) {
        let mut found = true;
        for k in 0..bytes.len() {
            if *module.offset((i + k) as isize) != bytes[k] as u8 && bytes[k] != -1 {
                found = false;
                break;
            }
        }
        if found {
            return Some(module.offset(i as isize));
        }
    }

    None
}