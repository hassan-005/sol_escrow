use anchor_lang::prelude::*;

#[account (discriminator = [1])]
pub struct Escrow {
    pub seed:u64,
    pub maker:Pubkey,
    pub mint_a:Pubkey,
    pub mint_b:Pubkey,
    pub recieve:u64,
    pub bump:u8
}

impl Space for Escrow {
    // First 8 Bytes are Discriminator (u64)
    const INIT_SPACE: usize = 8 + 8 + 1 + 32 + 32 + 32 + 8;
}