use anchor_lang::prelude::*;

pub mod state;
pub mod errors;
pub mod instructions;

use instructions::make::handler;
use instructions::*;

declare_id!("B5RqruQBkjhfvpkKafeRzY4ycwWeoTRwi6o6AxQB9sKu");

#[program]
pub mod sol_escrow {
    use super::*;
    pub fn make(ctx: Context<Make>, recieve:u64, amount:u64) -> Result<()> {
        let seed = &ctx.accounts.escrow.seed;
        handler(ctx, &seed, recieve, amount)?;
        Ok(())
    }
    
}

#[derive(Accounts)]
pub struct Initialize {}

