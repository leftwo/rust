fn main() {
    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{} days", 31);

    // Without a suffix, 31 becomes an i32. You can change what type 31 is
    // by providing a suffix.

    // There are various optional patterns this works with. Positional
    // arguments can be used.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    let object = "the lazy dog";
    // As can named arguments.
    println!("{subject} {verb} {object}",
             object = object,
             subject = "the quick brown fox",
             verb = "jumps over");

    // Special formatting can be specified after a `:`.
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    println!("{0} hex:0x{0:x} binary:{0:b}", 255);
    // You can right-align text with a specified width. This will output
    // "     1". 5 white spaces and a "1".
    // The $ tells it it's the width
    println!("{number:>width$}", number=1, width=6);

    // You can pad numbers with extra zeroes. This will output "000001".
    println!("{number:>0width$}", number=1, width=6);

}
