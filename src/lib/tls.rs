use rustls::{ClientConfig, RootCertStore};
use rustls_native_certs::load_native_certs;

fn get_cert_store() -> RootCertStore {
    let mut root_certs = rustls::RootCertStore::empty();
    for cert in load_native_certs().expect("Failed to load system certificates") {
        root_certs.add(&rustls::Certificate(cert.0)).unwrap();
    }
    root_certs
}

pub fn get_client_config() -> ClientConfig {
    ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(get_cert_store())
        .with_no_client_auth()
}
