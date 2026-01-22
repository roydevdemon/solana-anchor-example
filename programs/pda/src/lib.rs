use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

declare_id!("oPcDGLrzNmSfYd7kvWrgFtSpNsS7YpsM9oND3Pk3hxR");

#[program]
pub mod pda {
    use super::*;

    pub fn create(_ctx: Context<Create>, message: String) -> Result<()> {
        msg!("Create Message: {}", message);
        let account_data = &mut _ctx.accounts.message_account;
        account_data.user = _ctx.accounts.user.key();
        account_data.message = message; 
        account_data.bump = _ctx.bumps.message_account;

        Ok(())
    }

    pub fn update(_ctx: Context<Update>, message: String) -> Result<()> {
        msg!("Update Message: {}", message);
        let account_data = &mut _ctx.accounts.message_account;
        account_data.message = message;

        // transfer to vault account
        let transfer_accounts = Transfer {
            from: _ctx.accounts.user.to_account_info(),
            to: _ctx.accounts.vault_account.to_account_info(),
        };

        let cpi_context = CpiContext::new(
            _ctx.accounts.system_program.to_account_info(),
            transfer_accounts,
        ); 

        // transfer 1,000,000 lamports (0.001 SOL)
        transfer(cpi_context, 1_000_000)?;
        Ok(())
    }

    pub fn delete(_ctx: Context<Delete>) -> Result<()> {
        msg!("Delete Message");

        let user_key = _ctx.accounts.user.key();
        let signer_seeds: &[&[&[u8]]] = &[&[b"vault", user_key.as_ref(), &[_ctx.bumps.vault_account]]];

        let transfer_accounts = Transfer {
            from: _ctx.accounts.vault_account.to_account_info(),
            to: _ctx.accounts.user.to_account_info(),
        };

        let cpi_context = CpiContext::new(
            _ctx.accounts.system_program.to_account_info(),
            transfer_accounts
        ).with_signer(signer_seeds);

        transfer(cpi_context, _ctx.accounts.vault_account.lamports())?;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(message: String)]
pub struct Create<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        seeds = [b"message", user.key().as_ref()],
        bump,
        payer = user,
        space = 8 + 32 + 4 + message.len() + 1
    )]
    pub message_account: Account<'info, MessageAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(message: String)]
pub struct Update<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"message", user.key().as_ref()],
        bump = message_account.bump,
        realloc = 8 + 32 + 4 + message.len() + 1,
        realloc::payer = user,
        realloc::zero = true,
    )]
    pub message_account: Account<'info, MessageAccount>,
    pub system_program: Program<'info, System>,

    #[account(
        mut,
        seeds = [b"vault", user.key().as_ref()],
        bump,
    )]
    pub vault_account: SystemAccount<'info>,
}

#[derive(Accounts)]
pub struct Delete<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"message", user.key().as_ref()],
        bump = message_account.bump,
        close = user,
    )]
    pub message_account: Account<'info, MessageAccount>,

    #[account(
        mut,
        seeds = [b"vault", user.key().as_ref()],
        bump,
    )]
    pub vault_account: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct MessageAccount {
    pub user: Pubkey,
    pub message: String,
    pub bump: u8
}