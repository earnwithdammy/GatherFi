use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::GatherFiError;

#[derive(Accounts)]
pub struct Contribute<'info> {
    #[account(mut)]
    pub contributor: Signer<'info>,
    
    #[account(
        mut,
        constraint = event.is_active @ GatherFiError::EventNotActive,
        constraint = !event.is_cancelled @ GatherFiError::AlreadyCancelled,
        constraint = !event.is_paused @ GatherFiError::EventPaused,
    )]
    pub event: Account<'info, Event>,
    
    #[account(
        init_if_needed,
        payer = contributor,
        space = 8 + Contribution::SIZE,
        seeds = [b"contribution", event.key().as_ref(), contributor.key().as_ref()],
        bump
    )]
    pub contribution: Account<'info, Contribution>,
    
    #[account(
        mut,
        seeds = [b"escrow", event.key().as_ref()],
        bump = event.bump
    )]
    pub escrow: Account<'info, Escrow>,
    
    pub system_program: Program<'info, System>,
}

impl Contribution {
    pub const SIZE: usize = 32 + 32 + 8 + 8 + 8 + 1 + 8 + 1;
}

pub fn handler(
    ctx: Context<Contribute>,
    amount: u64,
) -> Result<()> {
    let event = &mut ctx.accounts.event;
    let contribution = &mut ctx.accounts.contribution;
    let escrow = &mut ctx.accounts.escrow;
    let clock = Clock::get()?;
    
    // Validate contribution
    require!(amount >= event.min_contribution, GatherFiError::InsufficientContribution);
    require!(clock.unix_timestamp < event.funding_deadline, "Funding deadline passed");
    require!(!event.is_funded, GatherFiError::TargetReached);
    
    // Check if this is first contribution
    let is_new_contributor = contribution.amount == 0;
    
    // Update contribution
    contribution.contributor = ctx.accounts.contributor.key();
    contribution.event = event.key();
    contribution.amount = contribution.amount.checked_add(amount).unwrap();
    contribution.voting_power = contribution.amount; // 1 lamport = 1 vote
    contribution.claimed_profits = 0;
    contribution.claimed_refund = false;
    
    if is_new_contributor {
        contribution.created_at = clock.unix_timestamp;
        contribution.bump = ctx.bumps.contribution;
        
        // Increment backer count
        event.total_backers = event.total_backers.checked_add(1).unwrap();
    }
    
    // Update event
    event.amount_raised = event.amount_raised.checked_add(amount).unwrap();
    event.updated_at = clock.unix_timestamp;
    
    // Update escrow
    escrow.total_amount = escrow.total_amount.checked_add(amount).unwrap();
    escrow.balance = escrow.balance.checked_add(amount).unwrap();
    
    // Check if funding target reached
    if event.amount_raised >= event.target_amount {
        event.is_funded = true;
        msg!("🎯 Funding target reached for {}!", event.name);
    }
    
    // Transfer SOL to escrow
    let transfer_instruction = anchor_lang::system_program::Transfer {
        from: ctx.accounts.contributor.to_account_info(),
        to: ctx.accounts.escrow.to_account_info(),
    };
    
    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        transfer_instruction,
    );
    
    anchor_lang::system_program::transfer(cpi_context, amount)?;
    
    msg!(
        "✅ {} contributed {} lamports to {}",
        ctx.accounts.contributor.key(),
        amount,
        event.name
    );
    msg!("💰 Total raised: {} / {}", event.amount_raised, event.target_amount);
    msg!("🗳️  Voting power gained: {}", amount);
    
    Ok(())
}