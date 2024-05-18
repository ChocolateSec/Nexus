use rocket::routes;

pub mod modules;

pub use modules::get_module;
pub use modules::get_modules;

pub fn routes() -> Vec<rocket::Route> {
    routes![get_module, get_modules]
}
