pub fn split_digits(number_to_split: &usize, vector_size: u8) -> Vec<u8> {
    let mut digits = Vec::<u8>::with_capacity(vector_size.into());
    let mut number = *number_to_split;
    while number != 0 {
        digits.push((number % 10).try_into().unwrap());
        number /= 10;
    }

    while digits.len() < vector_size.into() {
        digits.push(0);
    }

    digits.truncate(vector_size.into());

    digits
}
