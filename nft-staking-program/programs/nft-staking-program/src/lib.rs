use anchor_lang::prelude::*;

declare_id!("GxEQegvykBHF8c2kFqsaNQr2UzDYybnndp8DtCmbNbeW");

#[program]
pub mod nft_staking_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
