pub fn slice_plus<T: Clone>(a: &[T], b: &[T]) -> Box<[T]> {
    let mut result = a.to_vec();
    result.extend_from_slice(b);
    result.into_boxed_slice()
}

// A Character is a GraphemeCluster, not a unicode code point
// pub type Character = String;
