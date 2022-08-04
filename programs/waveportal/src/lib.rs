use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod waveportal {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init,payer=user,space=1024+1024)]
    pub waves: Account<'info,Wave>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info,System>,
}

#[account]
pub struct Waves{
    pub waves_list:[Wave],
}

#[derive(Clone,Debug,AnchorSerialize,AnchorDeserialize,Copy,Default)]
pub struct Wave{
    pub waver: Pubkey,
    pub message: String,
    pub timestamp: i64,
}
