#[cfg(test)]
mod tests {
    use rocket::http::{ContentType, Status};
    use rocket::local::blocking::Client;

    #[test]
    fn test_home() {
        let rocket = crate::build_rocket();
        let client = Client::new(rocket).expect("valid rocket instance");
        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_profile() {
        let rocket = crate::build_rocket();
        let client = Client::new(rocket).expect("valid rocket instance");
        let response = client.get("/profile/test").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_login_logout() {
        let rocket = crate::build_rocket();
        let client = Client::new(rocket).expect("valid rocket instance");

        // Add a user to the DB for testing
        let db = client.rocket().state::<crate::db::Db>().expect("expected db in rocket state");
        db.init_test().unwrap();

        // Attempt to login
        let response = client
            .post("/login")
            .header(ContentType::Form)
            .body("username=test&password=test")
            .dispatch();
        assert_eq!(response.status(), Status::Ok); // Expect a redirect after successful login

        // Attempt to logout
        let response = client.get("/logout").dispatch();
        assert_eq!(response.status(), Status::SeeOther); // Expect a redirect after successful logout
    }
}
