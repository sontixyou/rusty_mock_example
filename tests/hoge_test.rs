#[cfg(test)]
// このテストは失敗する。
// 外部クレートとしてmock_exampleをインポートしているため、テストは失敗する
mod test_example {
    use mockall_double::double;

    #[double]
    use mock_example::mockable_addition;

    use mock_example::mockable_multiply::multiply;

    #[test]
    fn mockable_multiply_test() {
        let ctx = mockable_addition::addition_context();
        ctx.expect().returning(|x| x + 1);
        assert_eq!(4, multiply(2));
    }
}
