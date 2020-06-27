#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

// home page
#[get("/")]
fn home() -> &'static str {
    "Hello, welcome to home page please add any number in url (like /10) to add-it"
}
// query parameter page
#[get("/<num>")]
fn num(num:u32)-> String {
    format!("Number is = {} after addition of '10' the result is = {}",num ,num + 10)
}

fn main() {
    rocket::ignite().mount("/", routes![home,num]).launch();
}