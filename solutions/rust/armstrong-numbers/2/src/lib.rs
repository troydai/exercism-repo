pub fn is_armstrong_number(num: u32) -> bool {
    let mut input:u64 = num.into();
    let mut remain = num;
    let mut v= Vec::<u32>::new();

    // split each digit of the number.
    // this also produce the number of the digits through the length of the vector.
    while remain > 0 {
        v.push(remain%10);
        remain /= 10;
    }

    // check if the number is an armstrong number.
    // deduct from the input to avoid overflow.
    for i in v.iter() {
        let p:u64= i.pow(v.len() as u32).into();
        if p > input {
            return false;
        }
        input -= p
    }

    input == 0
}
