//! RegisterRealEstateInvestmentTrustScheme instruction handler

use {
    crate::{
        error::RealEstateInvestmentTrustsError,
        state::{
            deposit_base::DepositBase, market_issuer::MarketIssuer,
            real_estate_investment_trust_scheme::RealEstateInvestmentTrustScheme,
            reits_type::ReitsType,
        },
    },
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
#[instruction(params: RegisterRealEstateInvestmentTrustSchemeParams)]
pub struct RegisterRealEstateInvestmentTrustScheme<'info> {
    // init means to create account
    // bump to use unique address for account
    #[account(
        init,
        payer = owner,
        space = 8 + RealEstateInvestmentTrustScheme::INIT_SPACE,
        seeds = [b"investment-trust-scheme", owner.key().as_ref()],
        bump
    )]
    pub real_estate_investment_trust_scheme: Account<'info, RealEstateInvestmentTrustScheme>,
    #[account(init, payer = owner, space = 8 + DepositBase::INIT_SPACE,
        constraint = !deposit_account.is_initialized @ RealEstateInvestmentTrustsError::AccountAlreadyInitialized
    )]
    pub deposit_account: Account<'info, DepositBase>,
    #[account(seeds = [b"auth", deposit_account.key().as_ref()], bump)]
    /// CHECK: no need to check this.
    pub pda_auth: UncheckedAccount<'info>,
    #[account(mut, seeds = [b"sol-vault", pda_auth.key().as_ref()], bump)]
    pub sol_vault: SystemAccount<'info>,
    // mut makes it changeble (mutable)
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct RegisterRealEstateInvestmentTrustSchemeParams {
    issuer: MarketIssuer, // market issuer details
    country: String,      // home country where trust scheme is implemented
}

// issuer length
const ISSUER_LENGTH: usize = 30;
// name length
const NAME_LENGTH: usize = 30;
// listing date length
const LISTING_DATE_LENGTH: usize = 20;
// country length
const COUNTRY_LENGTH: usize = 3;
const COUNTRY_LENGTH_2: usize = 2;

pub fn register_investment_trust_scheme(
    ctx: Context<RegisterRealEstateInvestmentTrustScheme>,
    params: &RegisterRealEstateInvestmentTrustSchemeParams,
) -> Result<()> {
    // validate inputs
    msg!("Validate inputs");
    if params.issuer.issuer.as_bytes().len() > 0
        && params.issuer.issuer.as_bytes().len() <= ISSUER_LENGTH
    {
    } else {
        return Err(RealEstateInvestmentTrustsError::InvalidIssuerLength.into());
    }

    if params.issuer.name.as_bytes().len() > 0 && params.issuer.name.as_bytes().len() <= NAME_LENGTH
    {
    } else {
        return Err(RealEstateInvestmentTrustsError::InvalidNameLength.into());
    }

    // 1 - DevelopmentRealEstateInvestmentTrusts i.e (D-REITs)
    // 2 - IncomeRealEstateInvestmentTrust i.e (I-REITs)

    let is_valid_reit = {
        match params.issuer.type_of_reit {
            1 | 2 => true,
            _ => false,
        }
    };

    if !is_valid_reit {
        return Err(RealEstateInvestmentTrustsError::InvalidTypeOfReit.into());
    }

    if params.issuer.listing_date.as_bytes().len() > 0
        && params.issuer.listing_date.as_bytes().len() <= LISTING_DATE_LENGTH
    {
    } else {
        return Err(RealEstateInvestmentTrustsError::InvalidListingDateLength.into());
    }

    if params.country.as_bytes().len() != COUNTRY_LENGTH
        && params.country.as_bytes().len() != COUNTRY_LENGTH_2
    {
        return Err(RealEstateInvestmentTrustsError::InvalidCountryLength.into());
    }

    let deposit_account = &mut ctx.accounts.deposit_account;
    let real_estate_investment_trust_scheme = &mut ctx.accounts.real_estate_investment_trust_scheme;

    // deposit account
    // * - means dereferencing
    deposit_account.owner = *ctx.accounts.owner.key;
    deposit_account.admin_auth_bump = ctx.bumps.pda_auth;
    deposit_account.admin_sol_vault_bump = Some(ctx.bumps.sol_vault);
    deposit_account.is_initialized = true;

    real_estate_investment_trust_scheme.owner = *ctx.accounts.owner.key;
    real_estate_investment_trust_scheme.issuer.issuer = params.issuer.issuer.to_string();
    real_estate_investment_trust_scheme.issuer.name = params.issuer.name.to_string();
    real_estate_investment_trust_scheme.issuer.type_of_reit = params.issuer.type_of_reit;
    real_estate_investment_trust_scheme.issuer.listing_date =
        params.issuer.listing_date.to_string();
    real_estate_investment_trust_scheme.country = params.country.to_string();
    real_estate_investment_trust_scheme.active = true;
    real_estate_investment_trust_scheme.is_initialized = true;

    Ok(())
}
