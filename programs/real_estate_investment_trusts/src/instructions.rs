// admin instructions
pub mod init;

// public instructions
pub mod register_investment_trust_scheme;
pub mod register_investor;

// bring everything in scope
pub use {init::*, register_investment_trust_scheme::*, register_investor::*};
