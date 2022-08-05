

fn main() {
    let l1 = vec![1, 2, 3];
    match sum_u32(&l1) {
        Some(v) => println!("sum is {}", v),
        None => println!("sum overflow"),
    }
    let l2 = vec![1, u32::MAX];
    match sum_u32(&l2) {
        Some(v) => println!("sum is {}", v),
        None => println!("sum overflow"),
    }
}

fn sum_u32(a: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0;
    for v in a.iter() {
        match sum.checked_add(*v) {
            Some(v) => sum = v,
            None => return None,
        }
    }
    Some(sum)
}