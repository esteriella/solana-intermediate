use anchor_lang::prelude::*;

declare_id!("aF5FEXUVz3fFyycXFZ3TaxcAqbHdejbVBk5xneDHcqA");

#[program]
pub mod staking_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
