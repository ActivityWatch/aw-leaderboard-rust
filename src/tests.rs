#[cfg(test)]
mod tests {
    use rocket::http::{ContentType, Status};
    use rocket::local::blocking::Client;

    #[test]
    fn test_home() {
        let rocket = crate::rocket();
        let client = Client::tracked(rocket).expect("valid rocket instance");
        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_user_profile() {
        let rocket = crate::rocket();
        let client = Client::tracked(rocket).expect("valid rocket instance");
        let response = client.get("/user/test").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_user_login_logout() {
        let rocket = crate::rocket();
        let client = Client::tracked(rocket).expect("valid rocket instance");

        // Register a user
        let response = client.post("/register").header(ContentType::Form).body("username=testuser&email=testunit@example.com&password=test").dispatch();
        assert_eq!(response.status(), Status::SeeOther); // Expect a redirect after successful registration

        // Attempt to login
        let response = client
            .post("/login")
            .header(ContentType::Form)
            .body("username=test&password=test")
            .dispatch();
        assert_eq!(response.status(), Status::SeeOther); // Expect a redirect after successful login

        // Attempt to logout
        let response = client.get("/logout").dispatch();
        assert_eq!(response.status(), Status::SeeOther); // Expect a redirect after successful logout
    }
}
