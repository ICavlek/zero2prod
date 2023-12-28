mod get;
mod post;
mod subscriptions_confirm;
pub use get::subscribe_form;
pub use post::{error_chain_fmt, subscribe};
pub use subscriptions_confirm::*;
