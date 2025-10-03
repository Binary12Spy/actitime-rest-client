/// Defines a trait for payloads with JSON serialization and deserialization capabilities.
pub trait PayloadTrait {
    /// Converts the payload to a JSON string.
    ///
    /// # Returns
    /// A JSON string representation of the payload.
    fn to_json(&self) -> String;

    /// Creates a payload instance from a JSON string.
    ///
    /// # Arguments
    /// * `json_str` - A JSON string representation of the payload.
    ///
    /// # Returns
    /// An instance of the payload.
    fn from_json(json_str: &str) -> Self;
}
