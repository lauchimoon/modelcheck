use std::fmt;

pub fn print_set<W, I, T>(f: &mut W, set: I) -> fmt::Result
where W: fmt::Write,
      I: IntoIterator<Item = T>,
      I::IntoIter:ExactSizeIterator,
      T: fmt::Display,
{
    let iter = set.into_iter();
    let length = iter.len();

    write!(f, "{{")?;
    for (i, x) in iter.enumerate() {
        if i + 1 >= length {
            write!(f, "{x}")?;
        } else {
            write!(f, "{x}, ")?;
        }
    }
    write!(f, "}}")?;
    Ok(())
}
