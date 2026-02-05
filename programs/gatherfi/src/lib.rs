use anchor_lang::prelude::*;
use instructions::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("GATHRFi1111111111111111111111111111111111111");

#[program]
pub mod gatherfi {
    use super::*;

    // ========== EVENT MANAGEMENT (3) ==========
    pub fn create_event(
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
        instructions::create_event::handler(
            ctx,
            name,
            description,
            target_amount,
            ticket_price,
            max_tickets,
            event_date,
            location,
            category,
        )
    }

    pub fn update_event(
        ctx: Context<UpdateEvent>,
        name: Option<String>,
        description: Option<String>,
        event_date: Option<i64>,
        location: Option<String>,
    ) -> Result<()> {
        instructions::update_event::handler(ctx, name, description, event_date, location)
    }

    pub fn cancel_event(ctx: Context<CancelEvent>) -> Result<()> {
        instructions::cancel_event::handler(ctx)
    }

    // ========== CROWDFUNDING (3) ==========
    pub fn contribute(
        ctx: Context<Contribute>,
        amount: u64,
    ) -> Result<()> {
        instructions::contribute::handler(ctx, amount)
    }

    pub fn finalize_funding(ctx: Context<FinalizeFunding>) -> Result<()> {
        instructions::finalize_funding::handler(ctx)
    }

    pub fn refund_contribution(ctx: Context<RefundContribution>) -> Result<()> {
        instructions::refund_contribution::handler(ctx)
    }

    // ========== NFT TICKETING (4) ==========
    pub fn mint_ticket(
        ctx: Context<MintTicket>,
        ticket_type: TicketType,
        zone: String,
    ) -> Result<()> {
        instructions::mint_ticket::handler(ctx, ticket_type, zone)
    }

    pub fn transfer_ticket(
        ctx: Context<TransferTicket>,
        new_owner: Pubkey,
    ) -> Result<()> {
        instructions::transfer_ticket::handler(ctx, new_owner)
    }

    pub fn check_in(ctx: Context<CheckIn>) -> Result<()> {
        instructions::check_in::handler(ctx)
    }

    pub fn refund_ticket(ctx: Context<RefundTicket>) -> Result<()> {
        instructions::refund_ticket::handler(ctx)
    }

    // ========== BUDGET & GOVERNANCE (3) ==========
    pub fn submit_budget(
        ctx: Context<SubmitBudget>,
        budget_items: Vec<BudgetItem>,
        total_amount: u64,
    ) -> Result<()> {
        instructions::submit_budget::handler(ctx, budget_items, total_amount)
    }

    pub fn vote_on_budget(
        ctx: Context<VoteOnBudget>,
        approve: bool,
    ) -> Result<()> {
        instructions::vote_on_budget::handler(ctx, approve)
    }

    pub fn release_milestone(
        ctx: Context<ReleaseMilestone>,
        milestone_index: u8,
        amount: u64,
    ) -> Result<()> {
        instructions::release_milestone::handler(ctx, milestone_index, amount)
    }

    // ========== PROFIT DISTRIBUTION (3) ==========
    pub fn calculate_profits(ctx: Context<CalculateProfits>) -> Result<()> {
        instructions::calculate_profits::handler(ctx)
    }

    pub fn claim_profits(ctx: Context<ClaimProfits>) -> Result<()> {
        instructions::claim_profits::handler(ctx)
    }

    pub fn withdraw_fees(ctx: Context<WithdrawFees>) -> Result<()> {
        instructions::withdraw_fees::handler(ctx)
    }

    // ========== SECURITY & UTILITIES (3) ==========
    pub fn initialize_escrow(ctx: Context<InitializeEscrow>) -> Result<()> {
        instructions::initialize_escrow::handler(ctx)
    }

    pub fn verify_ownership(ctx: Context<VerifyOwnership>) -> Result<()> {
        instructions::verify_ownership::handler(ctx)
    }

    pub fn emergency_pause(ctx: Context<EmergencyPause>) -> Result<()> {
        instructions::emergency_pause::handler(ctx)
    }

    // ========== ADDITIONAL UTILITIES (3) ==========
    pub fn update_event_category(
        ctx: Context<UpdateEventCategory>,
        category: EventCategory,
    ) -> Result<()> {
        instructions::update_event_category::handler(ctx, category)
    }

    pub fn add_milestone(
        ctx: Context<AddMilestone>,
        description: String,
        amount: u64,
        due_date: i64,
    ) -> Result<()> {
        instructions::add_milestone::handler(ctx, description, amount, due_date)
    }

    pub fn claim_refund(ctx: Context<ClaimRefund>) -> Result<()> {
        instructions::claim_refund::handler(ctx)
    }
}