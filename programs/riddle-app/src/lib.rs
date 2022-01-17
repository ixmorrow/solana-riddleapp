use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, MintTo, SetAuthority, Transfer, Mint};


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

    pub fn proxy_mint_to(ctx: Context<ProxyMintTo>, amount: u64) -> ProgramResult {
        token::mint_to(ctx.accounts.into(), amount)
    }

    pub fn proxy_transfer(ctx: Context<ProxyTransfer>, amount: u64) -> ProgramResult {
        token::transfer(ctx.accounts.into(), amount)
    }

    //could throw error here when someone tries to update without ansering correctly first
    pub fn update(ctx: Context<Update>, riddle: String, answer: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;

        if base_account.is_correct{
            base_account.riddle = riddle;
            base_account.answer = answer;
            base_account.is_correct = false;
        }

        Ok(())
    }

    //sends tokens to user when riddle is answered correctly
    //could burn one of your tokens for every wrong guess?????
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

#[derive(Accounts)]
pub struct ProxyMintTo<'info> {
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    pub mint: AccountInfo<'info>,
    #[account(mut)]
    pub to: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ProxyTransfer<'info> {
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    pub from: AccountInfo<'info>,
    #[account(mut)]
    pub to: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
}

/////////////////////////////////////////////////////////////////////////////////
impl<'a, 'b, 'c, 'info> From<&mut ProxyMintTo<'info>>
    for CpiContext<'a, 'b, 'c, 'info, MintTo<'info>>
{
    fn from(accounts: &mut ProxyMintTo<'info>) -> CpiContext<'a, 'b, 'c, 'info, MintTo<'info>> {
        let cpi_accounts = MintTo {
            mint: accounts.mint.clone(),
            to: accounts.to.clone(),
            authority: accounts.authority.clone(),
        };
        let cpi_program = accounts.token_program.clone();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

impl <'a, 'b, 'c, 'info> From<&mut ProxyTransfer<'info>> 
    for CpiContext<'a, 'b, 'c, 'info, Transfer<'info>>
{
    fn from(accounts: &mut ProxyTransfer<'info>) -> CpiContext<'a, 'b, 'c, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: accounts.from.clone(),
            to: accounts.to.clone(),
            authority: accounts.authority.clone(),
        };
        let cpi_program = accounts.token_program.clone();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

/////////////////////////////////////////////////////////////////////////////////