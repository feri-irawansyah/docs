use async_graphql::{Scalar, ScalarType, Value, InputValueResult};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct MyDateTime(pub DateTime<Utc>);

#[Scalar]
impl ScalarType for MyDateTime {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::String(s) = &value {
            Ok(MyDateTime(DateTime::parse_from_rfc3339(s)?.with_timezone(&Utc)))
        } else {
            Err("Invalid DateTime format".into())
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.to_rfc3339())
    }
}
