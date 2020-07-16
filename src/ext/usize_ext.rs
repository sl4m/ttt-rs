pub(crate) trait UsizeExt {
    fn sqrt(&self) -> usize;
    fn is_perfect_sq(&self) -> bool;
}

impl UsizeExt for usize {
    #[allow(
        clippy::as_conversions,
        clippy::cast_possible_truncation,
        clippy::cast_precision_loss,
        clippy::cast_sign_loss
    )]
    fn sqrt(&self) -> usize {
        (*self as f64).sqrt() as usize
    }

    fn is_perfect_sq(&self) -> bool {
        self.sqrt().pow(2) == *self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_the_approximate_sqrt_of_usize() {
        assert_eq!(3, 9usize.sqrt());
        assert_eq!(4, 16usize.sqrt());
        assert_eq!(5, 25usize.sqrt());
    }

    #[test]
    fn it_checks_if_num_is_perfect_square() {
        assert_eq!(true, 9usize.is_perfect_sq());
        assert_eq!(true, 16usize.is_perfect_sq());
        assert_eq!(false, 24usize.is_perfect_sq());
    }
}
