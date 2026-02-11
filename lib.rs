use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod errors;

use instructions::*;

declare_id!("RoboF11111111111111111111111111111111111");

#[program]
pub mod roboforge {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn track_event(ctx: Context<TrackEvent>, value: u64) -> Result<()> {
        track_event::handler(ctx, value)
    }
}
