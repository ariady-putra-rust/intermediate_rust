//! # Running Code on Clean-up with the `Drop` Trait
//! `Drop` lets you customize what happens when a value is about to go out of scope. You can provide an implementation
//! for the `Drop` trait on any type, and that code can be used to release resources like files or network connections.
//!
//! `Drop` trait is almost always used when implementing a smart pointer. For example, when a `Box<T>` is dropped
//! it will deallocate the space on the heap that the box points to.
//!
//! In some languages, for some types, the programmer must call code to free memory or resources every time they
//! finish using an instance of those types. Examples include file handles, sockets, or locks. If they forget, the
//! system might become overloaded and crash. In Rust, you can specify that a particular bit of code be run whenever
//! a value goes out of scope, and the compiler will insert this code automatically. As a result, you don’t need to
//! be careful about placing clean-up code everywhere in a program that an instance of a particular type is finished
//! with—you still won’t leak resources!

use std::io::Result;

struct CustomSmartPointer {
    data: String,
}
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

pub fn drop_trait() -> Result<()> {
    Ok({
        let _c = CustomSmartPointer {
            data: String::from("my stuff"),
        };
        let _d = CustomSmartPointer {
            data: String::from("other stuff"),
        };
        println!("CustomSmartPointers created.");
        // // # Dropping a Value Early with `std::mem::drop`
        // drop(_c);
        // println!("CustomSmartPointer dropped before the end of the function.");
        // // borrow of moved value: `c`
        // println!("stuff: {}", _c.data); // value borrowed here after move
        // println!("stuff: {}", _d.data);
    })
    // remember stack:
}
// CustomSmartPointers created.
// Dropping CustomSmartPointer with data `other stuff`!
// Dropping CustomSmartPointer with data `my stuff`!

// Rust automatically called `drop` for us when our instances went out of scope,
// calling the code we specified. Variables are dropped in the reverse order
// of their creation, so `d` was dropped before `c`. This example’s purpose is
// to give you a visual guide to how the `drop` method works; usually you would
// specify the clean-up code that your type needs to run rather than a print
// message.

// You can use code specified in a `Drop` trait implementation in many ways to make
// cleanup convenient and safe: for instance, you could use it to create your own
// memory allocator! With the `Drop` trait and Rust’s ownership system, you don’t
// have to remember to clean up because Rust does it automatically.

// You also don’t have to worry about problems resulting from accidentally cleaning
// up values still in use: the ownership system that makes sure references are always
// valid also ensures that `drop` gets called only once when the value is no longer
// being used.
