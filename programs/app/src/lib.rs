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

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn increase_counter(ctx: Context<IncreaseCounter>) -> Result<()> {
        increase_counter::handler(ctx)
    }

    pub fn player_check_in

    pub fn join_team
}
