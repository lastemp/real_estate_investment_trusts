//! real_estate_investment_trusts program entrypoint

pub mod error;
pub mod instructions;
pub mod state;

use {anchor_lang::prelude::*, instructions::*};

declare_id!("FT1V2dLgjb2una5Xe2dDGc35PSDXMzjfwmPyxquFvZMW");

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
}
