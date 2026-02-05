use anchor_lang::prelude::*;

#[error_code]
pub enum GatherFiError {
    #[msg("Event is not active")]
    EventNotActive,
    
    #[msg("Funding target already reached")]
    TargetReached,
    
    #[msg("Insufficient funds contributed")]
    InsufficientContribution,
    
    #[msg("Event is already cancelled")]
    AlreadyCancelled,
    
    #[msg("Cannot cancel funded event")]
    CannotCancelFunded,
    
    #[msg("Ticket already checked in")]
    AlreadyCheckedIn,
    
    #[msg("Ticket already refunded")]
    AlreadyRefunded,
    
    #[msg("Event date has passed")]
    EventDatePassed,
    
    #[msg("Max tickets sold out")]
    TicketsSoldOut,
    
    #[msg("Not ticket owner")]
    NotTicketOwner,
    
    #[msg("Not event organizer")]
    NotOrganizer,
    
    #[msg("Budget not approved")]
    BudgetNotApproved,
    
    #[msg("Milestone amount exceeds budget")]
    MilestoneExceedsBudget,
    
    #[msg("Voting period ended")]
    VotingEnded,
    
    #[msg("Already voted")]
    AlreadyVoted,
    
    #[msg("No profits to distribute")]
    NoProfits,
    
    #[msg("Profits already distributed")]
    ProfitsDistributed,
    
    #[msg("Escrow is locked")]
    EscrowLocked,
    
    #[msg("Invalid Nigerian state")]
    InvalidNigerianState,
    
    #[msg("Event category not supported")]
    InvalidEventCategory,
    
    #[msg("Ticket type not available")]
    TicketTypeUnavailable,
    
    #[msg("Invalid profit distribution")]
    InvalidProfitDistribution,
    
    #[msg("Platform fee too high")]
    PlatformFeeTooHigh,
    
    #[msg("Not enough voting power")]
    InsufficientVotingPower,
    
    #[msg("Event is paused")]
    EventPaused,
    
    #[msg("Invalid Nigerian city")]
    InvalidNigerianCity,
    
    #[msg("Event already finalized")]
    AlreadyFinalized,
    
    #[msg("Invalid ticket price")]
    InvalidTicketPrice,
    
    #[msg("Not a backer")]
    NotBacker,
}