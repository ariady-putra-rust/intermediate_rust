use std::{
    io::{Error, ErrorKind, Result},
    thread,
};

pub fn main_thread() -> Result<()> {
    Ok({
        let i = 42;

        /* let thread_handle = thread::spawn(|| i);
        closure may outlive the current function, but it borrows `i`, which is owned by the current function may outlive borrowed value `i`
        to force the closure to take ownership of `i` (and any other referenced variables), use the `move` keyword: `move`
        */
        let thread_handle = thread::spawn(move || i);

        println!("at main_thread: {i}");

        match thread_handle.join() {
            Ok(thread_result) => println!("at thread::join: {thread_result}"),
            Err(_) => {
                return Err(Error::new(
                    ErrorKind::Other,
                    "couldn't join on the associated thread",
                ))
            }
        };

        println!("after thread::join: {i}");
    })
}
