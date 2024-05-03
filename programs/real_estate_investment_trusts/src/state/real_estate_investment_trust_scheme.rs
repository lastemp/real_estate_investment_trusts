use crate::state::market_issuer::MarketIssuer;
use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct RealEstateInvestmentTrustScheme {
    pub owner: Pubkey, // publickey of the trust scheme promoter
    pub issuer: MarketIssuer,
    #[max_len(3)]
    pub country: String, // home country where trust scheme is implemented
    pub active: bool,               // status of trust scheme
    pub investor_funds_raised: u32, // funds raised by investors
    pub is_initialized: bool,       // is trust scheme initiated
    #[max_len(5)]
    pub investors: Vec<Pubkey>, // list of the investors
    pub unit_cost_of_investment_trusts: u32, // unit cost of investment trusts
    pub decimals: u8,               // decimals for the token mint
}
