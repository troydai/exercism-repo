pub fn is_armstrong_number(num: u32) -> bool {
    let input = num;
    let mut num = num;
    let mut v= Vec::<u32>::new();

    // split each digit of the number
    while num > 0 {
        v.push(num%10);
        num /= 10;
    }

    let mut delta:i64 = input.into();
    for i in v.iter() {
        let p = i.pow(v.len() as u32);
        delta -= p as i64;
        if delta < 0 {
            return false;
        }
    }

    delta == 0
}
