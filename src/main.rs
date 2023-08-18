mod api;
mod models;
mod repository;
mod tests;

#[macro_use]
extern crate rocket;

// This is based off of this tutorial:
// https://dev.to/hackmamba/build-a-rest-api-with-rust-and-mongodb-rocket-version-ah5

use std::str::FromStr;

// add import below
// use api::user_api::{create_user, get_user, update_user, delete_user, get_all_users}; //import the handler here
use api::order_api::{
    create_order, delete_order, get_account_orders, get_all_orders, get_buy_orders, get_order,
    get_sell_orders, update_order,
}; //import the handler here
use api::order_api::{hello, search};
use repository::mongodb_repo::MongoRepo;
use rocket::config;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header, Method, Status};
use rocket::{Request, Response};
use rocket_cors::{AllowedOrigins, CorsOptions}; // for CORS options

/// Necessary option to add to routes when frontend and backend are separated.
/// Needed to get requests from vue frontend
// fn configure_cors() -> rocket_cors::Cors {
//     // TODO: CHANGE THIS CORS OPTION TO BE MORE RESTRICTIVE
//     let allowed_origins = AllowedOrigins::all(); // Allow requests from any origin

//     rocket_cors::CorsOptions {
//         allowed_origins,
//         allowed_methods: rocket_cors::AllowedMethods::all().iter().cloned().collect(),
//         allowed_headers: rocket_cors::AllowedHeaders::all(),
//         ..Default::default()
//     }
//     .to_cors()
//     .expect("Failed to create CORS middleware")
// }

pub struct CORS;

/// Absolutely vital for configuration and separating front and backend
#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, PUT, GET, PATCH, OPTIONS, DELETE",
        ));
        // response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        // c.Writer.Header().Set("Access-Control-Allow-Headers", "Origin, Authorization, Content-Type, Accept")
        response.set_header(Header::new(
            "Access-Control-Allow-Headers",
            "Origin, Authorization, Content-Type, Accept, Authorization",
        ));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));

        // Handle the preflight request
        if _request.method() == Method::Options {
            response.set_status(Status::Ok);
        }
    }
}

#[launch]
fn rocket() -> _ {
    // let allowed_methods: rocket_cors::AllowedMethods = vec![Method::Get, Method::Post, Method::Patch]
    // .into_iter()
    // .collect();
    // let allowed_methods: rocket_cors::AllowedMethods = ["Get", "Post", "Delete"]
    // .iter()
    // .map(|s| FromStr::from_str(s).unwrap())
    // .collect();

    // let cors = CorsOptions::default()
    //     .allowed_origins(AllowedOrigins::all())
    //     .allowed_methods(allowed_methods
    //         // vec![Method::Get, Method::Post, Method::Patch]
    //         //     .into_iter()
    //             // .map(From::from)
    //             // .collect(),
    //     )
    //     .allow_credentials(true);

    //rocket::ignite().attach(cors.to_cors().unwrap());

    //------------------------------------------------------
    //User Story/TODOs
    //[X] - Migrate all Routes From Go over to Rust
    //[X] - Research the jobs api's we're pulling from (can find on discord)
    //[X] - test that can send and receive info from serpapi (google)
    //[] - have serpapi request be sent via json body instead of url params
    //[] - test that can send and receive info from jsearch
    //[] - write that information to an excel sheet
    //------------------------------------------------------

    //------------------------------------------------------
    //Optional TODOS
    //[] - have a request sent via url params to serp api, replacing _ or + with whitespace, so that way don't have to send json payload
    //------------------------------------------------------

    // create a connection to a db named "colony" and a collection named "market"
    //let db = MongoRepo::init("colony","market");
    rocket::build()
        //.manage(db)
        // .attach(CORS) here? This might be why I was getting a cors error forever ago
        // .mount("/", routes![create_order])
        // .mount("/", routes![get_order])
        // .mount("/", routes![update_order])
        // .mount("/", routes![delete_order])
        // .mount("/", routes![get_all_orders])
        // .mount("/", routes![get_account_orders])
        // .mount("/", routes![get_buy_orders])
        // .mount("/", routes![get_sell_orders])
        .mount("/", routes![hello])
        .mount("/", routes![search])
        .attach(CORS) // attach CORS options to allow frontend and backend to be separated
}
