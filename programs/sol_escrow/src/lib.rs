use anchor_lang::prelude::*;

mod state;
mod errors;
mod instructions;

use instructions::*;

declare_id!("B5RqruQBkjhfvpkKafeRzY4ycwWeoTRwi6o6AxQB9sKu");

#[program]
pub mod sol_escrow {
    use super::*;

    
}

#[derive(Accounts)]
pub struct Initialize {}

