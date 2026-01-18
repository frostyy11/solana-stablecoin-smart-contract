use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, MintTo, Burn, Transfer};

declare_id!("11111111111111111111111111111111");

#[program]
pub mod stablecoin {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, decimals: u8) -> Result<()> {
        let stablecoin_state = &mut ctx.accounts.stablecoin_state;
        stablecoin_state.authority = ctx.accounts.authority.key();
        stablecoin_state.mint = ctx.accounts.mint.key();
        stablecoin_state.paused = false;
        stablecoin_state.total_minted = 0;
        stablecoin_state.total_burned = 0;
        
        msg!("Stablecoin initialized with {} decimals", decimals);
        Ok(())
    }

    pub fn mint_tokens(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
        let stablecoin_state = &mut ctx.accounts.stablecoin_state;
        
        require!(!stablecoin_state.paused, StablecoinError::ContractPaused);
        require!(
            ctx.accounts.authority.key() == stablecoin_state.authority,
            StablecoinError::Unauthorized
        );

        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        token::mint_to(cpi_ctx, amount)?;
        
        stablecoin_state.total_minted = stablecoin_state.total_minted
            .checked_add(amount)
            .ok_or(StablecoinError::MathOverflow)?;
        
        msg!("Minted {} tokens", amount);
        Ok(())
    }

    pub fn burn_tokens(ctx: Context<BurnTokens>, amount: u64) -> Result<()> {
        let stablecoin_state = &mut ctx.accounts.stablecoin_state;
        
        require!(!stablecoin_state.paused, StablecoinError::ContractPaused);

        let cpi_accounts = Burn {
            mint: ctx.accounts.mint.to_account_info(),
            from: ctx.accounts.from.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        token::burn(cpi_ctx, amount)?;
        
        stablecoin_state.total_burned = stablecoin_state.total_burned
            .checked_add(amount)
            .ok_or(StablecoinError::MathOverflow)?;
        
        msg!("Burned {} tokens", amount);
        Ok(())
    }

    pub fn transfer_tokens(ctx: Context<TransferTokens>, amount: u64) -> Result<()> {
        let stablecoin_state = &ctx.accounts.stablecoin_state;
        
        require!(!stablecoin_state.paused, StablecoinError::ContractPaused);

        let cpi_accounts = Transfer {
            from: ctx.accounts.from.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        token::transfer(cpi_ctx, amount)?;
        
        msg!("Transferred {} tokens", amount);
        Ok(())
    }

    pub fn pause(ctx: Context<AdminControl>) -> Result<()> {
        let stablecoin_state = &mut ctx.accounts.stablecoin_state;
        
        require!(
            ctx.accounts.authority.key() == stablecoin_state.authority,
            StablecoinError::Unauthorized
        );
        
        stablecoin_state.paused = true;
        msg!("Contract paused");
        Ok(())
    }

    pub fn unpause(ctx: Context<AdminControl>) -> Result<()> {
        let stablecoin_state = &mut ctx.accounts.stablecoin_state;
        
        require!(
            ctx.accounts.authority.key() == stablecoin_state.authority,
            StablecoinError::Unauthorized
        );
        
        stablecoin_state.paused = false;
        msg!("Contract unpaused");
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(decimals: u8)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + StablecoinState::INIT_SPACE,
        seeds = [b"stablecoin"],
        bump
    )]
    pub stablecoin_state: Account<'info, StablecoinState>,
    
    #[account(
        init,
        payer = authority,
        mint::decimals = decimals,
        mint::authority = authority,
    )]
    pub mint: Account<'info, Mint>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(
        mut,
        seeds = [b"stablecoin"],
        bump
    )]
    pub stablecoin_state: Account<'info, StablecoinState>,
    
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct BurnTokens<'info> {
    #[account(
        mut,
        seeds = [b"stablecoin"],
        bump
    )]
    pub stablecoin_state: Account<'info, StablecoinState>,
    
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct TransferTokens<'info> {
    #[account(
        seeds = [b"stablecoin"],
        bump
    )]
    pub stablecoin_state: Account<'info, StablecoinState>,
    
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct AdminControl<'info> {
    #[account(
        mut,
        seeds = [b"stablecoin"],
        bump
    )]
    pub stablecoin_state: Account<'info, StablecoinState>,
    
    pub authority: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct StablecoinState {
    pub authority: Pubkey,
    pub mint: Pubkey,
    pub paused: bool,
    pub total_minted: u64,
    pub total_burned: u64,
}

#[error_code]
pub enum StablecoinError {
    #[msg("Unauthorized access")]
    Unauthorized,
    #[msg("Contract is paused")]
    ContractPaused,
    #[msg("Math overflow")]
    MathOverflow,
}