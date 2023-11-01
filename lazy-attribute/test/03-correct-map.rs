use lazy_attribute::lazy_map;

//--------------------------------------------------------------------------------------------------
// Main
//--------------------------------------------------------------------------------------------------

fn main() {
    println!("{:#?}", get_foo());
    println!("{:#?}", get_foo());
}

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

#[derive(Debug)]
struct Foo();

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

#[lazy_map(Foo, |r| r.unwrap())]
fn get_foo() -> Result<Foo, ()> {
    println!("get_foo called once!");
    Ok(Foo())
}