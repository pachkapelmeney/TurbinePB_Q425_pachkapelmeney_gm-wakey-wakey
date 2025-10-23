use anchor_lang::prelude::*;
use crate::Counter;


#[derive(Accounts)]
pub struct IncreaseCounter<'info>{
    // pub counter: Counter<'info>;
}
