use rocket;
use crate::connection;
use crate::sample;

pub fn create_routes() {
    rocket::ignite()
        .manage(connection::init_pool())
        .mount("/licenses",
               routes![
                    sample::handler::all_licenses,
                    sample::handler::create_license,
                    sample::handler::get_license,
                    sample::handler::update_license,
                    sample::handler::delete_license
                    ],
        )
        .mount("/customers",
               routes![
                    sample::handler::all_customers,
                    sample::handler::create_customer,
                    sample::handler::get_customer,
                    sample::handler::update_customer,
                    sample::handler::delete_customer
                    ],
        )
        .mount("/customer_licenses",
               routes![
                    sample::handler::all_customer_licenses,
                    sample::handler::create_customer_license,
                    sample::handler::get_customer_license,
                    sample::handler::update_customer_license,
                    sample::handler::delete_customer_license
                    ],
        ).launch();
}