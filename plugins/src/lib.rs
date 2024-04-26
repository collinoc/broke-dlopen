use std::cell::RefCell;

#[no_mangle]
pub fn init_func(plugins: &mut Vec<fn()>) {
    plugins.push(do_something);
    println!("Loaded plugin A");
}

pub fn do_something() {
    thread_local! {
        // static CELL: RefCell<u8> = RefCell::new(1); // This is fine
        static CELL: RefCell<Box<u8>> = RefCell::new(Box::new(1)); // This doesn't reload as expected
    }

    CELL.with_borrow(|cell| {
        println!("Cell: {cell}");
    });
}
