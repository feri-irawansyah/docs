use async_graphql::{Scalar, ScalarType};

#[derive(Clone, Debug)]
pub struct MyDateTime(pub chrono::DateTime<chrono::Utc>);

#[Scalar]
impl ScalarType for MyDateTime {
    fn parse(value: async_graphql::Value) -> async_graphql::InputValueResult<Self> {
        if let async_graphql::Value::String(s) = &value {
            let dt = chrono::DateTime::parse_from_rfc3339(s)
                .map_err(|_| async_graphql::InputValueError::custom("Invalid DateTime"))?
                .with_timezone(&chrono::Utc);
            Ok(MyDateTime(dt))
        } else {
            Err(async_graphql::InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> async_graphql::Value {
        async_graphql::Value::String(self.0.to_rfc3339())
    }
}

impl<'de> serde::Deserialize<'de> for MyDateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = chrono::DateTime::parse_from_rfc3339(&s)
            .map_err(serde::de::Error::custom)?
            .with_timezone(&chrono::Utc);
        Ok(MyDateTime(dt))
    }
}

impl serde::Serialize for MyDateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0.to_rfc3339())
    }
}