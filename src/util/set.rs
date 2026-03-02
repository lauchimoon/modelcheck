pub fn print_set<I>(set: I)
where I: IntoIterator<Item = String>,
      I::IntoIter:ExactSizeIterator,
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
