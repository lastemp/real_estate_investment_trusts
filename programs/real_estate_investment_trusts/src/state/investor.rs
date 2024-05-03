use anchor_lang::prelude::*;

#[account]
#[derive(Default, Debug, InitSpace)]
pub struct Investor {
    pub owner: Pubkey, // publickey of the investor
    #[max_len(50)]
    pub full_names: String, // full names i.e first name, middlename, surname
    #[max_len(3)]
    pub country: String, // home country of investor
    pub active: bool,  // status of investor
    pub total_units_investment_trusts: u32, // total units of investment trusts owned by investor
    pub available_funds: u32, // available funds equal to the investment trusts issued to investor
}
