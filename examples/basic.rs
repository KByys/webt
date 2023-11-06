use webt::header::authorization::Basic;

fn main() {
    let basic = Basic::new("karl", "123456");
    println!("{}", basic.basic());
    let basic = Basic::try_from(basic.basic());
    println!("{:#?}", basic)
}
