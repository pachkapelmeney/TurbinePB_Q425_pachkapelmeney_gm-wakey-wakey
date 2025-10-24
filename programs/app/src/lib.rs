pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("GBm1hAsWyBDfHmmpnb9sEBhwr6fJCGK2j3subz5zWqk5");
//reminder for myself: lib.rs exports onchain entrypoints & pick-ups handlers' results
#[program]
pub mod app {
    use super::*;

    // These are left just for demo purposes
    // pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    //     initialize_counter_legacy::handler(ctx)
    // }

    // pub fn increase_counter(ctx: Context<IncreaseCounter>) -> Result<()> {
    //     increase_counter::handler(ctx)
    // }

    // is the actual "counter increment"
    pub fn player_check_in(ctx: Context<PlayerCheckIn>, submited_wake_up_time_utc: u64) -> Result<()> {
        player_check_in::handler(ctx, submited_wake_up_time_utc)
    }

    pub fn join_team(ctx: Context<JoinTeam>) -> Result<()> {
        join_team::handler(ctx)
    }
}
