use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Serialize, Deserialize)]
pub struct InternalCall {
    pub service: String,
    pub method: String,
    #[serde(serialize_with = "serialize_json_raw", deserialize_with = "deserialize_json_raw")]
    pub body: serde_json::Value,
}

pub enum ResponseStatusCode {
    Success,
    Failure,
}

pub struct Response<T> {
    pub status_code: ResponseStatusCode,
    pub status_message: String,
    pub body: T,
}


pub struct SetGRPCMappingRequest {
    pub http_uri: String,
    pub grpc_service: String,
    pub grpc_method: String,
}

pub struct GetGRPCMappingRequest {
    pub http_uri: String,
}

pub struct DeleteGRPCMappingRequest {
    pub http_uri: String,
}

pub struct GetGRPCMappingResponse {
    pub grpc_service: String,
    pub grpc_method: String,
}

// Custom serialization function for the JSON raw field
fn serialize_json_raw<S>(value: &serde_json::Value, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
{
    value.serialize(serializer)
}

// Custom deserialization function for the JSON raw field
fn deserialize_json_raw<'de, D>(deserializer: D) -> Result<serde_json::Value, D::Error>
    where
        D: Deserializer<'de>,
{
    serde_json::Value::deserialize(deserializer)
}

#[cfg(test)]
mod test_mod {
    use std::cell::RefCell;

    use super::*;

    #[derive(Serialize)]
    struct TestStruct {
        name: String,
    }

    #[test]
    fn test_serialize_json_raw() {
        let json_value = serde_json::to_value(&TestStruct {
            name: "test".to_string(),
        }).unwrap();
        let json_value_ref = RefCell::new(json_value);
        let internal_call = InternalCall {
            service: "test".to_string(),
            method: "test".to_string(),
            body: json_value_ref.take(),
        };
        let serialized = serde_json::to_string(&internal_call).unwrap();
        assert_eq!(serialized, r#"{"service":"test","method":"test","body":{"name":"test"}}"#);

        let deserialized: InternalCall = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.service, "test", "test service failed");
        assert_eq!(deserialized.method, "test", "test method failed");
        assert_eq!(deserialized.body, serde_json::to_value(&TestStruct {
            name: "test".to_string(),
        }).unwrap(), "test body failed");

        // let test_struct = deserialized.body.
        //     assert_eq!(test_struct.name, "test","test body failed");
    }
}
