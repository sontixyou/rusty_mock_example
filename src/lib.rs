use mockall_double::double;

mod mockable {
    #[cfg(test)]
    use mockall::automock;

    pub struct Tashizan {}

    #[cfg_attr(test, automock)]
    impl Tashizan {
        // 関連関数だとモックはうまくいかない
        // pub fn addition(x: u32) -> u32 {
        pub fn addition(&self, x: u32) -> u32 {
            x
        }
    }
}

#[double]
use mockable::Tashizan;

fn multiply(x: Tashizan, y: u32) -> u32 {
    x.addition(1) * y
}
// fn multiply(y: u32) -> u32 {
//     Tashizan::addition(1) * y
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiply_test() {
        let mut mock = Tashizan::new();
        mock.expect_addition().returning(|x| x + 1);
        assert_eq!(84, multiply(mock, 42));
    }
}
