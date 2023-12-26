
#[macro_use] extern crate rocket;
extern crate rocket_dyn_templates;

// use rocket::response::Redirect;
use rocket_dyn_templates::Template;
use rocket::response::Redirect;


#[derive(serde::Serialize)]
struct Message {
    user: &'static str,
    body: &'static str
}

#[derive(serde::Serialize)]
struct BoardContext {
    current_user: Option<String>,
    messages: Vec<Message>,
    parent: &'static str
}

#[derive(serde::Serialize)]
struct AboutContext {
    parent: &'static str
}


#[get("/world")]              // <- route attribute
fn world() -> &'static str {  // <- request handler
    "hello, world1!"
}

#[get("/")]
fn index() -> Redirect {
    Redirect::to("/user/anonymous")
}

#[get("/user/<username>")]
fn board(username: String) -> Template {
    Template::render("index", &BoardContext {
        current_user: Some(username),
        messages: vec![Message{user: "userA", body: "This is the first test message."},
                        Message{user: "userB", body: "This is the second test message."}],
        parent: "layout"
    })
}

#[get("/about")]
fn about() -> Template {
    Template::render("about", &AboutContext {
        parent: "layout"
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index,about,board])
    .mount("/hello", routes![world, about])
    .attach(Template::fairing())
}

// fn main() {
//     rocket::build().mount("/", routes![index]).launch();
// }
