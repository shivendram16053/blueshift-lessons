use anchor_lang::prelude::*;

declare_id!("22222222222222222222222222222222222222222222");

mod error;
mod instructions;
mod state;

use instructions::*;

#[program]
pub mod blueshift_anchor_escrow {
    use super::*;

    #[instruction(discriminator = 0)]
    pub fn make(ctx: Context<Make>, seed: u64, recieve: u64, amount: u64) -> Result<()> {
        make::handler(ctx, seed, recieve, amount)?;
        Ok(())
    }

    #[instruction(discriminator = 1)]
    pub fn take(ctx: Context<Take>) -> Result<()> {
        take::handler(ctx)?;
        Ok(())
    }

    #[instruction(discriminator = 2)]
    pub fn refund(ctx: Context<Refund>) -> Result<()> {

        refund::handler(ctx)?;
        Ok(())
    }
}
