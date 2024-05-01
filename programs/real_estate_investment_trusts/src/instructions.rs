// admin instructions
pub mod init;

// public instructions
pub mod buy_investment_trusts;
pub mod register_investment_trust_scheme;
pub mod register_investor;
pub mod sell_investment_trusts;

// bring everything in scope
pub use {
    buy_investment_trusts::*, init::*, register_investment_trust_scheme::*, register_investor::*,
    sell_investment_trusts::*,
};
