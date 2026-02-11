use anchor_lang::prelude::*;
use crate::state::config::*;

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    let cfg = &mut ctx.accounts.config;
    cfg.authority = *ctx.accounts.authority.key;
    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 32)]
    pub config: Account<'info, Config>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
