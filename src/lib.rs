mod google_requester;
use crate::google_requester::GoogleRequester;

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type GoogleRequester;
        fn fetch(&self) -> String;
    }

    extern "Rust" {
        fn new_requester() -> Box<GoogleRequester>;
    }
}

fn new_requester() -> Box<GoogleRequester> {
    Box::new(GoogleRequester)
}
