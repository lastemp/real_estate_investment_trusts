use crate::state::reits_type::ReitsType;
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct MarketIssuer {
    #[max_len(30)]
    pub issuer: String,
    #[max_len(30)]
    pub name: String,
    pub type_of_reit: u8, //ReitsType,
    #[max_len(20)]
    pub listing_date: String,
}
