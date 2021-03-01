/// Shared borrows.
/// Create a variable `thing` and create a shared reference to it so another function can borrow / read it.
pub fn shared() {
    // Create a `String` via `format!(&str)`
    let thing = format!("food");

    // Both the caller/callee (lender/borrower) need to stick the &
    // When `thing` is borrowed it cannot be mutated.
    let shared_reference: &String = &thing;
    print_shared_borrow(shared_reference);

    // Notice that we can now access `thing` in this scope, since `print_shared_borrow` never owned `thing`.
    println!("I love, {:?}, again.", thing);

    // We lend `print_shared_borrow` thing several times using our shared reference in succession.
    // Shared references are a copy type, so you can use them as many times as you want.
    print_shared_borrow(shared_reference);
    print_shared_borrow(shared_reference);

    // all data is destroyed once this function is done.
}

pub fn mutable_until_borrowed() {
    // Create a mutable `String` via `format!(&str)`
    let mut thing = format!("food");
    // This is possible even though, later on, we make a read-only `shared_reference` borrow.
    // Since that code hasn't become a part of the lifetime yet, we can safely do this.
    thing.push('s');

    let shared_reference: &String = &thing;

    // This is not allowed.
    // ⬇️⬇️Uncomment and see the compile error. ⬇️⬇
    // thing.push('s');

    print_shared_borrow(shared_reference);

    // This is still not allowed.
    // ⬇️⬇️Uncomment and see the compile error. ⬇️⬇
    // thing.push('s');

    print_shared_borrow(shared_reference);

    // This is again, not allowed.
    // ⬇️⬇️Uncomment and see the compile error. ⬇️⬇
    // thing.push('s');
    // thing cannot be mutated until it goes out of scope completely.

    // Look at `mutable_borrow_lifetime`,
    // to see how scopes can help restructure this code such that controlled mutation is allowed.
}

pub fn borrow_lifetime() {
    // Create a mutable `String` via `format!(&str)`
    let mut thing = format!("food");

    // We scoped our shared borrow to this block only.
    // This allows us to mutate `thing` once the lifetime of the `shared_reference` is finished.
    {
        let shared_reference: &String = &thing;
        print_shared_borrow(shared_reference);
        print_shared_borrow(shared_reference);
    } // <-- borrow ends here.

    // Works!
    thing.push('s');

    // Similary, `shared_reference` is now out of scope.
    // ⬇️⬇️Uncomment and see the compile error. ⬇️⬇
    // print_shared_borrow(shared_reference);
}

pub fn print_shared_borrow(thing: &String) {
    println!("Ownership, {}", thing);

    // We cannot do this because shared data by default is immutable. Only reads are allowed.
    // Certain types can mutate shared data, but the API will suggest so and it'll be transparent to the caller.
    // It's safe to assume that, generally, that's not the case.
    // ⬇️⬇️Uncomment and see the compile error. ⬇️⬇
    // thing.push('s');
}
