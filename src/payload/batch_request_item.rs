use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

use crate::payload::PayloadTrait;
use crate::payload::enums::Method;
use crate::routes::Route;

/// Batch request item
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchRequestItem<BodyType, ResponseType>
where
    BodyType: Serialize + for<'de> Deserialize<'de>,
    ResponseType: Serialize + for<'de> Deserialize<'de>,
{
    /// Unique user identifier (read-only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// HTTP method of the request (e.g., GET, POST, PUT, DELETE)
    pub method: Method,
    /// Relative URL of the request
    pub relative_url: String,
    /// Date the project was created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_response_body: Option<bool>,
    /// Body of the request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<BodyType>,
    /// Placeholder for the response type
    #[serde(skip)]
    _response_type: PhantomData<ResponseType>,
}

// impl<BodyType, ResponseType> PayloadTrait for BatchRequestItem<BodyType, ResponseType>
// where
//     BodyType: Serialize + for<'de> Deserialize<'de>,
//     ResponseType: Serialize + for<'de> Deserialize<'de>,
// {
//     fn to_json(&self) -> String {
//         serde_json::to_string(self).unwrap()
//     }
//
//     fn from_json(json_str: &str) -> Self {
//         serde_json::from_str(json_str).unwrap()
//     }
// }

impl<BodyType, ResponseType> BatchRequestItem<BodyType, ResponseType>
where
    BodyType: Serialize + for<'de> Deserialize<'de>,
    ResponseType: Serialize + for<'de> Deserialize<'de>,
{
    /// Creates a new `BatchRequestItem` with the specified method and relative URL.
    ///
    /// # Arguments
    /// * `method` - The HTTP method for the request (e.g., GET, POST).
    /// * `relative_url` - The relative URL for the request.
    ///
    /// # Returns
    /// A new instance of `BatchRequestItem`.
    ///
    /// # Example
    /// ``` ignore
    /// let item: BatchRequestItem<YourBodyType, YourResponseType> = BatchRequestItem::new(Method::GET, "/your/relative/url".to_string());
    /// ```
    pub fn new(method: Method, relative_url: String) -> Self {
        Self {
            id: None,
            method,
            relative_url,
            include_response_body: None,
            body: None,
            _response_type: PhantomData,
        }
    }

    /// Sets the `id` field and returns the modified `BatchRequestItem`.
    ///
    /// # Arguments
    /// * `id` - The unique identifier for the batch request item.
    ///
    /// # Returns
    /// The modified `BatchRequestItem` with the updated `id`.
    pub fn with_id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    /// Sets the `include_response_body` field and returns the modified `BatchRequestItem`.
    ///
    /// # Arguments
    /// * `include_response_body` - A boolean indicating whether to include the response body.
    ///
    /// # Returns
    /// The modified `BatchRequestItem` with the updated `include_response_body`.
    pub fn with_include_response_body(mut self, include_response_body: bool) -> Self {
        self.include_response_body = Some(include_response_body);
        self
    }

    /// Sets the `body` field and returns the modified `BatchRequestItem`.
    ///
    /// # Arguments
    /// * `body` - The body of the request.
    ///
    /// # Returns
    /// The modified `BatchRequestItem` with the updated `body`.
    pub fn with_body(mut self, body: BodyType) -> Self {
        self.body = Some(body);
        self
    }
}

impl<PayloadType, ResponseType> From<Route<PayloadType, ResponseType>>
    for BatchRequestItem<PayloadType, ResponseType>
where
    PayloadType: Serialize + for<'de> Deserialize<'de>,
    ResponseType: Serialize + for<'de> Deserialize<'de>,
{
    fn from(route: Route<PayloadType, ResponseType>) -> Self {
        BatchRequestItem::new(route.method, route.relative_path)
    }
}
