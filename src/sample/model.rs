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
use crate::schema::customer_license;

use chrono;
use std::string::String;
//use diesel::sql_types::Date;

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "license"]
pub struct License {
    pub id: i32,
    pub name: String,
    pub duration: i32,
    pub number_of_nodes: i32,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[table_name = "license"]
pub struct NewLicense {
    pub name: String,
    pub duration: i32,
    pub number_of_nodes: i32,
}

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "customer_license"]
pub struct CustomerLicense{
    pub id: i32,
    pub customer_name: String,
    pub address: Option<String>,
    pub license_id: i32,
    pub license_code: String,
    pub start_date: chrono::NaiveDateTime,
    pub end_date: chrono::NaiveDateTime,
    pub number_of_nodes: i32,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[table_name = "customer_license"]
pub struct NewCustomerLicense{
    pub customer_name: String,
    pub address: String,
    pub license_id: i32,
    pub license_code: String,
    pub start_date: chrono::NaiveDateTime,
    pub end_date: chrono::NaiveDateTime,
    pub number_of_nodes: i32,
}
