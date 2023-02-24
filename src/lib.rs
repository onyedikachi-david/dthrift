//! # Implementation of an Osusu, Tanda, or Informal Loan Club Smart Contract
//!
//! Members can join the Osusu, Tanda, or Informal Loan Club by invoking
//! the join function and sending the specified amount in the specified currency.
//!
//! The smart contract keeps track of the contributions made by each member,
//! and the total amount of contributions. Each member is entitled to receive
//! a payout from the club's total contributions in each payout cycle.
//! The payout cycle is defined when initializing the contract and is set in days or weeks.
//!
//! Members can withdraw their contributions before the end of the payout
//! cycle, but they forfeit the right to receive future payouts from the club.
//!
//! At the end of each payout cycle, the smart contract initiates a payout
//! to a member based on a rotating algorithm that determines the next member
//! in line to receive a payout. The amount of the payout is equal to the
//! total amount of contributions divided by the number of payout cycles
//! and the number of members in the club.
//!
//! After the final payout cycle, any member can finalize the club
//! and receive the remaining balance of the club's contributions.
//! The smart contract transfers the balance to the account of the
//! member who finalizes the club. This can be done only once.
//!
//! Terminology: `Members` are the individuals who join the club
//! by invoking the join function and sending the specified amount
//! in the specified currency. `Contract` instances are created by
//! deploying a smart contract module and initializing it with the
//! payout cycle, amount of payout, and other parameters.

use concordium_std::{Duration, *};
use core::fmt::Debug;
use std::{collections::BTreeSet, ops::Add, time::Duration as STDDuration};
// use chrono::{DateTime, Duration, Utc};

#[derive(Serialize, SchemaType, Clone, Copy, Debug, PartialEq)]
pub enum TandaState {
    /// The Tanda is accepting new members.
    Open,
    /// The Tanda has reached its maximum number of members and is no longer
    /// accepting new members.
    Closed,
    /// The Tanda has not yet started the payout cycles.
    Pending,
    /// The Tanda is in progress and is currently paying out to members.
    InProgress,
    /// The Tanda has completed all payout cycles and is ready for finalization.
    Completed,
}
/// Your smart contract state.
// pub struct State {
//     // Your state
// name: String,
// description: String,
//     creator: AccountAddress,
//     members: Vec<AccountAddress>,
//     amount: u128,
//     collateral: u128,
//     max_members: u32,
//     purse_state: PurseState,
//     time_created: Timestamp,
//     time_interval: Timestamp,
//     end_time: Timestamp,
// }
#[derive(Debug, Serialize, SchemaType, Clone, PartialEq)]
pub struct State {
    /// The name of the Tanda or Osusu club
    name: String,
    /// A brief description of the Tanda club
    description: String,
    /// State of the Tanda
    tanda_state: TandaState,
    /// The creator of the Tanda club address
    creator: AccountAddress,
    /// The list of members who have joined the Tanda
    members: Option<Vec<(AccountAddress, u64)>>,
    /// The amount of money each member contributes to the Tanda
    contribution_amount: Amount,
    /// The penalty amount to paid in addition to the contribution amount.
    penalty_amount: Amount,
    /// The total amount of contributions made by all members
    total_contributions: Amount,
    /// The payout cycle for the Tanda
    payout_cycle: u64,
    /// The current payout cycle
    current_cycle: u64,
    /// The time when the Tanda started or will start
    start_time: Timestamp,
    /// The time when the Tanda will be finalized
    end_time: Timestamp,
    /// Payment interval for the Tanda club.
    time_interval: Duration,
    /// The member who is next in line to receive a payout
    next_receiver: Option<AccountAddress>,
    /// Last time withdrawal was made
    last_withdrawal_time: Timestamp,
    /// The list of accounts that have received payment after every cycle
    completed_cycles: Vec<(u64, Vec<AccountAddress>)>,
    /// The list of accounts that have made a contribution to the tanda
    contributors: BTreeSet<AccountAddress>,
    /// List of address that has withdrwan from the pot.
    withdrawn_addresses: BTreeSet<AccountAddress>,
    /// Withdrawal phase status
    withdrawal_phase_started: bool,
    /// The next withdrawal time.
    next_withdrawal_time: Timestamp,
    /// When withdrawal should start
    withdrawal_start_time: Timestamp,
    /// The maximum number of members allowed.
    max_contributors: u64,
    /// Index of users of members, just used to increment the member attribute index
    user_index: u64,
}
/// Your smart contract errors.
#[derive(Debug, PartialEq, Eq, Reject, Serial, SchemaType)]
enum Error {
    /// Failed parsing the parameter.
    #[from(ParseError)]
    ParseParamsError,
    /// Raised when the club is closed.
    TandaClosed,
    /// Raised when the club has reached its maximum member limit
    MaximumReached,
    /// Raised when an attempt is made to withdraw before the withdrawal time window.
    WithdrawalTimeNotReached,
    /// Raised when an attempt is made to start withdrawal phase when it's on already
    WithdrawalPhaseAlreadyStarted,
    /// Only account are allowed to join a club.
    ContractMember,
    /// Raised when the the total contributors isn't up to max_contibutors
    ContributorsNotComplete,
    /// The account is not authorized to perform the operation.
    Unauthorized,
    /// The Tanda club is already finalized.
    AlreadyFinalized,
    /// The Tanda club has not started yet.
    NotStarted,
    /// You are not Authorized to call this function.
    NotAuthorized,
    /// The Tanda club has already started.
    AlreadyStarted,
    /// The Tanda club is already finished.
    AlreadyFinished,
    /// The Tanda club has already been joined by the member.
    AlreadyJoined,
    /// The Tanda club is full and cannot accept new members.
    TandaFull,
    /// The member has not joined the Tanda club.
    NotJoined,
    /// The member has already made a contribution for the current cycle.
    AlreadyContributed,
    /// The address has already withdrawn from the box
    AlreadyWithdrawn,
    /// The address has not contributed before
    NotContributor,
    /// The member has missed the contribution deadline and has been penalized.
    Penalized,
    /// Raises when withdraw attempt is made before the interval.
    WithdrawalIntervalNotReached,
    /// The state is Invalid
    InvalidState,
    /// The contribution amount is invalid (e.g., zero or negative).
    InvalidContributionAmount,
    /// The payout cycle is invalid (e.g., zero or negative).
    InvalidPayoutCycle,
    /// The start time is invalid or in the past.
    InvalidStartTime,
    /// The end time is invalid or before the start time.
    InvalidEndTime,
    /// The time interval is invalid (e.g., zero or negative).
    InvalidTimeInterval,
    /// The penalty amount is invalid (e.g., zero or negative).
    InvalidPenaltyAmount,
    /// The maximum number of members is invalid (e.g., zero or negative).
    InvalidMaxContributors,
    /// The Tanda club name is invalid (e.g., empty or too long).
    InvalidName,
    /// The Tanda club description is invalid (e.g., empty or too long).
    InvalidDescription,
    /// The Tanda club creator is invalid (e.g., invalid account address).
    InvalidCreator,
    /// The Tanda club address is invalid (e.g., invalid account address).
    InvalidAddress,
    /// The amount to withdraw exceeds the Tanda pot.
    InsufficientBalance,
    /// The input parameter is invalid.
    InvalidParameter,
    /// An internal error occurred.
    InternalError,
}

// struct InitParameter {
//     name: String,
//     description: String,
//     creator: AccountAddress,
//     amount: u128,
//     collateral: u128,
//     max_members: u32,
//     time_interval: Timestamp,
// }
#[derive(Serialize, SchemaType, Clone, PartialEq)]
struct InitParameter {
    /// The name of the Tanda or Osusu club
    name: String,
    /// A brief description of the Tanda club
    description: String,
    /// The creator of the Tanda club address
    creator: AccountAddress,
    /// The amount of money each member contributes to the Tanda
    contribution_amount: Amount,
    /// The payout cycle for the Tanda
    payout_cycle: u64,
    /// The time when the Tanda will start using the RFC 3339 format (https://tools.ietf.org/html/rfc3339)
    start_time: Timestamp,
    /// The time when the Tanda will be finalized using the RFC 3339 format (https://tools.ietf.org/html/rfc3339)
    end_time: Timestamp,
    /// Payment interval for the Tanda club.
    time_interval: Duration,
    /// The penalty amount for missed payments
    penalty_amount: Amount,
    /// The maximum number of members allowed.
    max_contributors: u64,
}

#[derive(Serialize, SchemaType, Clone, PartialEq)]
pub struct JoinTandaParameter {
    penalty_amount: u64,
}

/// The event is logged when a new (or replacement) vote is cast by an account.
#[derive(Debug, Serialize, SchemaType)]
pub struct TandaEvent {
    /// The account that joined the Tanda.
    user: AccountAddress,
}

/// The event logged by this smart contract.
#[derive(Debug, Serial, SchemaType)]
pub enum Event {
    /// The event is logged when a new (or replacement) vote is cast by an
    /// account.
    Join(TandaEvent),
}

// Contract functions
/// Initialize the contract instance and start the Tanda.
/// A description, and other variables specified in the init struct`
/// have to be provided.
#[init(contract = "dthrift", parameter = "InitParameter")]
fn tanda_init<S: HasStateApi>(
    ctx: &impl HasInitContext,
    _state_builder: &mut StateBuilder<S>,
) -> InitResult<State> {
    let param: InitParameter = ctx.parameter_cursor().get()?;

    let account = ctx.init_origin();

    let now = ctx.metadata().slot_time();

    // let duration = Duration::from_millis(param.time_interval);

    // let now.duration_since(host.state().last_withdrawal_time).map_or(false, |dur| dur < host.state().time_interval)

    // let withdrawal_start_time =
    //     now.duration_since(param.start_time.into()) + param.time_interval.seconds().into() as u64;

    // let withdrawal_start_time = match now.duration_since(param.start_time.into()) {
    //     Some(duration) => duration + param.time_interval.seconds(),
    //     None => return Err(Error::InvalidState.into()),
    // }
    // .into_timestamp();

    let withdrawal_start_time = now
        .checked_add(param.time_interval.into())
        .ok_or(Error::InvalidState)?;
    // let test_duration = Duration::

    Ok(State {
        name: param.name,
        description: param.description,
        creator: account,
        tanda_state: TandaState::Open,
        members: None,
        contribution_amount: param.contribution_amount,
        penalty_amount: param.penalty_amount,
        total_contributions: concordium_std::Amount { micro_ccd: 0 },
        payout_cycle: param.payout_cycle,
        current_cycle: 0,
        start_time: param.start_time,
        end_time: param.end_time,
        last_withdrawal_time: Timestamp::from_timestamp_millis(0),
        next_withdrawal_time: Timestamp::from_timestamp_millis(0),
        withdrawal_start_time: withdrawal_start_time,
        time_interval: param.time_interval,
        next_receiver: None,
        completed_cycles: vec![],
        contributors: BTreeSet::new(),
        withdrawn_addresses: BTreeSet::new(),
        withdrawal_phase_started: false,
        max_contributors: param.max_contributors,
        user_index: 0,
    })
}

/// Enables a qualified user to join a Tanda club and pay penalty fee.
/// Adds a new member to the Tanda club and associates their address with a unique user index.
/// The user index is incremented each time a new member is added. If the maximum number of
/// contributors has already been reached, the function returns an error.
///
/// # Arguments
///
/// * ctx - The context of the current transaction.
/// * amount - The penalty amount.
///
/// # Errors
///
/// Returns an error if:
/// - It fails to parse the parameter.
/// - A contract tries to vote.
/// - The Tanda club has reached its maximum limit.
/// - The Tanda state is closed.
/// * The maximum number of contributors has already been reached.
///
#[receive(
    contract = "dthrift",
    name = "joinTanda",
    parameter = "JoinTandaParameter",
    error = "Error",
    mutable,
    enable_logger,
    payable
)]
fn join_tanda<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State, StateApiType = S>,
    amount: Amount,
    logger: &mut impl HasLogger,
) -> Result<(), Error> {
    // Check that the Tanda is still open
    ensure!(
        host.state().tanda_state == TandaState::Open,
        Error::TandaClosed
    );

    // Check if the Tanda is still open for new members to join.
    if host.state().end_time <= ctx.metadata().slot_time() {
        return Err(Error::TandaClosed);
    }

    // Check if the Tanda is not yet initialized.
    if host.state().start_time > ctx.metadata().slot_time() {
        return Err(Error::NotStarted);
    }

    // Check if the Tanda has reached its maximum limit.
    let members = &mut host.state().members.as_ref().map_or(0, |v| v.len());
    ensure!(
        *members as u64 == host.state().max_contributors,
        Error::MaximumReached
    );

    // Check if the contributor has already joined the Tanda.
    let contributor_address = ctx.invoker();
    if let Some(members) = &host.state().members {
        if members.iter().any(|(addr, _)| addr == &contributor_address) {
            return Err(Error::AlreadyJoined);
        }
    }

    // Check if the penalty amount is valid
    if amount != host.state().penalty_amount {
        return Err(Error::InvalidPenaltyAmount);
    }

    // Ensure that the sender is an account
    let acc = match ctx.sender() {
        Address::Account(acc) => acc,
        Address::Contract(_) => return Err(Error::ContractMember),
    };

    // Update penalty_amount
    let param: JoinTandaParameter = ctx.parameter_cursor().get()?;
    let penalty_amount = param.penalty_amount;
    host.state_mut().penalty_amount += concordium_std::Amount {
        micro_ccd: penalty_amount,
    };

    // Update the user_index count
    let new_user_index = host.state_mut().user_index + 1;
    host.state_mut().user_index = new_user_index;

    // Update the members list
    let new_user_address = acc;

    let new_member = (new_user_address, new_user_index);
    if let Some(mut members) = host.state_mut().members.take() {
        members.push(new_member);
        host.state_mut().members = Some(members.to_vec());
    } else {
        host.state_mut().members = Some(vec![new_member]);
    }

    //

    Ok(())
}

/// This function allows a member to contribute to the Tanda club.
/// The function checks that the member has already joined the
/// Tanda club, and the Tanda club is still open. If these
/// conditions are met, the function adds the contribution
/// amount to the total contributions, updates the member's
/// contribution, and schedules the next receiver of the Tanda payout.
///
/// # Arguments
///
/// * `ctx` - The context object that provides access to the current state and other data.
/// * `amount` - The amount of the contribution made by the member.
///
/// # Errors
///
/// This function will return an error if:
///
/// * The Tanda club is already closed.
/// * The maximum number of members has already been reached.
/// * The member has already joined the Tanda club.
/// * The contribution amount is less than the minimum required amount.
///
#[receive(
    contract = "dthrift",
    name = "contribute",
    parameter = "ContributeParameter",
    enable_logger,
    mutable,
    error = "Error",
    payable
)]
fn contribute<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State, StateApiType = S>,
    amount: Amount,
    logger: &mut impl HasLogger,
) -> Result<(), Error> {
    // Check that the contribution amount is greater than zero
    if amount <= (concordium_std::Amount { micro_ccd: 0 }) {
        return Err(Error::InvalidContributionAmount);
    }

    // Check that the contribution amount is equal to the set contribution amount
    let expected_contribution = host.state().contribution_amount;
    if amount != expected_contribution {
        return Err(Error::InvalidContributionAmount);
    }

    // Get the current time
    let current_time = ctx.metadata().slot_time();

    // Check that contributions are still allowed
    let start_time = host.state().start_time;
    if current_time < start_time {
        return Err(Error::NotStarted);
    }

    let end_time = host.state().end_time;
    if current_time > end_time {
        return Err(Error::TandaClosed);
    }

    // Check if the club is still open
    ensure!(
        host.state().tanda_state != TandaState::Closed,
        Error::TandaClosed
    );

    // Check that we haven't gotten to the end_time. If we have change the state to closed.

    // What if it is interval time?

    // Ensure that the sender is an account
    let acc = match ctx.sender() {
        Address::Account(acc) => acc,
        Address::Contract(_) => return Err(Error::ContractMember),
    };

    // Ensure that the address/account is a member; should join first+
    let sender_address = ctx.invoker();
    let existing_members = host.state_mut().members.take().unwrap_or_default();
    if existing_members
        .iter()
        .any(|(address, _)| address == &sender_address)
    {
        return Err(Error::NotJoined);
    }

    // Add to contributors set
    host.state_mut().contributors.insert(sender_address);
    // contributors.insert(sender_address);
    // host.state_mut().contributors = Some(contributors);

    // Increase the total_contributions
    let new_total_contributions = host.state_mut().total_contributions + amount;
    host.state_mut().total_contributions = new_total_contributions;

    Ok(())
}

/// Withdraws the current pot for the Tanda club.
///
/// # Arguments
///
/// * `ctx` - The context of the transaction.
///
/// # Errors
///
/// * `MemberNotFound` - When the account attempting to withdraw is not a member of the Tanda club.
/// * `TandaClosed` - When the Tanda club is not open for withdrawals.
///
#[receive(
    contract = "dthrift",
    name = "withdraw",
    parameter = "WithdrawParameter",
    enable_logger,
    mutable,
    error = "Error"
)]
fn withdraw<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State, StateApiType = S>,
    logger: &mut impl HasLogger,
) -> Result<(), Error> {
    // let host = host.state();

    // Get the current time.
    let now = ctx.metadata().slot_time();

    // Check if the current time is after the end time of the Tanda.
    if now >= host.state().end_time {
        return Err(Error::AlreadyFinalized);
    }

    // Check if the current time is before the next withdrawal time.
    // let time_since_last_withdrawal = now - host.state().last_withdrawal_time;
    // if time_since_last_withdrawal < host.state().time_interval {
    //     return Err(Error::WithdrawalTimeNotReached);
    // }

    // let now = now;
    // let time_since_last_withdrawal = now.duration_since(host.state().last_withdrawal_time);
    // if time_since_last_withdrawal < Some(host.state().time_interval.duration_between(host.state().time_interval)) {
    //     return Err(Error::WithdrawalTimeNotReached);
    // }

    // let now = now;
    // let time_since_last_withdrawal = now.duration_since(host.state().last_withdrawal_time);
    // if time_since_last_withdrawal < host.state().time_interval {
    //     return Err(Error::WithdrawalTimeNotReached);
    // }

    if now
        .duration_since(host.state().last_withdrawal_time)
        .map_or(false, |dur| dur < host.state().time_interval)
    {
        return Err(Error::WithdrawalTimeNotReached);
    }

    // Check if the club is closed
    if host.state().tanda_state == TandaState::Closed {
        return Err(Error::TandaClosed);
    }

    // Ensure that the sender is an account
    let acc = match ctx.sender() {
        Address::Account(acc) => acc,
        Address::Contract(_) => return Err(Error::ContractMember),
    };

    // Ensure that the address/account is a member; should join first+
    let sender_address = ctx.invoker();
    let existing_members = host.state_mut().members.take().unwrap_or_default();
    if existing_members
        .iter()
        .any(|(address, _)| address == &sender_address)
    {
        return Err(Error::NotJoined);
    }

    // If the address has not contributed, they cannot withdraw
    if !host.state().contributors.contains(&sender_address) {
        return Err(Error::NotContributor);
    }

    // Check if the sender has already withdrawn
    if host.state().withdrawn_addresses.contains(&sender_address) {
        return Err(Error::AlreadyWithdrawn);
    }

    // Add to withdrawn set
    host.state_mut().withdrawn_addresses.insert(sender_address);

    // Send total contribution amount to the address

    let total_contribution = host.state().total_contributions;
    host.invoke_transfer(&ctx.invoker(), total_contribution)
        .unwrap_abort();

    // Update the last withdrawal time.
    host.state_mut().last_withdrawal_time = now;
    Ok(())
}

/// This function starts the withdrawal phase for the Tanda club.
/// It checks if the Tanda club has reached its maximum number
/// of members and if all members have made a contribution.
/// It also checks if the current time is after the withdrawal
/// interval for the Tanda club. If these conditions are met,
/// the function changes the state of the Tanda club to Pending,
/// and schedules the first payout cycle by setting the first
/// receiver of the payout.
///
/// # Arguments
///
/// * ctx - The context object that provides access to the current state and other data.
///
/// # Errors
///
/// This function will return an error if:
///
/// * The Tanda club is already closed.
/// * The maximum number of members has not been reached yet.
/// * Not all members have made a contribution yet.
/// * The current time is before the withdrawal interval for the Tanda club.
#[receive(
    contract = "dthrift",
    name = "start_withdrawal_phase",
    enable_logger,
    mutable,
    error = "Error"
)]
fn start_withdrawal_phase<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State, StateApiType = S>,
    logger: &mut impl HasLogger,
) -> Result<(), Error> {
    // Ensure that the caller is the owner of the contract
    let caller = ctx.sender();
    let owner = host.state().creator;
    if caller != concordium_std::Address::Account(owner) {
        return Err(Error::NotAuthorized);
    }

    // Ensure that the withdrawal phase has not already started
    if host.state().withdrawal_phase_started {
        return Err(Error::WithdrawalPhaseAlreadyStarted);
    }

    // Ensure all members have contributed.
    if host.state().contributors.len() != host.state().max_contributors as usize {
        return Err(Error::ContributorsNotComplete);
    }

    // Ensure the current time is past the withdrawal interval.
    let now = ctx.metadata().slot_time();
    if now < host.state().withdrawal_start_time {
        return Err(Error::WithdrawalIntervalNotReached);
    }

    // Ensure the Tanda is in the InProgress state.
    if host.state().tanda_state != TandaState::InProgress {
        return Err(Error::InvalidState);
    }

    // Set the Tanda state to Pending.
    host.state_mut().tanda_state = TandaState::Pending;

    // set the next_withdrawal_time
    // let withdrawal_start_time = now
    //     .checked_add(host.state_mut().time_interval.into())
    //     .ok_or(Error::InvalidState)?;

    // Calculate the next withdrawal time.
    let withdrawal_interval: Duration = host.state().time_interval.into();
    let next_withdrawal_time =
        host.state().withdrawal_start_time.timestamp_millis() + withdrawal_interval.millis();
    host.state_mut().next_withdrawal_time = Timestamp::from_timestamp_millis(next_withdrawal_time);

    // Mark the withdrawal phase as started.
    host.state_mut().withdrawal_phase_started = true;
    Ok(())
}

// Withdraw penalty amount

// A function to Start a new contribution phase

/// View function that returns the content of the state.
#[receive(contract = "dthrift", name = "view", return_value = "State")]
fn view<'b, S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    host: &'b impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<&'b State> {
    Ok(host.state())
}

#[concordium_cfg_test]
mod tests {}
