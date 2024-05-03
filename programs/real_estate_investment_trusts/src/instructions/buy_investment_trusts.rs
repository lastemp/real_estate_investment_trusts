//! BuyInvestmentTrusts instruction handler

use {
    crate::{
        error::RealEstateInvestmentTrustsError,
        state::{
            investor::Investor,
            real_estate_investment_trust_scheme::RealEstateInvestmentTrustScheme,
        },
    },
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        token::{transfer, Mint, Token, TokenAccount, Transfer},
    },
};

#[derive(Accounts)]
#[instruction(params: BuyInvestmentTrustsParams)]
pub struct BuyInvestmentTrusts<'info> {
    #[account(mut,
        constraint = real_estate_investment_trust_scheme.is_initialized @ RealEstateInvestmentTrustsError::AccountNotInitialized
    )]
    pub real_estate_investment_trust_scheme: Account<'info, RealEstateInvestmentTrustScheme>,
    #[account(mut,has_one = owner,
        constraint = investor.active @ RealEstateInvestmentTrustsError::InvalidInvestorStatus
    )]
    pub investor: Account<'info, Investor>,
    #[account(mut)]
    pub sender_tokens: Account<'info, TokenAccount>,
    #[account(mut)]
    pub recipient_tokens: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mint_token: Account<'info, Mint>,
    // mut makes it changeble (mutable)
    #[account(mut)]
    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub associate_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct BuyInvestmentTrustsParams {
    pub amount: u32,
}

pub fn buy_investment_trusts(
    ctx: Context<BuyInvestmentTrusts>,
    params: &BuyInvestmentTrustsParams,
) -> Result<()> {
    msg!("Validate inputs");
    if params.amount == 0 {
        return Err(RealEstateInvestmentTrustsError::InvalidAmount.into());
    }

    let sender = &ctx.accounts.owner;
    let sender_tokens = &ctx.accounts.sender_tokens;
    let recipient_tokens = &ctx.accounts.recipient_tokens;
    let token_program = &ctx.accounts.token_program;
    let real_estate_investment_trust_scheme = &mut ctx.accounts.real_estate_investment_trust_scheme;
    let investor = &mut ctx.accounts.investor;
    let unit_cost_of_investment_trusts: u32 =
        real_estate_investment_trust_scheme.unit_cost_of_investment_trusts;
    let investor_funds_raised = real_estate_investment_trust_scheme.investor_funds_raised;
    let total_units_investment_trusts: u32 = investor.total_units_investment_trusts;
    let available_funds: u32 = investor.available_funds;
    let decimals = real_estate_investment_trust_scheme.decimals as u64;
    let _amount = params.amount;

    /* // _amount is in decimal format hence need to convert to actual value
    let actual_amount = _amount
        .checked_div(decimals)
        .ok_or(RealEstateInvestmentTrustsError::InvalidArithmeticOperation)?; */

    // Get unit_investment_trusts from the product of unit_cost_of_investment_trusts and _amount
    let unit_investment_trusts = unit_cost_of_investment_trusts
        .checked_mul(_amount)
        .ok_or(RealEstateInvestmentTrustsError::InvalidArithmeticOperation)?;

    // Increment total_units_investment_trusts with new unit_investment_trusts
    investor.total_units_investment_trusts = total_units_investment_trusts
        .checked_add(unit_investment_trusts)
        .ok_or(RealEstateInvestmentTrustsError::InvalidArithmeticOperation)?;

    // Increment available_funds with new _amount
    investor.available_funds = available_funds
        .checked_add(_amount)
        .ok_or(RealEstateInvestmentTrustsError::InvalidArithmeticOperation)?;

    // Increment investor_funds_raised with new _amount
    real_estate_investment_trust_scheme.investor_funds_raised = investor_funds_raised
        .checked_add(_amount)
        .ok_or(RealEstateInvestmentTrustsError::InvalidArithmeticOperation)?;

    let base: i32 = 10;
    let exponent: i32 = real_estate_investment_trust_scheme.decimals as i32;

    // lets get the amount in decimal format
    // 10 ** 9 * 3(base 10, 9 decimals, 3 amount), // 3 amount of token to transfer (in smallest unit i.e 9 decimals)
    let result = (base as f64).powi(exponent.abs());
    let _amount = (_amount as u64)
        .checked_mul(result as u64)
        .ok_or(RealEstateInvestmentTrustsError::InvalidArithmeticOperation)?;

    transfer(
        CpiContext::new(
            token_program.to_account_info(),
            Transfer {
                from: sender_tokens.to_account_info(),
                to: recipient_tokens.to_account_info(),
                authority: sender.to_account_info(),
            },
        ),
        _amount,
    )?;

    Ok(())
}
