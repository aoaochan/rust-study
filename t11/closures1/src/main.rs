fn main() {
    #[derive(Debug)]
    struct Rectangle { width: u32, height: u32 }
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];
    // list.sort_by_key(|r| r.width);
    // println!("{list:#?}");
    // let mut sort_op = vec![];
    // let v = String::from("closure called");
    // list.sort_by_key(|r| {
    //     sort_op.push(v);
    //     r.width
    // });
    // println!("{list:#?}");
    let mut cnt_sort_op = 0;
    list.sort_by_key(|r| {
        cnt_sort_op += 1;
        r.width * r.height
    });
    println!("{list:#?}, sorted in {cnt_sort_op} operations");

    // impl<T> Option<T> {
    //     pub fn unwrap_or_else<F>(self, f: F) -> T
    //     where 
    //         F: FnOnce() -> T
    //     {
    //         match self T {
    //             Some(x) => x,
    //             None => f()
    //         }
    //     }
    // }

    // let list = vec![1, 2, 3];
    // println!("Before defining closure: {list:?}");
    // std::thread::spawn(move || println!("From thread: {list:?}"))
    //     .join()
    //     .unwrap();

    // let mut list = vec![1, 2, 3];
    // println!("Before defining closure: {list:?}");

    // let mut borrows_mutably = || list.push(7);

    // borrows_mutably();
    // println!("After calling closure: {list:?}");

    // let list = vec![1, 2, 3];
    // println!("Before defining closure: {list:?}");

    // let only_borrows = || println!("From closure: {list:?}");

    // println!("Before calling closure: {list:?}");
    // only_borrows();
    // println!("After calling closure: {list:?}");
}
