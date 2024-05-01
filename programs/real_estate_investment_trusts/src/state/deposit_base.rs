use anchor_lang::prelude::*;

#[account]
#[derive(Default, Debug, InitSpace)]
pub struct DepositBase {
    pub owner: Pubkey,
    pub admin_auth_bump: u8,
    pub admin_treasury_vault_bump: Option<u8>,
    pub is_initialized: bool,
}
