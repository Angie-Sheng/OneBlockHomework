fn main() {
    fn bubble_sort(l: &mut [i32]) {
        let len = l.len();

        for i in 0..len {
            for k in 0..len - i - 1{
                if l[k] > l[k+1]{
                    l.swap(k, k+1);
                }
            }
        }
    }

    fn bubble_sort_trait<T: PartialOrd>(l: &mut [T]) {
        let len = l.len();

        for i in 0..len {
            for k in 0..len - i - 1{
                if l[k] > l[k+1]{
                    l.swap(k, k+1);
                }
            }
        }
    }


    let mut l = [5_i32, 56, 2, 4, 1];
    println!("给定一个i32数组{:?}", l);
    bubble_sort(&mut l);
    println!("排序结果为{:?}", l);


    let mut l2 = ["xz", "327", "nice", "bang"];
    println!("给定一个任意实现了ParticialOrd的数组{:?}", l2);
    bubble_sort_trait(&mut l2);
    println!("排序结果为{:?}", l2);

    
}
