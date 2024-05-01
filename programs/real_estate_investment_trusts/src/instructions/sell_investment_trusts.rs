//! SellInvestmentTrusts instruction handler

use {
    crate::{error::RealEstateInvestmentTrustsError, state::deposit_base::DepositBase},
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        token::{transfer_checked, Mint, Token, TokenAccount, TransferChecked},
    },
};

#[derive(Accounts)]
#[instruction(params: SellInvestmentTrustsParams)]
pub struct SellInvestmentTrusts<'info> {
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
    pub amount: u64,
}

pub fn sell_investment_trusts(
    ctx: Context<SellInvestmentTrusts>,
    params: &SellInvestmentTrustsParams,
) -> Result<()> {
    msg!("Validate inputs");
    if params.amount == 0 {
        return Err(RealEstateInvestmentTrustsError::InvalidAmount.into());
    }

    let sender_tokens = &ctx.accounts.sender_tokens;
    let recipient_tokens = &ctx.accounts.recipient_tokens;
    let mint_token = &ctx.accounts.mint_token;
    let deposit_account = &ctx.accounts.deposit_account;
    let pda_auth = &mut ctx.accounts.pda_auth;
    let treasury_vault = &mut ctx.accounts.treasury_vault;
    let token_program = &ctx.accounts.token_program;
    let _amount = params.amount;
    let _decimals: u8 = 9;

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

    transfer_checked(cpi, _amount, _decimals)?;

    Ok(())
}
