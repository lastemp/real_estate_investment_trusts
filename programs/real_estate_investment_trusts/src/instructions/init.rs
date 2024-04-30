//! Init instruction handler

use {
    crate::{error::RealEstateInvestmentTrustsError, state::configs::InvestmentTrustsConfigs},
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
#[instruction(params: InitParams)]
pub struct Init<'info> {
    // init means to create account
    // bump to use unique address for account
    #[account(
        init,
        payer = owner,
        space = 8 + InvestmentTrustsConfigs::INIT_SPACE,
        constraint = !investment_trusts_configs.is_initialized @ RealEstateInvestmentTrustsError::AccountAlreadyInitialized,
        seeds = [b"investment-trusts-configs"],
        bump
    )]
    pub investment_trusts_configs: Account<'info, InvestmentTrustsConfigs>,
    // mut makes it changeble (mutable)
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitParams {
    pub is_initialized: bool,
}

pub fn init(ctx: Context<Init>, _params: &InitParams) -> Result<()> {
    msg!("Validate inputs");

    let investment_trusts_configs = &mut ctx.accounts.investment_trusts_configs;

    // investment trusts configs
    investment_trusts_configs.is_initialized = true;

    Ok(())
}
