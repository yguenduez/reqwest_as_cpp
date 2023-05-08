pub struct GoogleRequester;
impl GoogleRequester {
    pub fn fetch(&self) -> String {
        match ureq::get("https://www.google.com").call() {
            Ok(response) => response.into_string().expect("Could not transform response into string"),
            Err(err) => format!("{:?}", err)
        }
    }
}

mod tests {
    use crate::google_requester::GoogleRequester;

    #[test]
    fn make_request() {
        println!("{}", GoogleRequester {}.fetch())
    }
}
