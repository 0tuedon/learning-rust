use std::rc::Rc;

use std::cell::RefCell;

struct ImRef {
    is_cool: RefCell<bool>,
}
fn main() {
    // Box
    let x = 5;
    let y = Box::new(x);
    let z = *y;

    println!("{},{}, {}", x, y, z);

    // When you want multiple owners

    let a = Rc::new(String::from("Pointer"));
    let b = a.clone();
    let c = b.clone();

    println!("{},{},{}", a.contains("tued"), b, c);

    let ref_cell = ImRef {
        is_cool: RefCell::new(true),
    };
    let mut new_ref = ref_cell.is_cool.borrow_mut();
    *new_ref = false;

    println!("{}", new_ref)
}
