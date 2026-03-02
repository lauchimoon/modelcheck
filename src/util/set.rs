use std::fmt::Display;

pub fn print_set<I, T>(set: I)
where I: IntoIterator<Item = T>,
      I::IntoIter:ExactSizeIterator,
      T: Display,
{
    let iter = set.into_iter();
    let length = iter.len();

    print!("{{");
    for (i, x) in iter.enumerate() {
        if i + 1 >= length {
            print!("{x}");
        } else {
            print!("{x}, ");
        }
    }
    println!("}}");
}
