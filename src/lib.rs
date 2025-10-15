#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

extern crate serde_repr;
extern crate serde;
extern crate serde_json;
extern crate url;
extern crate reqwest;

pub mod apis;
pub mod models;

// Re-export API functions for convenience
pub use apis::auth::*;
pub use apis::boards::*;
pub use apis::columns::*;
pub use apis::tasks::*;
pub use apis::chats::*;
pub use apis::users::*;
pub use apis::projects::*;
pub use apis::departments::*;
pub use apis::stickers::*;
pub use apis::group_chats::*;
pub use apis::webhooks::*;
pub use apis::files::*;
