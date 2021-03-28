pub mod correctors;

#[cfg(test)]
mod tests {
    use crate::correctors;
    #[test]
    fn images_are_adapted() {
        assert_eq!(correctors::do_correct_images(), "test");
    }
}
