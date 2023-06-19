#[macro_use] // ensures we can use the macros in rocket
extern crate rocket;

mod auth;
mod models;
mod schema;

use auth::BasicAuth;
use diesel::prelude::*;
use models::{NewRustacean, Rustacean};
use rocket::response::status;
use rocket::serde::json::{json, Json, Value}; // this json macros converts data to json file
use rocket_sync_db_pools::database;
use schema::rustaceans;

#[database("sqlite")]
struct DbConn(diesel::SqliteConnection);

#[get("/rustaceans")]
async fn get_rustaceans(_auth: BasicAuth, db: DbConn) -> Value {
    // json!([{"id": 1, "name":"John Doe"}, {"id" : 2, "name":"John Doe again"}])
    db.run(|c| {
        let rustaceans = rustaceans::table
            .order(rustaceans::id.desc())
            .limit(1000)
            .load::<Rustacean>(c)
            .expect("DB error!");
        json!(rustaceans)
    })
    .await
}

#[get("/rustaceans/<id>")]
fn view_rustacean(id: i32, _auth: BasicAuth) -> Value {
    json!({"id": id, "name":"John Doe", "email":"john@doe.com"})
}

#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
async fn create_rustacean(
    _auth: BasicAuth,
    db: DbConn,
    new_rustacean: Json<NewRustacean>,
) -> Value {
    db.run(|c| {
        let result = diesel::insert_into(rustaceans::table)
            .values(new_rustacean.into_inner())
            .execute(c)
            .expect("DB error when inserting ");
        json!(result)
    })
    .await
}

#[put("/rustaceans/<id>", format = "json")]
fn update_rustacean(id: i32, _auth: BasicAuth) -> Value {
    json!({"id": id, "name" : "Jon Doe", "email":"John@dow.com"})
}

#[delete("/rustaceans/<_id>")]
fn delete_rustacean(_id: i32, _auth: BasicAuth) -> status::NoContent {
    status::NoContent
}

#[catch(404)]
fn not_found() -> Value {
    json!("Not found!")
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            routes![
                get_rustaceans,
                view_rustacean,
                create_rustacean,
                update_rustacean,
                delete_rustacean
            ],
        )
        .register("/", catchers![not_found])
        .attach(DbConn::fairing())
        .launch()
        .await;
}
