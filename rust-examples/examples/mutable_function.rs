fn main() {
    let mut x = 5;
    {
        let mut square_x = || x *= x;
        square_x();
    }
    assert_eq!(x, 25);

    fn do_twice<F>(mut func: F)
    where
        F: FnMut(),
    {
        func();
    }

    let mut x: usize = 1;
    {
        let add_two_to_x = || x += 2;
        do_twice(add_two_to_x);
    }
    {
        let sub_two_to_x = || x -= 2;
        do_twice(sub_two_to_x);
    }

    assert_eq!(x, 1);
}
