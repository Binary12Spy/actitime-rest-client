use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

use crate::payload::enums::Method;

pub struct Route<PayloadType, ResponseType>
where
    PayloadType: Serialize + for<'de> Deserialize<'de>,
    ResponseType: Serialize + for<'de> Deserialize<'de>,
{
    pub method: Method,
    pub relative_path: String,
    _payload_type: PhantomData<PayloadType>,
    _response_type: PhantomData<ResponseType>,
}

impl<PayloadType, ResponseType> Route<PayloadType, ResponseType>
where
    PayloadType: Serialize + for<'de> Deserialize<'de>,
    ResponseType: Serialize + for<'de> Deserialize<'de>,
{
    pub fn new(method: Method, relative_path: &str) -> Self {
        Self {
            method,
            relative_path: relative_path.to_string(),
            _payload_type: PhantomData,
            _response_type: PhantomData,
        }
    }
}
