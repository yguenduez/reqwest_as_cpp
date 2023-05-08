use rustls::ClientConfig;
use std::sync::Arc;
use ureq::Agent;

pub struct GoogleRequester {
    agent: Agent,
}
impl GoogleRequester {
    pub fn new() -> Self {
        let certs =
            rustls_native_certs::load_native_certs().expect("Could not fetch root certificates!");
        let mut root_cert_store = rustls::RootCertStore::empty();
        for cert in certs {
            root_cert_store
                .add(&rustls::Certificate(cert.0))
                .expect("Could not add root certs to the root cert store");
        }

        let client_config = ClientConfig::builder()
            .with_safe_defaults()
            .with_root_certificates(root_cert_store)
            .with_no_client_auth();
        let tls_config = Arc::new(client_config);
        let agent = ureq::builder().tls_config(tls_config).build();

        Self { agent }
    }

    pub fn fetch(&self) -> String {
        match self.agent.get("https://www.google.com").call() {
            Ok(response) => response
                .into_string()
                .expect("Could not transform response into string"),
            Err(err) => format!("{:?}", err),
        }
    }
}

mod tests {
    use crate::google_requester::GoogleRequester;

    #[test]
    fn make_request() {
        let requester = GoogleRequester::new();
        println!("{}", requester.fetch())
    }
}
