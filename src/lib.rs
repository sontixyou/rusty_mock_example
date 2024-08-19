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

use mockall::automock;
#[automock]
pub mod mockable_addition {
    pub fn addition(x: u32) -> u32 {
        x
    }
}

mod mockable_multiply {

    use mockall_double::double;

    #[double]
    use super::mockable_addition;

    // これでもよい
    // #[cfg(test)]
    // use super::mock_mockable_addition as mockable_addition;
    // #[cfg(not(test))]
    // use super::mockable_addition;

    // pub async fn multiply(y: u32) -> u32 {
    pub fn multiply(y: u32) -> u32 {
        mockable_addition::addition(1) * y
    }
}

#[cfg(test)]
mod multiply_tests {
    use mockall_double::double;

    #[double]
    use crate::mockable_addition;

    use crate::mockable_multiply::multiply;

    // async関数でもモックできる
    // #[tokio::test]
    // async fn multiply_test() {
    //     // #[test]
    //     // fn multiply_test() {
    //     let ctx = mockable_addition::addition_context();
    //     ctx.expect().returning(|x| x + 1);
    //     assert_eq!(4, multiply(2).await);
    // }
    #[test]
    fn multiply_test() {
        // #[test]
        // fn multiply_test() {
        let ctx = mockable_addition::addition_context();
        ctx.expect().returning(|x| x + 1);
        assert_eq!(4, multiply(2));
    }
}
