//! BuyInvestmentTrusts instruction handler

use {
    crate::error::RealEstateInvestmentTrustsError,
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        token::{transfer, Mint, Token, TokenAccount, Transfer},
    },
};

#[derive(Accounts)]
#[instruction(params: BuyInvestmentTrustsParams)]
pub struct BuyInvestmentTrusts<'info> {
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
    pub amount: u64,
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
    let _amount = params.amount;

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
