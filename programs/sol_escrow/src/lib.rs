use anchor_lang::prelude::*;

declare_id!("B5RqruQBkjhfvpkKafeRzY4ycwWeoTRwi6o6AxQB9sKu");

#[program]
pub mod sol_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
