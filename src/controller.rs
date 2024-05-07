pub mod root;

use rocket::routes;

pub use root::get_index;

pub fn routes() -> Vec<rocket::Route> {
    routes![get_index]
}
