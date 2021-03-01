pub fn slice() {
    let thing: String = format!("hi2u");

    // This is a string slice, notice the &thing.
    // It skips the first byte and starts from the first byte of the String buffer.

    print_slice(&thing[1..]);
    print_slice(&thing);

    // We can also work on slices of the original string, without making any new copies.
    println!("{}", remove_vowels(&thing[1..]));
}

// String slices are represented by &str.
// This `name` is just a borrowed reference, that instead of referring to the whole string, refers to a subset of the data.
// It isn't a copy but just a reference to a slice of a string, which makes it a lot more efficient.
//
// +---------------------------------------+
// |                                       |
// |                                       |
// |                String                 | --\
// |                                       |    ---\
// |                                       |        ---\
// |--------------------------------------->            ---\        +----------^----------^---------^----------+
// |                                       |                ---\    |          |          |         |          |
// |                 len                   |                    --> |    h     |    i     |   2     |   u      |
// |                                       |                        |          |          |         |          |
// |                                       |                        +------------------------------------------+
// |--------------------------------------->                                        ^
// |                                       |                                       -/
// |                                       |                                     -/
// |                 data                  |                                   -/
// |                                       |                                 -/
// |                                       |                               -/
// |--------------------------------------->                             -/
// |                                       |                           -/
// |                                       |                         -/
// |                 cap                   |                       -/
// |                                       |                     -/
// +---------------------------------------+                   -/
//              original string                              -/
//                                                         -/
//                                                       -/
//                                                     -/
// +---------------------------------------+         -/
// |                                       |       -/
// |                 data                  |     -/
// |                                       |   -/
// |                                       | -/
// |-------------------------------------->|
// |                                       |
// |                 len                   |
// |                                       |
// +---------------------------------------+
//                  slice
//
pub fn print_slice(name: &str) {
    println!("{}", name);
}

// We can also work on slices of the original string, without making any new copies.
// Here we use higher-level constructs like `fold` and `match` backed by fairly efficient use of memory underneath.
// The slice passed to remove_vowels is neither a copy nor a new allocation.
// The only new allocation here is the fresh output string being created without vowels.
pub fn remove_vowels(name: &str) -> String {
    name.chars().fold(String::new(), |mut output, c| {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                // skip vowels
            }
            _ => {
                output.push(c);
            }
        }

        output
    })
}
