#[test]
fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}

#[test]
fn addition() {
    let result = crate::add(1, 5);
    assert_eq!(result, 6);
}
