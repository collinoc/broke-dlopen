fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = String::new();

    let mut libs = Vec::new();
    let mut plugins = Vec::new();

    loop {
        plugins.clear();
        libs.clear();

        println!("Press enter to reload...");
        std::io::stdin().read_line(&mut buffer)?;

        unsafe {
            // This typically would loop over a directory, attempting to load all *.so files
            // that match a certain signature, but this is just an example
            let lib = libloading::Library::new("../target/release/libplugins.so")?;
            let plugin: libloading::Symbol<fn(&mut Vec<fn()>)> = lib.get(b"init_func")?;

            plugin(&mut plugins);
            libs.push(lib);
        }

        for plugin in &plugins {
            plugin();
        }
    }
}
