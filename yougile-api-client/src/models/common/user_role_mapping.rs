use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Type for user-role mappings in Project and Department
/// Maps user IDs to their roles (worker, admin, observer for projects; manager, member for departments)
pub type UserRoleMapping = HashMap<String, String>;