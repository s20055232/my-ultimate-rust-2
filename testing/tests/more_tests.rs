#[test]
fn test_sploosh() {
    assert_eq!(
        testing::sploosh(
            testing::splish(-1, 0),
            testing::splish(1, 1),
            testing::splish(3, 2),
        ),
        4
    );
}
