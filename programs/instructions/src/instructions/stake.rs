use anchor_lang::prelude::*;
pub use crate::state::DataShape;

#[derive(Accounts)]
pub struct Stake<'info>{
    #[account(mut)]
    pub maker : Signer<'info>,

    #[account(
        init_if_needed,
        payer=maker,
        space= 8 + DataShape::INIT_SPACE,
        seeds=[b"user",maker.key().as_ref()],
        bump
    )]
    pub pda_account : Account<'info,DataShape>,

    pub system_program: Program<'info,System>
}


pub fn stake_ops(ctx : Context<Stake>,amount:u64) -> Result<()>{
     let pda_account = &mut ctx.accounts.pda_account;
     let clock = Clock::get()?;

  
     pda_account.amount += amount;
     pda_account.owner = ctx.accounts.maker.key(); 
     pda_account.last_updated = clock.unix_timestamp as u64;
     msg!("Success! Your total staked  tokens in account is {} .", amount);
     Ok(())
}