use my_lib::MyType;

fn main() {
    let my_inst = MyType::builder().field(42).build();
    println!("{my_inst:?}");
}
