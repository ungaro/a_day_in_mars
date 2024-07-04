use anchor_lang::prelude::*;

declare_id!("6bkNVmmmwqMRxDUBubehXFsTb6ckgGnH1xE4UeQ25tQA");

#[program]
pub mod programs {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
