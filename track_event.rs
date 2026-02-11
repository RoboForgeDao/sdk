use anchor_lang::prelude::*;
use crate::state::session::*;

pub fn handler(ctx: Context<TrackEvent>, value: u64) -> Result<()> {
    let session = &mut ctx.accounts.session;
    session.counter += value;
    Ok(())
}

#[derive(Accounts)]
pub struct TrackEvent<'info> {
    #[account(mut)]
    pub session: Account<'info, Session>,
}
