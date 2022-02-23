#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[allow(unused)]
fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![hello])
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::http::Status;
    use rocket::local::Client;

    #[test]
    fn test_hello() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/").dispatch();

        println!("response: {:#?}", response.body_string());
        assert_eq!(response.status(), Status::Ok);
    }
}
