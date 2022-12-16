fn main() {
    fn add_u32(list: &[u32]) -> Option<u32> {
        let mut result: u32 = 0;
        for v in list {
            if (result + *v) < result {
                return None;
            } else {
                result += *v;
            }
        }
        return Some(result);
    }

    let l1: &[u32] = &[327, 5, 326];
    println!("{:#?}", add_u32(l1));

    // let a = u64::MAX;
    // println!("{}", a);

    let a = u32::MAX - 1;
    println!("{}", a);

    let l2: &[u32] = &[a, 0, 4, 5];
    println!("{:#?}", add_u32(l2));
}
