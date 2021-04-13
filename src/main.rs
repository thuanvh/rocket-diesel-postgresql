// #![feature(proc_macro_hygiene, decl_macro)]

// #[macro_use] extern crate rocket;
// #[macro_use] extern crate rocket_contrib;
// #[macro_use] extern crate serde_derive;

// #[cfg(test)] mod tests;

// use std::sync::Mutex;

// use rocket::{Rocket, State, response::Debug};
// use rusqlite::{Connection, Error, types::ToSql};
// use std::{io, env};
// use rocket::data::{Data};
// use uuid::Uuid;
// use rocket::response::content;
// use rocket_contrib::json::{Json, JsonValue};
// use serde::Deserialize;

// mod sample;
// // type DbConn = Mutex<Connection>;

// // #[derive(Deserialize)]
// // struct Task {
// //   id: String,
// //   path: String,
// //   status: String, 
// // }

// // fn init_database(conn: &Connection) {
// //     // conn.execute("CREATE TABLE entries (
// //     //               id              INTEGER PRIMARY KEY,
// //     //               name            TEXT NOT NULL
// //     //               )", &[] as &[&dyn ToSql])
// //     //     .expect("create entries table");

// //     // conn.execute("INSERT INTO entries (id, name) VALUES ($1, $2)",
// //     //         &[&0 as &dyn ToSql, &"Rocketeer"])
// //     //     .expect("insert single entry into entries table");

// //     conn.execute("CREATE TABLE IF NOT EXISTS tasks( 
// //                   id varchar(50) primary key,
// //                   file varchar(255) not null,
// //                   status varchar(50) not null)",
// //                   &[] as &[&dyn ToSql])
// //         .expect("create tasks table");
// //     conn.execute("CREATE TABLE IF NOT EXISTS tasks_result(
// //                   id varchar(50) primary key,
// //                   result TEXT not null)"
// //                   ,&[] as &[&dyn ToSql])
// //                   .expect("create tasks table");

// // }

// // // #[get("/")]
// // // fn index() -> &'static str {
// // //     "Hello, world!"
// // // }

// // #[get("/list")]
// // fn hello(db_conn: State<'_, DbConn>) -> Result<String, Debug<Error>>  {
// //     db_conn.lock()
// //         .expect("db connection lock")
// //         .query_row("SELECT name FROM entries WHERE id = 0",
// //                    &[] as &[&dyn ToSql], |row| { row.get(0) })
// //         .map_err(Debug)
// // }

// // // #[launch]
// // // fn rocket() -> Rocket {
// // //     // Open a new in-memory SQLite database.
// // //     let conn = Connection::open_in_memory().expect("in memory db");

// // //     // Initialize the `entries` table in the in-memory database.
// // //     init_database(&conn);

// // //     // Have Rocket manage the database pool.
// // //     rocket::ignite()
// // //         .manage(Mutex::new(conn))
// // //         .mount("/", routes![hello])
// // // }

// // //use rocket::response::content;

// // // #[get("/")]
// // // fn json() -> content::Json<&'static str> {
// // //     content::Json("{ 'hi': 'world' }")
// // // }
// // #[post("/upload", data = "<data>")]
// // fn upload(db_conn: State<'_, DbConn>, data: Data) -> JsonValue {
// //     let file = env::temp_dir().join("upload.txt");
// //     let ret = data.stream_to_file(&file)
// //         .map(|n| n.to_string())
// //         .map_err(Debug);
// //     let uuid = create_task(db_conn, file.to_str().unwrap());
// //     //return Ok(uuid.to_simple().to_string());
// //     // let strvalue = format!("{{'id':'{}'}}",uuid.to_simple().to_string());
// //     // return content::Json(&strvalue.as_str());
// //     // let task = Task {
// //     //     id: uuid.to_simple().to_string()
// //     // };
// //     // Json(task)
// //     json!({"id":uuid.to_simple().to_string()})
// // }

// // fn create_task(db_conn: State<'_, DbConn>, file: &str) -> Uuid {
// //     let my_uuid = Uuid::new_v4();

// //     db_conn.lock()
// //         .expect("db connection lock")
// //         .execute("INSERT INTO tasks (id, file, status) VALUES ($1, $2, $3)",
// //              &[my_uuid.to_simple().to_string(), file.to_string() , (&"Created").to_string()])
// //          .expect("insert single entry into tasks table");
// //     return my_uuid;
// // }
// // fn check_task(db_conn: State<'_, DbConn>, uuid_str: &str) -> String {
// //     let my_uuid = Uuid::parse_str(uuid_str).unwrap();
// //     let status = db_conn.lock()
// //         .expect("db connection lock")

// //         .query_row("SELECT status FROM tasks WHERE id = $1",
// //                    &[my_uuid.to_simple().to_string()], |row| { row.get(0) })
// //         .expect("select single entry into tasks table");
// //     return status;
// // }
// // #[get("/status/<id>")]
// // fn status(db_conn: State<'_, DbConn>, id : String) -> JsonValue {
// //     json!({"status": check_task(db_conn, &id)})
// // }

// // #[get("/")]
// // fn index() -> &'static str {
// //     "Upload your text files by POSTing them to /upload."
// // }

// fn main() {
//     // // Open a new in-memory SQLite database.
//     // //let conn = Connection::open_in_memory().expect("in memory db");
//     // let conn = Connection::open("tasks.db").expect("open tasks db");
//     // // Initialize the `entries` table in the in-memory database.
//     // init_database(&conn);

//     // rocket::ignite()
//     //     .manage(Mutex::new(conn))
//     //     .mount("/", routes![index, hello, upload, status]).launch();

//     sample::router::create_routes();
// }





#![feature(decl_macro, proc_macro_hygiene)]
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate chrono;

use dotenv::dotenv;

mod sample;
mod schema;
mod connection;

fn main() {
    dotenv().ok();
    sample::router::create_routes();
}