use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Mint, MintTo, Token, TokenAccount};

declare_id!("B9moipnMHzzHJN95x3TqX7A1sZ4wWKgP2Rwi4nLsXAjs");

#[program]
pub mod kozatsky_business {
    use super::*;

    pub fn initialize_player(ctx: Context<InitializePlayer>) -> Result<()> {
        let player = &mut ctx.accounts.player;
        player.owner = ctx.accounts.user.key();
        player.last_search_timestamp = 0;
        player.bump = ctx.bumps.player;
        Ok(())
    }

    pub fn search_resources(ctx: Context<SearchResources>, amount: u64) -> Result<()> {
        let player = &mut ctx.accounts.player;
        let now = Clock::get()?.unix_timestamp;

        require!(
            now - player.last_search_timestamp >= 60,
            ErrorCode::CooldownNotPassed
        );

        player.last_search_timestamp = now;

        let seeds: &[&[u8]] = &[b"mint-authority", &[ctx.bumps.mint_authority]];
        let signer = &[seeds];

        token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.resource_mint.to_account_info(),
                    to: ctx.accounts.user_resource_account.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info(),
                },
                signer,
            ),
            amount,
        )?;

        Ok(())
    }

    pub fn burn_resource(ctx: Context<BurnResource>, amount: u64) -> Result<()> {
        token::burn(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Burn {
                    mint: ctx.accounts.resource_mint.to_account_info(),
                    from: ctx.accounts.user_resource_account.to_account_info(),
                    authority: ctx.accounts.user.to_account_info(),
                },
            ),
            amount,
        )?;

        Ok(())
    }

    pub fn craft_item(ctx: Context<CraftItem>) -> Result<()> {
        require!(ctx.accounts.iron_account.amount >= 3, ErrorCode::NotEnoughResources);
        require!(ctx.accounts.wood_account.amount >= 1, ErrorCode::NotEnoughResources);
        require!(ctx.accounts.leather_account.amount >= 1, ErrorCode::NotEnoughResources);

        // burn ресурси
        token::burn(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Burn {
                    mint: ctx.accounts.iron_mint.to_account_info(),
                    from: ctx.accounts.iron_account.to_account_info(),
                    authority: ctx.accounts.user.to_account_info(),
                },
            ),
            3,
        )?;

        token::burn(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Burn {
                    mint: ctx.accounts.wood_mint.to_account_info(),
                    from: ctx.accounts.wood_account.to_account_info(),
                    authority: ctx.accounts.user.to_account_info(),
                },
            ),
            1,
        )?;

        token::burn(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Burn {
                    mint: ctx.accounts.leather_mint.to_account_info(),
                    from: ctx.accounts.leather_account.to_account_info(),
                    authority: ctx.accounts.user.to_account_info(),
                },
            ),
            1,
        )?;

        // mint NFT (1 штука)
        let seeds: &[&[u8]] = &[b"mint-authority", &[ctx.bumps.mint_authority]];
        let signer = &[seeds];

        token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.item_mint.to_account_info(),
                    to: ctx.accounts.user_item_account.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info(),
                },
                signer,
            ),
            1,
        )?;

        msg!("Item crafted + NFT minted!");

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializePlayer<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 32 + 8 + 1,
        seeds = [b"player", user.key().as_ref()],
        bump
    )]
    pub player: Account<'info, Player>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SearchResources<'info> {
    #[account(
        mut,
        seeds = [b"player", user.key().as_ref()],
        bump = player.bump
    )]
    pub player: Account<'info, Player>,

    pub user: Signer<'info>,

    #[account(mut)]
    pub resource_mint: Account<'info, Mint>,

    #[account(mut)]
    pub user_resource_account: Account<'info, TokenAccount>,

    /// CHECK: PDA authority
    #[account(seeds = [b"mint-authority"], bump)]
    pub mint_authority: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct BurnResource<'info> {
    #[account(mut)]
    pub resource_mint: Account<'info, Mint>,

    #[account(mut)]
    pub user_resource_account: Account<'info, TokenAccount>,

    pub user: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct CraftItem<'info> {
    #[account(mut)]
    pub iron_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub wood_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub leather_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub iron_mint: Account<'info, Mint>,

    #[account(mut)]
    pub wood_mint: Account<'info, Mint>,

    #[account(mut)]
    pub leather_mint: Account<'info, Mint>,

    #[account(mut)]
    pub item_mint: Account<'info, Mint>,

    #[account(mut)]
    pub user_item_account: Account<'info, TokenAccount>,

    /// CHECK: PDA authority
    #[account(seeds = [b"mint-authority"], bump)]
    pub mint_authority: UncheckedAccount<'info>,

    pub user: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

#[account]
pub struct Player {
    pub owner: Pubkey,
    pub last_search_timestamp: i64,
    pub bump: u8,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Cooldown 60 seconds not passed")]
    CooldownNotPassed,

    #[msg("Not enough resources")]
    NotEnoughResources,
}