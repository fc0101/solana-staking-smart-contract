use anchor_lang::prelude::*;

declare_id!("5DZyDdBj7cgqzJc8KuQjexxpY38uNLYm3uYbojGQXFdJ");

#[program]
pub mod solana_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
