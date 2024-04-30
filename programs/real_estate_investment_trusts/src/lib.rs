//! real_estate_investment_trusts program entrypoint

pub mod error;
pub mod instructions;
pub mod state;

use {anchor_lang::prelude::*, instructions::*};

declare_id!("7xckh4Xhzk9DDB4BjQK7PUdJbVGLoA5RFpSdcxAMvhRW");

#[program]
pub mod real_estate_investment_trusts {
    use super::*;

    // admin instructions
    pub fn init(ctx: Context<Init>, params: InitParams) -> Result<()> {
        instructions::init(ctx, &params)
    }

    // public instructions
    pub fn register_investment_trust_scheme(
        ctx: Context<RegisterRealEstateInvestmentTrustScheme>,
        params: RegisterRealEstateInvestmentTrustSchemeParams,
    ) -> Result<()> {
        instructions::register_investment_trust_scheme(ctx, &params)
    }

    pub fn register_investor(
        ctx: Context<RegisterInvestor>,
        params: RegisterInvestorParams,
    ) -> Result<()> {
        instructions::register_investor(ctx, &params)
    }

    pub fn buy_investment_trusts(
        ctx: Context<BuyInvestmentTrusts>,
        params: BuyInvestmentTrustsParams,
    ) -> Result<()> {
        instructions::buy_investment_trusts(ctx, &params)
    }
}
