//! RegisterInvestor instruction handler

use {
    crate::{error::RealEstateInvestmentTrustsError, state::investor::Investor},
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
#[instruction(params: RegisterInvestorParams)]
pub struct RegisterInvestor<'info> {
    // init means to create account
    // bump to use unique address for account
    #[account(
        init,
        payer = owner,
        space = 8 + Investor::INIT_SPACE,
        seeds = [b"investor", owner.key().as_ref()],
        bump
    )]
    pub investor: Account<'info, Investor>,
    // mut makes it changeble (mutable)
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct RegisterInvestorParams {
    full_names: String, // full names i.e first name, middlename, surname
    country: String,    // home country of tree owner
}

// full names length
const FULL_NAMES_LENGTH: usize = 50;
// country length
const COUNTRY_LENGTH: usize = 3;
const COUNTRY_LENGTH_2: usize = 2;

pub fn register_investor(
    ctx: Context<RegisterInvestor>,
    params: &RegisterInvestorParams,
) -> Result<()> {
    // validate inputs
    msg!("Validate inputs");
    if params.full_names.as_bytes().len() > 0
        && params.full_names.as_bytes().len() <= FULL_NAMES_LENGTH
    {
    } else {
        return Err(RealEstateInvestmentTrustsError::InvalidFullNamesLength.into());
    }

    if params.country.as_bytes().len() != COUNTRY_LENGTH
        && params.country.as_bytes().len() != COUNTRY_LENGTH_2
    {
        return Err(RealEstateInvestmentTrustsError::InvalidCountryLength.into());
    }

    let investor = &mut ctx.accounts.investor;

    // * - means dereferencing
    investor.owner = *ctx.accounts.owner.key;
    investor.full_names = params.full_names.to_string();
    investor.country = params.country.to_string();
    investor.active = true;

    Ok(())
}
