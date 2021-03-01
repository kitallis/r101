/// Ownership 101. Create a variable `thing` and transfer its ownership to another function forever.
/// Non-copyable ownership: things move from one place to another.
pub fn moves() {
    // Create a `String` via `format!(&str)`
    let thing = format!("food");
    print_ownership(thing);

    // Notice that we now cannot access thing in this scope, since `print_ownership` now owns `thing`.
    // ⬇️⬇️Uncomment and see the compiler error ⬇️⬇️
    // println!("I love, {:?}, again.", thing);
}

/// Transfer ownership of a copy of `thing` and retain ownership of the original.
/// Cloning: run some code to make a copy.
pub fn cloning() {
    // Create a String via `format!(&str)`
    let thing = format!("food");

    // Clone() makes a deep clone and creates a fresh copy of thing.
    print_ownership(thing.clone());

    // We continue to own `thing` and use it after its clone has been passed to `print_ownership`.
    // So `print_ownership`'s copy will be destroyed automatically after `print_ownership` is done with it.
    // And we continue to retain `thing` in this scope.
    println!("I love, {:?}, again.", thing);
}

/// Ownership of certain types that implement auto-Copy (effectively clone without implementing a clone).
/// Copyable: types that are implicitly copied.
pub fn copy_traits() {
    // Create i32 and f32 `thing`s.
    let thing: i32 = 42;
    let another_thing: f32 = 42.2;

    print_ownership_with_auto_copy(thing, another_thing);

    // We continue to own `thing` and `another_thing` and use it after it has been passed to `print_ownership_with_auto_copy`.
    // This is because simple types like i32 and f32 (integers and floats) are Copy types (i.e. they implement the Copy trait) by default.
    println!("I love, {:?}, again.", thing);
    println!("I love, {:?}, again.", another_thing);
}

/// Another way to "retain" ownership by passing the variable back to the caller after you're done with it.
/// This avoids copying, only transfers ownership.
/// When thing is sent to `print_ownership_and_return`, `with_returns` loses ownership.
/// When `print_ownership_and_return` returns, `with_returns re-acquires ownership under `thing`.
/// Overall, we've only allocated `thing` once in this scope.
pub fn returns() {
    // Create a `String` via `format!(&str)`
    let thing = format!("food");
    // Shadow to illustrate ownership transference.
    let thing = print_ownership_and_return(thing);

    // We continue to own 'thing' and use it after it has been passed to `print_ownership_and_return`.
    // So `print_ownership_and_return` simply parries the ownership back to us.
    println!("I love, {:?}, again.", thing);
}

/// So to summarize we have the following ownership forms for values:
///
/// Non-copyable (Moves)
/// Clone
/// Copy

pub fn print_ownership(thing: String) {
    println!("Ownership, {}", thing);
    // thing is now freed from memory, we can check this via the heapsize crate
    // TODO measure heapsize etc.
}

pub fn print_ownership_with_auto_copy(thing: i32, another_thing: f32) {
    println!("Ownership with auto-copy, {}, {}", thing, another_thing)
}

pub fn print_ownership_and_return(thing: String) -> String {
    println!("Ownership and return, {}", thing);
    thing
}
