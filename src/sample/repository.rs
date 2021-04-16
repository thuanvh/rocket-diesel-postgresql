#![allow(proc_macro_derive_resolution_fallback)]
use diesel;
use diesel::prelude::*;

use crate::sample::model::License;
use crate::sample::model::NewLicense;
use crate::sample::model::Customer;
use crate::sample::model::NewCustomer;
use crate::sample::model::CustomerLicense;
use crate::sample::model::NewCustomerLicense;
use crate::schema::customer;
use crate::schema::customer::dsl::*;
use crate::schema::license;
use crate::schema::license::dsl::*;
use crate::schema::customer_license;
use crate::schema::customer_license::dsl::*;

pub fn create_customer(new_customer: NewCustomer, connection: &PgConnection) -> QueryResult<Customer> {
    diesel::insert_into(customer::table)
        .values(&new_customer)
        .get_result::<Customer>(connection)
}

pub fn show_customers(connection: &PgConnection) -> QueryResult<Vec<Customer>>  {
    //posts.filter(published.eq(true))
    
    //customer.limit(5)
    //    .load::<Customer>(&*connection)
    customer
        .load::<Customer>(&*connection)
}

pub fn get_customer(customer_id_var: i32, connection: &PgConnection) -> QueryResult<Customer> {
    customer::table.find(customer_id_var).get_result::<Customer>(connection)
}

pub fn update_customer(customer_id_var: i32, customer_var: Customer, connection: &PgConnection) -> QueryResult<Customer> {
    diesel::update(customer::table.find(customer_id_var))
        .set(&customer_var)
        .get_result::<Customer>(connection)
}

pub fn delete_customer(customer_id_var: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(customer::table.find(customer_id_var))
        .execute(connection)
}


pub fn create_license(new_license: NewLicense, connection: &PgConnection) -> QueryResult<License> {
    diesel::insert_into(license::table)
        .values(&new_license)
        .get_result::<License>(connection)
}

pub fn show_licenses(connection: &PgConnection) -> QueryResult<Vec<License>>  {
    //posts.filter(published.eq(true))
    
    //license.limit(5)
    //    .load::<License>(&*connection)
    license
        .load::<License>(&*connection)
}

pub fn get_license(license_id_var: i32, connection: &PgConnection) -> QueryResult<License> {
    license::table.find(license_id_var).get_result::<License>(connection)
}

pub fn update_license(license_id_var: i32, license_var: License, connection: &PgConnection) -> QueryResult<License> {
    diesel::update(license::table.find(license_id_var))
        .set(&license_var)
        .get_result::<License>(connection)
}

pub fn delete_license(license_id_var: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(license::table.find(license_id_var))
        .execute(connection)
}



pub fn create_customer_license(new_customer_license: NewCustomerLicense, conn: &PgConnection) -> QueryResult<CustomerLicense> {
    diesel::insert_into(customer_license::table)
        .values(&new_customer_license)
        .get_result::<CustomerLicense>(conn)
}

pub fn show_customer_licenses(connection: &PgConnection) -> QueryResult<Vec<CustomerLicense>>  {
    //posts.filter(published.eq(true))
    
    //license.limit(5)
    //    .load::<License>(&*connection)
    customer_license
        .load::<CustomerLicense>(&*connection)
}

pub fn get_customer_license(customer_license_id: i32, connection: &PgConnection) -> QueryResult<CustomerLicense> {
    customer_license::table.find(customer_license_id).get_result::<CustomerLicense>(connection)
}
pub fn get_customer_license_by_customer_id(customer_id_val: i32, connection: &PgConnection) -> QueryResult<Vec<CustomerLicense>>{
    customer_license::table.filter(customer_id.eq(customer_id_val)).load::<CustomerLicense>(*&connection)
}
pub fn get_customer_license_by_license_id(license_id_val: i32, connection: &PgConnection) -> QueryResult<Vec<CustomerLicense>> {
    customer_license::table.filter(license_id.eq(license_id_val)).load::<CustomerLicense>(*&connection)
}

pub fn update_customer_license(customer_license_id: i32, customer_license_var: CustomerLicense, connection: &PgConnection) -> QueryResult<CustomerLicense> {
    diesel::update(customer_license::table.find(customer_license_id))
        .set(&customer_license_var)
        .get_result::<CustomerLicense>(connection)
}

pub fn delete_customer_license(customer_license_id: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(customer_license::table.find(customer_license_id))
        .execute(connection)
}
