mod command;
mod error;

pub mod ans_action;
pub mod msg;
pub mod raw_action;
#[cfg(feature = "testing")]
pub mod tests;

// Export interface for use in SDK modules
pub use abstract_adapter_utils::{coins_in_assets, cw_approve_msgs, Identify};
pub use command::{DexCommand, Fee, FeeOnInput, Return, Spread};
pub use error::DexError;

pub const DEX_ADAPTER_ID: &str = "xenosgeck:dex";
