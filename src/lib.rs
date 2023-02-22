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

use concordium_std::*;
use core::fmt::Debug;
use std::collections::BTreeSet;

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
    contribution_amount: u64,
    /// The penalty amount to paid in addition to the contribution amount.
    penalty_amount: u64,
    /// The total amount of contributions made by all members
    total_contributions: u64,
    /// The payout cycle for the Tanda
    payout_cycle: u64,
    /// The current payout cycle
    current_cycle: u64,
    /// The time when the Tanda started or will start
    start_time: Timestamp,
    /// The time when the Tanda will be finalized
    end_time: Timestamp,
    /// The member who is next in line to receive a payout
    next_receiver: Option<AccountAddress>,
    /// The list of accounts that have received payment after every cycle
    completed_cycles: Vec<(u64, Vec<AccountAddress>)>,
    /// The list of accounts that have made a contribution to the tanda
    contributors: BTreeSet<AccountAddress>,
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
    /// Raised when a smart contract tries to join a club.
    /// Only account are allowed to join a club.
    ContractMember,
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
#[derive(Serialize, SchemaType, Clone)]
struct InitParameter {
    /// The name of the Tanda or Osusu club
    name: String,
    /// A brief description of the Tanda club
    description: String,
    /// The creator of the Tanda club address
    creator: AccountAddress,
    /// The amount of money each member contributes to the Tanda
    contribution_amount: u64,
    /// The payout cycle for the Tanda
    payout_cycle: u64,
    /// The time when the Tanda will start using the RFC 3339 format (https://tools.ietf.org/html/rfc3339)
    start_time: Timestamp,
    /// The time when the Tanda will be finalized using the RFC 3339 format (https://tools.ietf.org/html/rfc3339)
    end_time: Timestamp,
    /// The penalty amount for missed payments
    penalty_amount: u64,
    /// The maximum number of members allowed.
    max_contributors: u64,
}

#[derive(Serialize, SchemaType, Clone)]
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
    // Your code
    let param: InitParameter = ctx.parameter_cursor().get()?;
    // let acc = match ctx.sender() {
    //     Address::Account(acc) => acc,
    //     Address::Contract(_) => return Err(ContractError::ContractVoter),
    // };
    let account = ctx.init_origin();

    Ok(State {
        name: param.name,
        description: param.description,
        creator: account,
        tanda_state: TandaState::Open,
        members: None,
        contribution_amount: param.contribution_amount,
        penalty_amount: param.penalty_amount,
        total_contributions: 0,
        payout_cycle: param.payout_cycle,
        current_cycle: 0,
        start_time: param.start_time,
        end_time: param.end_time,
        next_receiver: None,
        completed_cycles: vec![],
        contributors: BTreeSet::new(),
        max_contributors: param.max_contributors,
        user_index: 0,
    })
}

/// Enables a qualified user to join a Tanda club and pay penalty fee.
///
/// It fails if:
/// - It fails to parse the parameter.
/// - A contract tries to vote.
/// - The Tanda club has reached its maximum limit.
/// - The Tanda state is closed.
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
    // penalty_amount: Amount,
) -> Result<(), Error> {
    // let parameter = ctx.parameter_cursor().get()?;
    // Check that the Tanda is still open
    ensure!(
        host.state().tanda_state != TandaState::Closed,
        Error::TandaClosed
    );

    // Check if the Tanda has reached its maximum limit.
    let members = &mut host.state().members.as_ref().map_or(0, |v| v.len());
    ensure!(
        *members as u64 == host.state().max_contributors,
        Error::MaximumReached
    );

    // Ensure that the sender is an account
    let acc = match ctx.sender() {
        Address::Account(acc) => acc,
        Address::Contract(_) => return Err(Error::ContractMember),
    };

    // Update penalty_amount
    let param: JoinTandaParameter = ctx.parameter_cursor().get()?;
    let penalty_amount = param.penalty_amount;
    host.state_mut().penalty_amount += penalty_amount;

    // Update the user_index count
    // let new_user_index = host.state_mut().user_index += 1;

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
