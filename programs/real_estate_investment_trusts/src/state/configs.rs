use crate::state::market_issuer::MarketIssuer;
use anchor_lang::prelude::*;

#[account]
#[derive(Default, InitSpace)]
pub struct InvestmentTrustsConfigs {
    #[max_len(5)]
    pub issuers: Vec<MarketIssuer>,
    pub is_initialized: bool,
}
