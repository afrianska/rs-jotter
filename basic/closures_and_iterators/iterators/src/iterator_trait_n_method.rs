#[allow(dead_code)]
pub trait Iterator {
    type Item; // will talk about associated types in other chapter

    fn next(&mut self) -> Option<Self::Item>;
}

#[allow(dead_code)]
pub fn itnnm() {}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
