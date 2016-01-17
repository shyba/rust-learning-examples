pub fn add(x: i32, y: i32) -> i32 {
    x + y
}


#[cfg(test)]
mod tests {
    use super::add;

    #[test]
    fn it_works() {
        assert_eq!(4, add(2, 2));
    }
}
