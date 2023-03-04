use rocket::{
    http::Status,
    response::{self, content, status},
    serde::{json::Json, Deserialize},
};
use serde::Serialize;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}
#[get("/test")]
fn test() -> &'static str {
    panic!("test!")
}

#[catch(500)]
fn internal_error() -> &'static str {
    "Internal error, contact administrator 500"
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[derive(Debug)]
struct Task<'r> {
    description: &'r str,
    complete: bool,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct ValidationError<'r> {
    validation_error: &'r str,
}

#[post("/task", format = "json", data = "<task>")]
fn new_todo(
    task: Json<Task<'_>>,
) -> Result<status::Custom<Json<Task>>, status::BadRequest<Json<ValidationError>>> {
    println!("{task:?}",);

    match task.complete {
        false => {
            let response = Task {
                description: "Hardcoded response",
                complete: false,
            };

            Ok(status::Custom(Status::Created, Json(response)))
        }
        true => {
            let response = ValidationError {
                validation_error: "It is not possible to create task in status completed",
            };
            Err(status::BadRequest(Some(Json(response))))
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, test, hello, new_todo])
        .register("/", catchers![internal_error])
}
