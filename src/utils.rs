pub fn pluralize<T>(len: usize, singular: T, plural: T) -> T {
    match len {
        1 => singular,
        _ => plural,
    }
}
