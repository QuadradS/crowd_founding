use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("C71tuCNPkeoPXqVQcrt2EokX3V1KwiSDGD5nmWjCDqJV");

#[program]
pub mod crowd_founding {
    use super::*;

    pub fn initialize(ctx: Context<Create>, name: String, description: String) -> ProgramResult {
        let campaign = &mut ctx.accounts.campaign;

        campaign.name = name;
        campaign.description = description;
        campaign.amount_donated = 0;
        campaign.admin = *ctx.accounts.user.key;

        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> ProgramResult {
        let campaign = &mut ctx.accounts.campaign;
        let user = &mut ctx.accounts.user;

        if campaign.admin != *user.key {
            return Err(ProgramError::IncorrectProgramId);
        }

        let rent_balance = Rent::get()?.minimum_balance(campaign.to_account_info().data_len());

        if **campaign.to_account_info().lamports.borrow() - rent_balance < amount {
            return Err(ProgramError::InsufficientFunds);
        }

        **campaign.to_account_info().try_borrow_mut_lamports()? -= amount;
        **user.to_account_info().try_borrow_mut_lamports()? += amount;


        return Ok(());
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = user, space = 9000, seeds = [b"CAMPAIGN_DEMO".as_ref(), user.key.as_ref()], bump)]
    pub campaign: Account<'info, Campaign>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct Campaign {
    admin: Pubkey,
    name: String,
    description: String,
    amount_donated: u64,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    campaign: Account<'info, Campaign>,
    #[account(mut)]
    user: Signer<'info>,
}