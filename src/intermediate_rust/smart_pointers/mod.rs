//! # Smart Pointers
//! A _pointer_ is a general concept for a variable that contains an address in memory.
//! This address refers to, or “points at,” some other data. The most common kind of
//! pointer in Rust is a reference. References are indicated by the `&` symbol and borrow
//! the value they point to. They don’t have any special capabilities other than referring
//! to data, and have no overhead.
//!
//! _Smart pointers_, on the other hand, are data structures that act like a pointer but
//! also have additional metadata and capabilities. The concept of smart pointers isn’t
//! unique to Rust: smart pointers originated in C++ and exist in other languages as well.
//! Rust has a variety of smart pointers defined in the standard library that provide
//! functionality beyond that provided by references. To explore the general concept,
//! we’ll look at a couple of different examples of smart pointers, including a _reference
//! counting_ smart pointer type. This pointer enables you to allow data to have multiple
//! owners by keeping track of the number of owners and, when no owners remain, cleaning
//! up the data.
//!
//! Rust, with its concept of ownership and borrowing,
//! has an additional difference between references and smart pointers:
//! while references only borrow data, in many cases,
//! smart pointers own the data they point to.
//!
//! Smart pointers are usually implemented using structs. Unlike an ordinary struct,
//! smart pointers implement the `Deref` and `Drop` traits:
//! - The `Deref` trait allows an instance of the smart pointer struct to behave like a
//!   reference so you can write your code to work with either references or smart pointers.
//! - The `Drop` trait allows you to customize the code that’s run when an instance of the
//!   smart pointer goes out of scope.

use std::io::Result;

mod box_t;
mod rc_t;
mod ref_cell_t;
mod traits;

#[allow(dead_code)]
pub enum SmartPointer {
    Nothing,
    Drop,
    Deref,
    BoxT, // `Box<T>` for allocating values on the heap
    RcT,  // `Rc<T>`, a reference counting type that enables multiple ownership
    RefCellT, // `Ref<T>` and `RefMut<T>`, accessed through `RefCell<T>`,
          // a type that enforces the borrowing rules at runtime instead of compile time
}

/// Here is a recap of the reasons to choose `Box<T>`, `Rc<T>`, or `RefCell<T>`:
/// 1. `Rc<T>`              enables multiple owners of the same data;
///    `Box<T>` and `RefCell<T>` have single owners.
/// 2. `Box<T>`     allows      immutable or mutable borrows checked at compile time;
///    `Rc<T>`      allows only immutable            borrows checked at compile time;
///    `RefCell<T>` allows      immutable or mutable borrows checked at      runtime.
/// 3. Because `RefCell<T>`     allows       mutable borrows checked at      runtime,
///    you can mutate the value inside the `RefCell<T>` even when the `RefCell<T>`
///    is immutable.
pub fn smart_pointer(smart_pointer: SmartPointer) -> Result<()> {
    match smart_pointer {
        SmartPointer::BoxT => box_t::box_t(),
        SmartPointer::RcT => rc_t::rc_t(),
        SmartPointer::RefCellT => ref_cell_t::ref_cell_t(),
        SmartPointer::Deref => traits::deref::deref_trait(),
        SmartPointer::Drop => traits::drop::drop_trait(),
        _ => Ok(()),
    }
}
