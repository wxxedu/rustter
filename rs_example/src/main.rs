use rustter::flutter;

#[flutter]
fn add_two_num(x: u32, y: u32) -> u32 {
    x + y
}

fn main() {
    let res = add_two_num(4, 4);
    println!("res: {}", res);
}
