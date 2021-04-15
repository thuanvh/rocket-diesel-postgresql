// #![feature(decl_macro, proc_macro_hygiene)]
// #[macro_use]
// extern crate diesel;
// extern crate dotenv;
// extern crate r2d2;
// extern crate r2d2_diesel;
// #[macro_use]
// extern crate rocket;
// extern crate rocket_contrib;
// #[macro_use]
// extern crate serde_derive;

use std::env;

use diesel::result::Error;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use crate::connection::DbConn;
use crate::sample;

use crate::sample::model::Customer;
use crate::sample::model::NewCustomer;
use crate::sample::model::License;
use crate::sample::model::NewLicense;
use crate::sample::model::CustomerLicense;
use crate::sample::model::NewCustomerLicense;

#[get("/")]
pub fn all_customers(connection: DbConn) -> Result<Json<Vec<Customer>>, Status> {
    sample::repository::show_customers(&connection)
        .map(|customer| Json(customer))
        .map_err(|error| error_status(error))
}

#[post("/", format ="application/json", data = "<new_customer>")]
pub fn create_customer(new_customer: Json<NewCustomer>, connection: DbConn) ->  Result<status::Created<Json<Customer>>, Status> {
    println!("here 0 {}",&new_customer.name);
    sample::repository::create_customer(new_customer.into_inner(), &connection)
        .map(|customer| customer_created(customer))
        .map_err(|error| error_status(error))
}
#[get("/<id>")]
pub fn get_customer(id: i32, connection: DbConn) -> Result<Json<Customer>, Status> {
    sample::repository::get_customer(id, &connection)
        .map(|customer| Json(customer))
        .map_err(|error| error_status(error))
}
#[put("/<id>", format = "application/json", data = "<customer>")]
pub fn update_customer(id: i32, customer: Json<Customer>, connection: DbConn) -> Result<Json<Customer>, Status> {
    sample::repository::update_customer(id, customer.into_inner(), &connection)
        .map(|customer| Json(customer))
        .map_err(|error| error_status(error))
}
#[delete("/<id>")]
pub fn delete_customer(id: i32, connection: DbConn) -> Result<status::NoContent, Status> {
    sample::repository::delete_customer(id, &connection)
        .map(|_| status::NoContent)
        .map_err(|error| error_status(error))
}


#[get("/")]
pub fn all_licenses(connection: DbConn) -> Result<Json<Vec<License>>, Status> {
    sample::repository::show_licenses(&connection)
        .map(|license| Json(license))
        .map_err(|error| error_status(error))
}

#[post("/", format ="application/json", data = "<new_license>")]
pub fn create_license(new_license: Json<NewLicense>, connection: DbConn) ->  Result<status::Created<Json<License>>, Status> {
    println!("here 0 {}",&new_license.name);
    sample::repository::create_license(new_license.into_inner(), &connection)
        .map(|license| license_created(license))
        .map_err(|error| error_status(error))
}
#[get("/<id>")]
pub fn get_license(id: i32, connection: DbConn) -> Result<Json<License>, Status> {
    sample::repository::get_license(id, &connection)
        .map(|license| Json(license))
        .map_err(|error| error_status(error))
}
#[put("/<id>", format = "application/json", data = "<license>")]
pub fn update_license(id: i32, license: Json<License>, connection: DbConn) -> Result<Json<License>, Status> {
    sample::repository::update_license(id, license.into_inner(), &connection)
        .map(|license| Json(license))
        .map_err(|error| error_status(error))
}
#[delete("/<id>")]
pub fn delete_license(id: i32, connection: DbConn) -> Result<status::NoContent, Status> {
    sample::repository::delete_license(id, &connection)
        .map(|_| status::NoContent)
        .map_err(|error| error_status(error))
}

// Customer License region
#[get("/")]
pub fn all_customer_licenses(connection: DbConn) -> Result<Json<Vec<CustomerLicense>>, Status> {
    sample::repository::show_customer_licenses(&connection)
        .map(|license| Json(license))
        .map_err(|error| error_status(error))
}

#[post("/", format ="application/json", data = "<new_customer_license>")]
pub fn create_customer_license(new_customer_license: Json<NewCustomerLicense>, connection: DbConn) ->  Result<status::Created<Json<CustomerLicense>>, Status> {
    println!("here 0 {}",&new_customer_license.customer_id);
    sample::repository::create_customer_license(new_customer_license.into_inner(), &connection)
        .map(|customer_license| customer_license_created(customer_license))
        .map_err(|error| error_status(error))
}
#[get("/<id>")]
pub fn get_customer_license(id: i32, connection: DbConn) -> Result<Json<CustomerLicense>, Status> {
    sample::repository::get_customer_license(id, &connection)
        .map(|customer_license| Json(customer_license))
        .map_err(|error| error_status(error))
}
#[put("/<id>", format = "application/json", data = "<customer_license>")]
pub fn update_customer_license(id: i32, customer_license: Json<CustomerLicense>, connection: DbConn) -> Result<Json<CustomerLicense>, Status> {
    sample::repository::update_customer_license(id, customer_license.into_inner(), &connection)
        .map(|customer_license| Json(customer_license))
        .map_err(|error| error_status(error))
}
#[delete("/<id>")]
pub fn delete_customer_license(id: i32, connection: DbConn) -> Result<status::NoContent, Status> {
    sample::repository::delete_customer_license(id, &connection)
        .map(|_| status::NoContent)
        .map_err(|error| error_status(error))
}

// Region of common functions

fn license_created(post: License) -> status::Created<Json<License>> {
    println!("here final");
    status::Created(
        format!("{host}:{port}/license/{id}", host = host(), port = port(), id = post.id).to_string(),
        Some(Json(post)))
}

fn customer_license_created(post: CustomerLicense) -> status::Created<Json<CustomerLicense>> {
    println!("here final");
    status::Created(
        format!("{host}:{port}/customerlicense/{id}", host = host(), port = port(), id = post.id).to_string(),
        Some(Json(post)))
}

fn customer_created(post: Customer) -> status::Created<Json<Customer>> {
    println!("here final");
    status::Created(
        format!("{host}:{port}/customer/{id}", host = host(), port = port(), id = post.id).to_string(),
        Some(Json(post)))
}

fn host() -> String {
    env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set")
}

fn port() -> String {
    env::var("ROCKET_PORT").expect("ROCKET_PORT must be set")
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}
