use firstlist::{SListV1, SListV3};

pub fn main() {
    let list : SListV1<i32> = SListV1::Cons(1, Box::new(SListV1::Cons(2, Box::new(SListV1::Nil))));
    println!("{:?}", list);

    let mut listv3 = SListV3::new();
    listv3.push(0);
    listv3.push(1);
    listv3.push(2);
    println!("{:?}", listv3);

    listv3.pop();
}