use anchor_lang::prelude::*;


declare_id!("3bJ7o42f2bMQnFBdtDcudPo3oHCtGqg9HZKjvKc2HFgP");

pub mod instructions;
pub mod state;
pub use instructions::*;
pub use state::*;


#[program]
mod hello_anchor {
    use super::*;
    pub fn stake_amount(ctx: Context<Stake>,amount:u64) -> Result<()> {
       instructions::stake_ops(ctx,amount)?;

       Ok(())
    }
}

