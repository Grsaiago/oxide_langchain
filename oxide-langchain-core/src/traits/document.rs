use serde_json::Value;
use uuid::Uuid;

pub trait Document {
    fn id(&self) -> Option<Uuid> {
        Some(Uuid::new_v4())
    }

    fn metadata(&self) -> Option<Value> {
        None
    }

    fn pagecontent(&self) -> Option<String> {
        None
    }
}