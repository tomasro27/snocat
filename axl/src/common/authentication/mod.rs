//! Types supporting authentication of AXL tunnel connections
#[deny(unused_imports)]

mod traits;
pub use traits::*;

mod no_op_authentication;
pub use no_op_authentication::NoOpAuthenticationHandler;

mod simple_ack_authentication;
pub use simple_ack_authentication::SimpleAckAuthenticationHandler;

mod delegated_authentication;
pub use delegated_authentication::DelegatedAuthenticationHandler;