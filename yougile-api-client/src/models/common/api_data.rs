use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Type for api_data in Company - arbitrary key-value data for development purposes
pub type ApiData = HashMap<String, serde_json::Value>;