use std::cell::RefCell;

#[no_mangle]
pub fn lib_func() {
    thread_local! {
        // static CELL: RefCell<u8> = RefCell::new(1); // This is fine
        static CELL: RefCell<Box<u8>> = RefCell::new(Box::new(1)); // This doesn't change if reloaded
    }

    CELL.with_borrow(|cell| {
        println!("Cell: {cell}");
    });
}
