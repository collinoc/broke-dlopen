use std::{thread, time::Duration};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        unsafe {
            let lib = libloading::Library::new("../target/release/libplugins.so")?;
            let lib_func: libloading::Symbol<fn()> = lib.get(b"lib_func")?;

            lib_func();
        }

        thread::sleep(Duration::from_secs(1));
    }
}
