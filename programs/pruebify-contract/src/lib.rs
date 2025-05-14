use anchor_lang::prelude::*;

declare_id!("5PDjJzo1RSFr4nEsS4aKrGooANcV9FWPYJspg2aLbgBm");

#[program]
pub mod pruebify_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
