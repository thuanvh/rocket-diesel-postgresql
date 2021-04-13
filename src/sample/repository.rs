#![allow(proc_macro_derive_resolution_fallback)]
use diesel;
use diesel::prelude::*;

use crate::sample::model::License;
use crate::sample::model::NewLicense;
use crate::sample::model::CustomerLicense;
use crate::sample::model::NewCustomerLicense;
use crate::schema::license;
use crate::schema::license::dsl::*;
use crate::schema::customer_license;
use crate::schema::customer_license::dsl::*;

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

pub fn update_customer_license(customer_license_id: i32, customer_license_var: CustomerLicense, connection: &PgConnection) -> QueryResult<CustomerLicense> {
    diesel::update(customer_license::table.find(customer_license_id))
        .set(&customer_license_var)
        .get_result::<CustomerLicense>(connection)
}

pub fn delete_customer_license(customer_license_id: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(customer_license::table.find(customer_license_id))
        .execute(connection)
}
