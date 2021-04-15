// #[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
// #[table_name = "posts"]
// pub struct Post {
//     pub id: i32,
//     pub title: String,
//     pub body: String,
//     pub published: bool,
// }
// #[derive(Insertable, Serialize, Deserialize)]
// #[table_name="posts"]
// pub struct NewPost {
//     pub title: String,
//     pub body: String,
// }

#![allow(proc_macro_derive_resolution_fallback)]
use crate::schema::license;
use crate::schema::customer;
use crate::schema::customer_license;

use chrono;
use std::string::String;
//use diesel::sql_types::Date;

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "customer"]
pub struct Customer {
    pub id: i32,
    pub name: String,
    pub address: Option<String>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[table_name = "customer"]
pub struct NewCustomer {
    pub name: String,
    pub address: Option<String>,
}


#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "license"]
pub struct License {
    pub id: i32,
    pub name: String,
    pub duration: i32,
    pub cpu: i32,
    pub storage: i32,
    pub number_of_nodes: i32,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[table_name = "license"]
pub struct NewLicense {
    pub name: String,
    pub duration: i32,
    pub cpu: i32,
    pub storage: i32,
    pub number_of_nodes: i32,
}

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "customer_license"]
pub struct CustomerLicense{
    pub id: i32,
    pub customer_id: i32,
    pub license_id: i32,
    pub license_code: String,
    pub active: bool,
    pub start_date: chrono::NaiveDateTime,
    pub end_date: chrono::NaiveDateTime,
    pub cpu: i32,
    pub storage: i32,
    pub number_of_nodes: i32,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[table_name = "customer_license"]
pub struct NewCustomerLicense{
    pub customer_id: i32,
    pub license_id: i32,
    pub license_code: String,
    pub active: bool,
    pub start_date: chrono::NaiveDateTime,
    pub end_date: chrono::NaiveDateTime,
    pub cpu: i32,
    pub storage: i32,
    pub number_of_nodes: i32,
}
