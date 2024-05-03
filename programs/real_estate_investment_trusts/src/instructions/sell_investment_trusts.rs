//! SellInvestmentTrusts instruction handler

use {
    crate::{
        error::RealEstateInvestmentTrustsError,
        state::{
            deposit_base::DepositBase, investor::Investor,
            real_estate_investment_trust_scheme::RealEstateInvestmentTrustScheme,
        },
    },
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        token::{transfer_checked, Mint, Token, TokenAccount, TransferChecked},
    },
};

#[derive(Accounts)]
#[instruction(params: SellInvestmentTrustsParams)]
pub struct SellInvestmentTrusts<'info> {
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
    #[account(mut,
        constraint = deposit_account.is_initialized @ RealEstateInvestmentTrustsError::AccountNotInitialized
    )]
    pub deposit_account: Account<'info, DepositBase>,
    #[account(seeds = [b"auth", deposit_account.key().as_ref()], bump)]
    /// CHECK: no need to check this.
    pub pda_auth: UncheckedAccount<'info>,
    #[account(mut, seeds = [b"treasury-vault", pda_auth.key().as_ref()], bump)]
    pub treasury_vault: SystemAccount<'info>,
    // mut makes it changeble (mutable)
    #[account(mut)]
    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub associate_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct SellInvestmentTrustsParams {
    pub amount: u32,
}

pub fn sell_investment_trusts(
    ctx: Context<SellInvestmentTrusts>,
    params: &SellInvestmentTrustsParams,
) -> Result<()> {
    msg!("Validate inputs");
    if params.amount == 0 {
        return Err(RealEstateInvestmentTrustsError::InvalidAmount.into());
    }

    let real_estate_investment_trust_scheme = &mut ctx.accounts.real_estate_investment_trust_scheme;
    let investor = &mut ctx.accounts.investor;
    let sender_tokens = &ctx.accounts.sender_tokens;
    let recipient_tokens = &ctx.accounts.recipient_tokens;
    let mint_token = &ctx.accounts.mint_token;
    let deposit_account = &ctx.accounts.deposit_account;
    let pda_auth = &mut ctx.accounts.pda_auth;
    let treasury_vault = &mut ctx.accounts.treasury_vault;
    let token_program = &ctx.accounts.token_program;
    let unit_cost_of_investment_trusts: u32 =
        real_estate_investment_trust_scheme.unit_cost_of_investment_trusts;
    let investor_funds_raised = real_estate_investment_trust_scheme.investor_funds_raised;
    let total_units_investment_trusts: u32 = investor.total_units_investment_trusts;
    let available_funds: u32 = investor.available_funds;
    let decimals: u8 = real_estate_investment_trust_scheme.decimals;
    let _amount = params.amount;

    /* // _amount is in decimal format hence need to convert to actual value
    let actual_amount = _amount
        .checked_div(decimals as u64)
        .ok_or(RealEstateInvestmentTrustsError::InvalidArithmeticOperation)?; */

    // investor's available funds should exceed transfer amount
    if available_funds > _amount {
    } else {
        return Err(RealEstateInvestmentTrustsError::InsufficientFunds.into());
    }

    // Get unit_investment_trusts from the product of unit_cost_of_investment_trusts and actual_amount
    let unit_investment_trusts = unit_cost_of_investment_trusts
        .checked_mul(_amount)
        .ok_or(RealEstateInvestmentTrustsError::InvalidArithmeticOperation)?;

    // Deduct sold unit_investment_trusts from investor's total_units_investment_trusts
    investor.total_units_investment_trusts = total_units_investment_trusts
        .checked_sub(unit_investment_trusts)
        .ok_or(RealEstateInvestmentTrustsError::InvalidArithmeticOperation)?;

    // Deduct actual_amount(sold unit_investment_trusts) from investor's available funds
    investor.available_funds = available_funds
        .checked_sub(_amount)
        .ok_or(RealEstateInvestmentTrustsError::InvalidArithmeticOperation)?;

    // Deduct actual_amount(sold unit_investment_trusts) from investor_funds_raised
    real_estate_investment_trust_scheme.investor_funds_raised = investor_funds_raised
        .checked_sub(_amount)
        .ok_or(RealEstateInvestmentTrustsError::InvalidArithmeticOperation)?;

    /* let base: i32 = 10;
    let exponent: i32 = real_estate_investment_trust_scheme.decimals as i32; */
    let base: u32 = 10;
    let exponent = real_estate_investment_trust_scheme.decimals as u32;
    // lets get the amount in decimal format
    // 10 ** 9 * 3(base 10, 9 decimals, 3 amount), // 3 amount of token to transfer (in smallest unit i.e 9 decimals)
    //let result = (base as f64).powi(exponent.abs());
    let result = (base).pow(exponent);
    let _amount = (_amount as u64)
        .checked_mul(result as u64)
        .ok_or(RealEstateInvestmentTrustsError::InvalidArithmeticOperation)?;

    // Transfer funds from treasury vault to recipient
    let cpi_accounts = TransferChecked {
        from: sender_tokens.to_account_info(),
        mint: mint_token.to_account_info(),
        to: recipient_tokens.to_account_info(),
        authority: treasury_vault.to_account_info(),
    };

    let seeds = &[
        b"treasury-vault",
        pda_auth.to_account_info().key.as_ref(),
        &[deposit_account.admin_treasury_vault_bump.unwrap()],
    ];

    let signer = &[&seeds[..]];

    let cpi = CpiContext::new_with_signer(token_program.to_account_info(), cpi_accounts, signer);

    transfer_checked(cpi, _amount, decimals)?;

    Ok(())
}
