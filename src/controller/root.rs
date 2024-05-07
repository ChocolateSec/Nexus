use rocket::get;

#[get("/")]
pub fn get_index() -> &'static str {
    "Hello from Nexus, the HotChocolate module registry!\nI'm blazing fast and memory-safe, as I'm written in Rust!"
}
