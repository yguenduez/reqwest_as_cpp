pub struct GoogleRequester;
impl GoogleRequester {
    pub fn fetch(&self) -> String {
        let body = reqwest::blocking::get("https://www.google.com")
            .expect("Could not sent request")
            .text()
            .expect("Could not transform to text")
            .to_string();
        body
    }
}

mod tests {
    use crate::google_requester::GoogleRequester;

    #[test]
    fn make_request() {
        println!("{}", GoogleRequester {}.fetch())
    }
}
