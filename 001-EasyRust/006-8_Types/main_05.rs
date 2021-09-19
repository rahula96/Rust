fn main() {
    let len = "foo".len();
    assert_eq!(3, len);

    assert_eq!("foo".len(), 5);
    assert_eq!("foo".chars().count(), 3);
}
