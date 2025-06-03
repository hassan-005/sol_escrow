use anchor_lang::prelude::*;

#[instruction(seed: u64)]
pub struct Make<'info>{
    #[account(mut)]
    pub maker : Signer<'info>, 
    #[account(
        init,
        space = Escrow::InitSpace + Escrow::Discriminator.len(),
        seeds = [b"escrow", maker.key.as_ref(), seed.to_le_bytes().as_ref() ],
        bump
    )]
    pub escrow : Account<'info, Escrow>,

    #[account(
        mint::token_program = token_program
    )]
    pub mint_a: InterfaceAccount<'info, Mint>,

    #[account(
        mint::token_program = token_program
    )]
    pub mint_b: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = maker,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program
    )]
    pub vault : InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    



}


impl<'info> Make<'info>{
    fn populate_escrow(&mut self, seed:u64, amount: u64, bump: u8) -> Result<()>{
        self.escrow.set_inner(Escrow{
            seed,
            maker: self.maker.key(),
            mint_a: self.mint_a,
            mint_b: self.mint_b,
            recieve: amount,
            bump
        });
        Ok(())
    } 

    fn deposit_token_a(&self, amount:u64 ) -> Result<()> {
        transfer_checked(
            CpiContext::new(
                self.token_program.to_program_info(),
                TransferChecked{
                    from: self.maker_ata_a.to_account_info(),
                    mint: self.mint_a.to_account_info(),
                    to: self.vault.to_account_info(),
                    authority: self.maker.to_account_info()
            },
            ),
            amount,
            self.mint_a.decimals
        )?;

        Ok(())
    }
}

pub fn handler(ctx: Context<Make>, seed: u8 , recieve: u64, amount: u64, ) -> Result<()>{
    
    require!(recieve >= 0, EscrowError::InvalidAmount);      
    require!(amount >= 0, EscrowError::InvalidAmount);

    ctx.accounts.populate_escrow(seed, amount, ctx.bumps.escrow)?;
    
    ctx.accounts.deposit_token_a( amount)?;

    Ok(())
}