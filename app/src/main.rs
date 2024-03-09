// Import the rocket macro
#[macro_use] // Enables the use of its macros
extern crate rocket;

// #[get] and #[launch] are macros in the rocket crate.

// Routes
#[get("/")] // An attribute that declares a route with the GET method and the “/” path.
fn index() -> &'static str {
    "Welcome to leseni!"
}

// Handlers
#[launch] // An attribute that marks the entry point of the Rocket application.
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}