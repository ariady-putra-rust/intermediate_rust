use std::io::Result;

pub fn lambda() -> Result<()> {
    Ok({
        // The way a closure captures and handles values from the environment affects which traits the closure implements,
        // and traits are how functions and structs can specify what kinds of closures they can use.
        // Closures will automatically implement one, two, or all three of these `Fn` traits, in an additive fashion,
        // depending on how the closure’s body handles the values:
        //
        // 1. `FnOnce` applies to closures that can be called once.
        // All closures implement at least this trait, because all closures can be called.
        // A closure that moves captured values out of its body will only implement `FnOnce` and none of the other `Fn` traits,
        // because it can only be called once.
        //
        // 2. `FnMut` applies to closures that don’t move captured values out of their body,
        // but that might mutate the captured values. These closures can be called more than once.
        //
        // 3. `Fn` applies to closures that don’t move captured values out of their body and that don’t mutate captured values,
        // as well as closures that capture nothing from their environment.
        // These closures can be called more than once without mutating their environment,
        // which is important in cases such as calling a closure multiple times concurrently.
    })
}
