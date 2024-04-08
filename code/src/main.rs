//Import rocket implicitly for using macros globally
#[macro_use] extern crate rocket;

#[get("/home")]
fn home() -> &'static str {

}

#[launch]
fn rocket() -> _ {
	rocket::build()
		.manage(channel::)
}
