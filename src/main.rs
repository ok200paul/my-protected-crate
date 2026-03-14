use runlicense_sdk::{verify_license, LicenseError};

fn main() {
    match verify_license!("ok200paul/my-protected-crate") {
        Ok(license) => {
            println!("Licensed to: {}", license.customer_id);
        }
        Err(e) => {
            eprintln!("License verification failed: {e}");
            std::process::exit(1);
        }
    }
}
