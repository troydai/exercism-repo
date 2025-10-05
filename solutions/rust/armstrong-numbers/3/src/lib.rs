pub fn is_armstrong_number(num: u32) -> bool {
    let mut input:u64 = num.into();
    let mut remain = num;

    // find the number of digits using log10
    let count = (num as f64).log10() as u32 + 1;

    // split each digit of the number.
    // this also produce the number of the digits through the length of the vector.
    while remain > 0 {
        let digit = remain % 10;
        let p:u64 = digit.pow(count).into();
        if p > input {
            return false;
        }

        input -= p;
        remain /= 10;
    }

    input == 0
}
