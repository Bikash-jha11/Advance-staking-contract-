use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct DataShape{
    pub owner:Pubkey,
    pub amount:u64,
    pub last_updated:u64,
    pub reward:u64,
}

