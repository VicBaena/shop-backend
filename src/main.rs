#[macro_use] extern crate rocket;

mod routes;
use crate::routes::get_routes::get::{index, salutation};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![salutation])
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::local::blocking::Client;
    use rocket::http::Status;
    use super::routes::get_routes::get;

    #[test]
    fn test_index(){
        let string_response = "Hello from the back side";
        let client =  Client::tracked(rocket()).expect("Valid rocket instance");
        let response = client.get(uri!(get::index)).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), string_response);
    }

    #[test]
    fn test_salutation(){
        let string_response = "another salutation";
        let client =  Client::tracked(rocket()).expect("Valid rocket instance");
        let response = client.get(uri!(get::salutation)).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), string_response);
    }
}