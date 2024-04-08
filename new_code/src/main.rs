
//Import rocket implicitly for using macros globally
#[macro_use] extern crate rocket;
use rocket::{form::Form, fs::{FileServer, relative}, response::stream::{Event, EventStream}, serde::{Deserialize, Serialize}, tokio::sync::broadcast::{channel, error::RecvError, Sender}, State};
use rocket::tokio::select;
use rocket::Shutdown;

#[derive(Debug,Clone,FromForm, Serialize, Deserialize)]//Take for mdate and change to a message struct
#[serde(crate="rocket::serde")]

struct Message{
	#[field(validate = len(..30))]
	//room name can only be 29 characters long
	pub room: String,
	#[field(validate = len(..30))]
	pub username: String,
	pub message: String,
}

//Post messages
#[post("/message", data = "<form>")]
fn post(form:Form<Message>, queue: &State<Sender<Message>>){
	//It will fail if there are no active subscribers which is ok.
	let _res = queue.send(form.into_inner());
	
}

//Get request to events path
//Infinite stream of server sent events kind of like sockets but in one direction
#[get("/events")]
async fn events(queue: &State<Sender<Message>>,mut end: Shutdown) -> EventStream![] {
	//Get the receiver end of the channel
	let mut rx = queue.subscribe();
	//Infinite loop to keep the stream open
	EventStream! {
		loop{
			let msg = select!{
				//If we get a message from the channel
				msg = rx.recv() => match msg{
					//If the message is Ok then return the message
					Ok(msg) => msg,
					//If the message is an error then check if the channel is closed or lagged
					Err(RecvError::Closed) => break,
					//If the message is lagged then continue
					Err(RecvError::Lagged(_)) => continue,
				},
				//If we get a shutdown signal then break the loop
				_ = &mut end => break,
			};
			//Yield the message as a json event
			yield Event::json(&msg);
		}
	}
}

#[launch]
fn rocket() -> _ {
	rocket::build()
	//Adding state(channel sender end )
		.manage(channel::<Message>(1024).0)//get the first element of the tuple
        .mount("/", routes![post, events])
		//Handler to mount static files in a folder called static
		.mount("/", FileServer::from(relative!("frontend")))
}
