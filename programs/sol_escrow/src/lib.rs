use anchor_lang::prelude::*;

pub mod state;
pub mod errors;
pub mod instructions;

use instructions::*;

declare_id!("B5RqruQBkjhfvpkKafeRzY4ycwWeoTRwi6o6AxQB9sKu");

#[program]
pub mod sol_escrow {
    use super::*;
    pub fn make(ctx: Context<Make>, recieve:u64, amount:u64) -> Result<()> {
        let seed = ctx.accounts.escrow.seed;
        let _ = instructions::make::handler(ctx, &seed, recieve, amount)?;
        Ok(())
    }

    pub fn take(ctx: Context<Take>) -> Result<()>{
        let _ = instructions::take::handler(ctx);
        Ok(())
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        let _ = instructions::refund::handler(ctx);
        Ok(())
    }
    
}



