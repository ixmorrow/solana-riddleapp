use anchor_lang::prelude::*;


declare_id!("BRgx1ATbc2rP9ge3k2RyCi3RWxNzbaqaveeSCeX8ZzhT");

#[program]
pub mod riddleapp {
    
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, riddle: String, answer: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.riddle = riddle;
        base_account.answer = answer;
        base_account.is_correct = false;
        Ok(())
    }

    pub fn update(ctx: Context<Update>, riddle: String, answer: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;

        if base_account.is_correct{
            base_account.riddle = riddle;
            base_account.answer = answer;
            base_account.is_correct = false;
        }

        Ok(())
    }

    pub fn check(ctx: Context<Check>, answer: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;

        if base_account.answer == answer{
            base_account.is_correct = true;
        }

        Ok(())
    }
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
#[derive(Accounts)]
pub struct Check<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

#[account]
pub struct BaseAccount {
    //pub data: String,
    pub riddle: String,
    //change this to private once I'm done testing so that no one can access it and cheat
    answer: String,
    is_correct: bool,
}