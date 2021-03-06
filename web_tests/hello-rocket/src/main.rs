#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

#[get(/)]
fn index() -> &'static str {
    "hello, rust buckets"
}

#[lauch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
