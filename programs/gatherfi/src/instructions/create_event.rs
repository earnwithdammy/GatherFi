use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint, Transfer};
use crate::state::*;
use crate::errors::GatherFiError;

#[derive(Accounts)]
pub struct CreateEvent<'info> {
    #[account(mut)]
    pub organizer: Signer<'info>,
    
    #[account(
        init,
        payer = organizer,
        space = 8 + Event::SIZE,
        seeds = [b"event", organizer.key().as_ref()],
        bump
    )]
    pub event: Account<'info, Event>,
    
    #[account(
        init,
        payer = organizer,
        space = 8 + Escrow::SIZE,
        seeds = [b"escrow", event.key().as_ref()],
        bump
    )]
    pub escrow: Account<'info, Escrow>,
    
    #[account(
        init,
        payer = organizer,
        space = 8 + ProfitPool::SIZE,
        seeds = [b"profits", event.key().as_ref()],
        bump
    )]
    pub profit_pool: Account<'info, ProfitPool>,
    
    #[account(
        init,
        payer = organizer,
        space = 8 + Budget::SIZE,
        seeds = [b"budget", event.key().as_ref()],
        bump
    )]
    pub budget: Account<'info, Budget>,
    
    #[account(
        init_if_needed,
        payer = organizer,
        space = 8 + 8,
        seeds = [b"event_counter"],
        bump
    )]
    pub event_counter: Account<'info, EventCounter>,
    
    pub system_program: Program<'info, System>,
}

#[account]
pub struct EventCounter {
    pub count: u64,
    pub bump: u8,
}

impl Event {
    pub const SIZE: usize = 32 + 256 + 256 + 1 + 8 + 8 + 8 + 8 + 8 + 4 + 4 + 8 + 256 + 64 + 64 + 64 + 1 + 1 + 1 + 1 + 1 + 4 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 32 + 32 + 32 + 1;
}

impl Escrow {
    pub const SIZE: usize = 32 + 8 + 8 + 8 + 1 + 1 + 1000 + 1 + 1 + 32 + 1 + 8 + 1;
}

impl ProfitPool {
    pub const SIZE: usize = 32 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 1 + 1 + 8 + 4 + 4 + 8 + 1;
}

impl Budget {
    pub const SIZE: usize = 32 + 32 + 2000 + 8 + 8 + 8 + 1 + 8 + 8 + 4 + 8 + 1 + 1 + 8 + 8 + 1;
}

pub fn handler(
    ctx: Context<CreateEvent>,
    name: String,
    description: String,
    target_amount: u64,
    ticket_price: u64,
    max_tickets: u32,
    event_date: i64,
    location: String,
    category: EventCategory,
) -> Result<()> {
    let event = &mut ctx.accounts.event;
    let escrow = &mut ctx.accounts.escrow;
    let profit_pool = &mut ctx.accounts.profit_pool;
    let budget = &mut ctx.accounts.budget;
    let event_counter = &mut ctx.accounts.event_counter;
    let clock = Clock::get()?;
    
    // Validate inputs
    require!(target_amount > 0, GatherFiError::InsufficientContribution);
    require!(ticket_price > 0, GatherFiError::InvalidTicketPrice);
    require!(max_tickets > 0, GatherFiError::TicketsSoldOut);
    require!(event_date > clock.unix_timestamp, GatherFiError::EventDatePassed);
    
    // Validate Nigerian location
    let (city, state) = validate_nigerian_location(&location)?;
    
    // Initialize event
    event.organizer = ctx.accounts.organizer.key();
    event.name = name;
    event.description = description;
    event.category = category;
    event.target_amount = target_amount;
    event.amount_raised = 0;
    event.min_contribution = 1000000; // 0.001 SOL minimum
    event.ticket_price = ticket_price;
    event.tickets_sold = 0;
    event.max_tickets = max_tickets;
    event.revenue_from_tickets = 0;
    event.event_date = event_date;
    event.location = location;
    event.city = city;
    event.state = state;
    event.country = "Nigeria".to_string();
    
    // Status flags
    event.is_active = true;
    event.is_funded = false;
    event.is_cancelled = false;
    event.is_paused = false;
    event.is_finalized = false;
    
    // Governance
    event.total_backers = 0;
    event.total_votes = 0;
    event.votes_for = 0;
    event.votes_against = 0;
    event.voting_ends_at = event_date - 86400; // 1 day before event
    
    // Timestamps
    event.created_at = clock.unix_timestamp;
    event.updated_at = clock.unix_timestamp;
    event.funding_deadline = clock.unix_timestamp + 30 * 86400; // 30 days
    
    // PDAs
    event.escrow = ctx.accounts.escrow.key();
    event.profit_pool = ctx.accounts.profit_pool.key();
    event.budget = ctx.accounts.budget.key();
    event.bump = ctx.bumps.event;
    
    // Initialize escrow
    escrow.event = event.key();
    escrow.total_amount = 0;
    escrow.released_amount = 0;
    escrow.balance = 0;
    escrow.milestone_count = 0;
    escrow.current_milestone = 0;
    escrow.milestones = Vec::new();
    escrow.is_locked = false;
    escrow.requires_approval = true;
    escrow.approvers = vec![event.organizer];
    escrow.approvals_needed = 1;
    escrow.created_at = clock.unix_timestamp;
    escrow.bump = ctx.bumps.escrow;
    
    // Initialize profit pool
    profit_pool.event = event.key();
    profit_pool.total_revenue = 0;
    profit_pool.other_revenue = 0;
    profit_pool.total_expenses = 0;
    profit_pool.platform_fee = 500; // 5% in basis points
    profit_pool.net_profit = 0;
    profit_pool.backer_share = 6000; // 60% in basis points
    profit_pool.organizer_share = 3500; // 35% in basis points
    profit_pool.platform_share = 500; // 5% in basis points
    profit_pool.is_calculated = false;
    profit_pool.is_distributed = false;
    profit_pool.distribution_date = None;
    profit_pool.backers_paid = 0;
    profit_pool.total_backers = 0;
    profit_pool.created_at = clock.unix_timestamp;
    profit_pool.bump = ctx.bumps.profit_pool;
    
    // Initialize budget
    budget.event = event.key();
    budget.organizer = event.organizer;
    budget.items = Vec::new();
    budget.total_amount = 0;
    budget.amount_spent = 0;
    budget.amount_remaining = 0;
    budget.is_approved = false;
    budget.votes_for = 0;
    budget.votes_against = 0;
    budget.total_voters = 0;
    budget.voting_ends_at = clock.unix_timestamp + 7 * 86400; // 7 days
    budget.is_locked = false;
    budget.is_completed = false;
    budget.created_at = clock.unix_timestamp;
    budget.updated_at = clock.unix_timestamp;
    budget.bump = ctx.bumps.budget;
    
    // Increment event counter
    event_counter.count += 1;
    event_counter.bump = ctx.bumps.event_counter;
    
    msg!(
        "🎉 Event created: {} in {}, Nigeria",
        event.name,
        event.city
    );
    msg!("📍 Location: {}", event.location);
    msg!("💰 Target: {} lamports", event.target_amount);
    msg!("🎟️  Ticket price: {} lamports", event.ticket_price);
    msg!("📅 Event date: {}", event.event_date);
    
    Ok(())
}