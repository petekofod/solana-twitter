use anchor_lang::prelude::*;

declare_id!("HDtAViACt1piWAfsaJXG4VMUtg3eKgxuCAz57hp3W4yc");

#[program]
pub mod solana_twitter {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
