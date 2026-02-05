use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint, MintTo};
use anchor_spl::associated_token::AssociatedToken;
use crate::state::*;
use crate::errors::GatherFiError;

#[derive(Accounts)]
pub struct MintTicket<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    
    #[account(
        mut,
        constraint = event.is_active @ GatherFiError::EventNotActive,
        constraint = !event.is_cancelled @ GatherFiError::AlreadyCancelled,
        constraint = event.is_funded @ GatherFiError::TargetReached,
        constraint = event.tickets_sold < event.max_tickets @ GatherFiError::TicketsSoldOut,
    )]
    pub event: Account<'info, Event>,
    
    #[account(
        init,
        payer = buyer,
        space = 8 + Ticket::SIZE,
        seeds = [b"ticket", event.key().as_ref()],
        bump
    )]
    pub ticket: Account<'info, Ticket>,
    
    #[account(
        init,
        payer = buyer,
        mint::decimals = 0,
        mint::authority = event,
        seeds = [b"ticket_mint", event.key().as_ref(), &[event.tickets_sold]],
        bump
    )]
    pub ticket_mint: Account<'info, Mint>,
    
    #[account(
        init,
        payer = buyer,
        associated_token::mint = ticket_mint,
        associated_token::authority = buyer,
    )]
    pub buyer_token_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        seeds = [b"ticket_counter", event.key().as_ref()],
        bump
    )]
    pub ticket_counter: Account<'info, TicketCounter>,
    
    #[account(mut)]
    pub profit_pool: Account<'info, ProfitPool>,
    
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl Ticket {
    pub const SIZE: usize = 32 + 32 + 32 + 4 + 1 + 64 + 64 + 1 + 1 + 1 + 8 + 8 + 8 + 32 + 256 + 1;
}

pub fn handler(
    ctx: Context<MintTicket>,
    ticket_type: TicketType,
    zone: String,
) -> Result<()> {
    let event = &mut ctx.accounts.event;
    let ticket = &mut ctx.accounts.ticket;
    let ticket_counter = &mut ctx.accounts.ticket_counter;
    let profit_pool = &mut ctx.accounts.profit_pool;
    let clock = Clock::get()?;
    
    // Validate ticket purchase
    require!(clock.unix_timestamp < event.event_date, GatherFiError::EventDatePassed);
    require!(!event.is_cancelled, GatherFiError::AlreadyCancelled);
    
    // Update ticket counter
    ticket_counter.count = ticket_counter.count.checked_add(1).unwrap();
    let ticket_number = ticket_counter.count;
    
    // Calculate price based on ticket type
    let price_multiplier = match ticket_type {
        TicketType::Regular => 100,
        TicketType::VIP => 200,
        TicketType::EarlyBird => 80,
        TicketType::Student => 60,
        TicketType::Group => 150, // Per person in group
        TicketType::VVIP => 300,
        TicketType::Backstage => 500,
        TicketType::Table => 1000,
    };
    
    let ticket_price = event.ticket_price
        .checked_mul(price_multiplier as u64)
        .unwrap()
        .checked_div(100)
        .unwrap();
    
    // Transfer payment from buyer to event
    let transfer_instruction = anchor_lang::system_program::Transfer {
        from: ctx.accounts.buyer.to_account_info(),
        to: ctx.accounts.profit_pool.to_account_info(),
    };
    
    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        transfer_instruction,
    );
    
    anchor_lang::system_program::transfer(cpi_context, ticket_price)?;
    
    // Initialize ticket
    ticket.mint = ctx.accounts.ticket_mint.key();
    ticket.event = event.key();
    ticket.owner = ctx.accounts.buyer.key();
    ticket.ticket_number = ticket_number;
    ticket.ticket_type = ticket_type;
    ticket.zone = zone;
    ticket.seat = None; // Can be assigned later
    ticket.is_checked_in = false;
    ticket.is_refunded = false;
    ticket.is_transferred = false;
    ticket.purchase_price = ticket_price;
    ticket.purchase_time = clock.unix_timestamp;
    ticket.checked_in_time = None;
    ticket.check_in_staff = None;
    
    // Generate metadata URI for Nigerian context
    let metadata_uri = format!(
        "https://ipfs.gatherfi.ng/tickets/{}/{}.json",
        event.key(),
        ticket_number
    );
    ticket.metadata_uri = metadata_uri;
    ticket.bump = ctx.bumps.ticket;
    
    // Update event
    event.tickets_sold = event.tickets_sold.checked_add(1).unwrap();
    event.revenue_from_tickets = event.revenue_from_tickets.checked_add(ticket_price).unwrap();
    event.updated_at = clock.unix_timestamp;
    
    // Update profit pool
    profit_pool.total_revenue = profit_pool.total_revenue.checked_add(ticket_price).unwrap();
    profit_pool.updated_at = clock.unix_timestamp;
    
    // Mint NFT ticket
    let cpi_accounts = MintTo {
        mint: ctx.accounts.ticket_mint.to_account_info(),
        to: ctx.accounts.buyer_token_account.to_account_info(),
        authority: ctx.accounts.event.to_account_info(),
    };
    
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
    
    token::mint_to(cpi_context, 1)?; // Mint 1 token
    
    msg!(
        "🎟️  Ticket #{} minted for {}",
        ticket_number,
        event.name
    );
    msg!("👤 Owner: {}", ctx.accounts.buyer.key());
    msg!("💰 Price: {} lamports", ticket_price);
    msg!("🎫 Type: {:?}", ticket_type);
    msg!("📍 Zone: {}", zone);
    
    Ok(())
}