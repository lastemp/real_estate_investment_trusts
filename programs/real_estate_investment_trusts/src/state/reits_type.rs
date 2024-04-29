use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, InitSpace)]
pub enum ReitsType {
    DevelopmentRealEstateInvestmentTrusts, // (D-REITs)
    IncomeRealEstateInvestmentTrust,       // (I-REITs)
    None,
}
