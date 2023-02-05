pub fn get_flags(flags: i32) -> Vec<i32> {
    let mut fvec: Vec<bool> = Vec::new();
    let mut rest = flags;
    while rest != 0 {
        fvec.push((rest % 2) != 0);
        rest = rest / 2;
    }

    let mut tvec: Vec<i32> = Vec::new();
    for (i, value) in fvec.iter().enumerate() {
        if *value {
            tvec.push(i as i32);
        }
    }
    tvec
}

pub fn vec_to_flag(vec: Vec<bool>) -> u128 {
    let mut flag = 0;
    for (i, &val) in vec.iter().enumerate() {
        if val {
            flag |= 1 << i;
        }
    }
    flag
}