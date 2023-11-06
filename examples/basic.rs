use webt::header::{authorization::Basic, HeaderKey};

fn main() {
    let basic = Basic::new("karl", "123456");
    println!("{}", basic.value());
    let basic = Basic::try_from(basic.value());
    println!("{:#?}", basic)
}
