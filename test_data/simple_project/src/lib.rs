
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn multiply(left: u64, right: u64) -> u64 {
    left * right
}

pub fn repeat(str: &str, times: usize) -> String {
    str.repeat(times)
}

#[cfg(test)]
mod test {
    use crate::add;

    #[test]
    fn add_8_and_8_is_16() {
        let result = add(8, 8);
        assert_eq!(result, 16);
    }
}
