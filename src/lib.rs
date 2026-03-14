use runlicense_sdk::{verify_license, LicenseError};

/// A license-verified client. All crate functionality is accessed through this struct,
/// ensuring a valid license is required before any operations can be performed.
pub struct ProtectedClient {
    customer_id: String,
}

impl ProtectedClient {
    /// Create a new client after verifying the license.
    /// Returns an error if license verification fails.
    pub fn new() -> Result<Self, LicenseError> {
        let license = verify_license!("ok200paul/my-protected-crate")?;
        Ok(Self {
            customer_id: license.customer_id,
        })
    }

    /// Returns the licensed customer ID.
    pub fn customer_id(&self) -> &str {
        &self.customer_id
    }

    /// Example protected function — replace with your actual logic.
    pub fn do_something(&self) -> String {
        format!("Hello from licensed client ({})", self.customer_id)
    }
}
