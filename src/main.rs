#[macro_use] // ensures we can use the macros in rocket
extern crate rocket;

mod auth;
mod models;
mod repositories;
mod schema;

use auth::BasicAuth;
use diesel::prelude::*;
use models::{NewRustacean, Rustacean};
use repositories::RustaceanRepository;
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
        let rustaceans = RustaceanRepository::find_multiple(c, 100).expect("DB error");
        json!(rustaceans)
    })
    .await
    // before creating separate file
    // db.run(|c| {
    //     let rustaceans = rustaceans::table
    //         .order(rustaceans::id.desc())
    //         .limit(1000)
    //         .load::<Rustacean>(c)
    //         .expect("DB error!");
    //     json!(rustaceans)
    // })
    // .await
}

#[get("/rustaceans/<id>")]
async fn view_rustacean(id: i32, _auth: BasicAuth, db: DbConn) -> Value {
    db.run(move |c| {
        let rustacean = RustaceanRepository::find(c, id)
            // pre repository addition
            // rustaceans::table
            //     .find(id)
            //     .get_result::<Rustacean>(c)
            .expect("DB error when selecting rustacean");
        json!(rustacean)
    })
    .await
}

#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
async fn create_rustacean(
    _auth: BasicAuth,
    db: DbConn,
    new_rustacean: Json<NewRustacean>,
) -> Value {
    db.run(|c| {
        let result = RustaceanRepository::create(c, new_rustacean.into_inner()) // calling into inner here, as opposed to in repository file
            // diesel::insert_into(rustaceans::table)
            //     .values(new_rustacean.into_inner())
            //     .execute(c)
            .expect("DB error when inserting ");
        json!(result)
    })
    .await
}

#[put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
async fn update_rustacean(
    id: i32,
    _auth: BasicAuth,
    db: DbConn,
    rustacean: Json<Rustacean>,
) -> Value {
    db.run(move |c| {
        let result = RustaceanRepository::save(c, id, rustacean.into_inner()) // calling into inner as rustacean coming in is a json
            // diesel::update(rustaceans::table.find(id))
            //     .set((
            //         rustaceans::name.eq(rustacean.name.to_owned()),
            //         rustaceans::email.eq(rustacean.email.to_owned()),
            //     ))
            // .execute(c)
            .expect("DB error when updating.");
        json!(result)
    })
    .await
}

#[delete("/rustaceans/<id>")]
async fn delete_rustacean(id: i32, _auth: BasicAuth, db: DbConn) -> status::NoContent {
    db.run(move |c| {
        RustaceanRepository::delete(c, id)
            // diesel::delete(rustaceans::table.find(id))
            //     .execute(c)
            .expect("DB error looking for rustacean to delete");
        status::NoContent
    })
    .await
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
