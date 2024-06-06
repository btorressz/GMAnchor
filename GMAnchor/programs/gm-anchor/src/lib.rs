use anchor_lang::prelude::*;

declare_id!("5a4nuTa2oWgnfVmEDhv7deSf2QSuYGo5Usq7iQiWuv1h");

#[program]
pub mod gm_anchor {
    use super::*;

    pub fn say_gm(ctx: Context<SayGm>, name: String) -> Result<()> {
        let greeting_account = &mut ctx.accounts.greeting_account;
        greeting_account.name = name.clone();
        msg!("GM, {}!", name);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SayGm<'info> {
    #[account(init, payer = user, space = 8 + 40)]
    pub greeting_account: Account<'info, GreetingAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct GreetingAccount {
    pub name: String,
}
