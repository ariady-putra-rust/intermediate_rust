use std::io::Result;

mod box_t;
mod rc_t;
mod ref_cell_t;

#[allow(dead_code)]
pub enum SmartPointer {
    Nothing,
    BoxT,
    RcT,
    RefCellT,
}

/// Here is a recap of the reasons to choose `Box<T>`, `Rc<T>`, or `RefCell<T>`:
///
/// 1. `Rc<T>`              enables multiple owners of the same data;
///    `Box<T>` and `RefCell<T>` have single owners.
/// 2. `Box<T>`     allows      immutable or mutable borrows checked at compile time;
///    `Rc<T>`      allows only immutable            borrows checked at compile time;
///    `RefCell<T>` allows      immutable or mutable borrows checked at      runtime.
/// 3. Because `RefCell<T>`     allows       mutable borrows checked at      runtime,
/// you can mutate the value inside the `RefCell<T>` even when the `RefCell<T>` is immutable.
pub fn smart_pointer(smart_pointer: SmartPointer) -> Result<()> {
    match smart_pointer {
        SmartPointer::BoxT => box_t::box_t(),
        SmartPointer::RcT => rc_t::rc_t(),
        SmartPointer::RefCellT => ref_cell_t::ref_cell_t(),
        _ => Ok(()),
    }
}
