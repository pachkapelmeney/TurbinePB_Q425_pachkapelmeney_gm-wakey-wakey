use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Overflow! Too much!")]
    Overflow,

    #[msg("Already 3 players in a team!")]
    TooMuchPlayersInTeam,

    #[msg("Only player can add him self")]
    OnlyPlayerCanAddHimself


}
