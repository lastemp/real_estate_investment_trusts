//! CreateToken instruction handler

use {
    crate::{
        error::RealEstateInvestmentTrustsError,
        state::real_estate_investment_trust_scheme::RealEstateInvestmentTrustScheme,
    },
    anchor_lang::{prelude::*, system_program},
    anchor_spl::{
        associated_token,
        associated_token::AssociatedToken,
        token::{initialize_mint, mint_to, InitializeMint, Mint, MintTo, Token, TokenAccount},
    },
};

#[derive(Accounts)]
#[instruction(params: CreateTokenParams)]
pub struct CreateToken<'info> {
    #[account(mut,
        constraint = real_estate_investment_trust_scheme.is_initialized @ RealEstateInvestmentTrustsError::AccountNotInitialized
    )]
    pub real_estate_investment_trust_scheme: Account<'info, RealEstateInvestmentTrustScheme>,
    #[account(mut)]
    ///CHECK:
    pub token_account: AccountInfo<'info>,
    #[account(mut)]
    pub mint_token: Signer<'info>,
    // mut makes it changeble (mutable)
    #[account(mut)]
    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub associate_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateTokenParams {
    pub amount: u32,
}

pub fn create_token(ctx: Context<CreateToken>, params: &CreateTokenParams) -> Result<()> {
    msg!("Validate inputs");
    if params.amount == 0 {
        return Err(RealEstateInvestmentTrustsError::InvalidAmount.into());
    }

    //let sender = &ctx.accounts.owner;
    //let sender_tokens = &ctx.accounts.sender_tokens;
    //let recipient_tokens = &ctx.accounts.recipient_tokens;
    //let token_program = &ctx.accounts.token_program;
    let real_estate_investment_trust_scheme = &ctx.accounts.real_estate_investment_trust_scheme;
    let decimals = real_estate_investment_trust_scheme.decimals;
    let _amount = params.amount;

    let base: u32 = 10;
    let exponent = real_estate_investment_trust_scheme.decimals as u32;

    // lets get the amount in decimal format
    // 10 ** 9 * 3(base 10, 9 decimals, 3 amount), // 3 amount of token to transfer (in smallest unit i.e 9 decimals)
    let result = (base).pow(exponent);
    let _amount = (_amount as u64)
        .checked_mul(result as u64)
        .ok_or(RealEstateInvestmentTrustsError::InvalidArithmeticOperation)?;

    system_program::create_account(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::CreateAccount {
                from: ctx.accounts.owner.to_account_info(),
                to: ctx.accounts.mint_token.to_account_info(),
            },
        ),
        10_000_000,
        82,
        ctx.accounts.token_program.key,
    )?;

    initialize_mint(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            InitializeMint {
                mint: ctx.accounts.mint_token.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        ),
        decimals,
        ctx.accounts.owner.key,
        Some(ctx.accounts.owner.key),
    )?;

    associated_token::create(CpiContext::new(
        ctx.accounts.associate_token_program.to_account_info(),
        associated_token::Create {
            payer: ctx.accounts.owner.to_account_info(),
            associated_token: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.owner.to_account_info(),
            mint: ctx.accounts.mint_token.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
        },
    ))?;

    mint_to(
        CpiContext::new(
            ctx.accounts.token_account.to_account_info(),
            MintTo {
                authority: ctx.accounts.owner.to_account_info(),
                mint: ctx.accounts.mint_token.to_account_info(),
                to: ctx.accounts.token_account.to_account_info(),
            },
        ),
        _amount,
    )?;

    Ok(())
}
