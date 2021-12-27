use anchor_lang::prelude::*;


declare_id!("BRgx1ATbc2rP9ge3k2RyCi3RWxNzbaqaveeSCeX8ZzhT");

#[program]
pub mod riddle_app {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, riddle: String, answer: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.riddle = riddle;
        base_account.answer = answer;
        Ok(())
    }

    pub fn update(ctx: Context<Update>, riddle: String, answer: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.riddle = riddle;
        base_account.answer = answer;
        Ok(())
    }

    // pub fn check(ctx: Context<Check>, answer: String) -> bool {
    //     let base_account = &ctx.accounts.base_account;

    //     match answer {
    //         base_account.answer => true,
    //         _ => false
    //     }

    // }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 64 + 64)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

//account to check user answer, wanted to seperate this from client side so it doesn't directly access the correct answer
// #[derive(Accounts)]
// pub struct Check<'info> {
//     #[account]
//     pub base_account: Account<'info, BaseAccount>,
// }

#[account]
pub struct BaseAccount {
    //pub data: String,
    pub riddle: String,
    pub answer: String,
}