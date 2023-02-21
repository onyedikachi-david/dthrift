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

#[derive(Serialize, SchemaType, Clone, Copy, Debug)]
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
//     name: String,
//     description: String,
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
#[derive(Debug, Serialize, SchemaType, Clone)]
pub struct State {
    /// State of the Tanda
    tanda_state: TandaState,
    /// The list of members who have joined the Tanda
    members: Vec<(AccountAddress, u64)>,
    /// The amount of money each member contributes to the Tanda
    contribution_amount: u64,
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
}
/// Your smart contract errors.
#[derive(Debug, PartialEq, Eq, Reject, Serial, SchemaType)]
enum Error {
    /// Failed parsing the parameter.
    #[from(ParseError)]
    ParseParamsError,
    /// Your error
    YourError,
}

#[derive(Serialize, SchemaType, Clone)]
// struct InitParameter {
//     name: String,
//     description: String,
//     creator: AccountAddress,
//     amount: u128,
//     collateral: u128,
//     max_members: u32,
//     time_interval: Timestamp,
// }

struct InitParameter {
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
}

/// Init function that creates a new smart contract.
#[init(contract = "dthrift", parameter = "InitParameter")]
fn init<S: HasStateApi>(
    ctx: &impl HasInitContext,
    _state_builder: &mut StateBuilder<S>,
) -> InitResult<State> {
    // Your code
    let param: InitParameter = ctx.parameter_cursor().get()?;
    let acc = match ctx.sender() {
        Address::Account(acc) => acc,
        Address::Contract(_) => return Err(ContractError::ContractVoter),
    };

    Ok(State {
        name: param.name,
        description: param.description,
        creator: acc,
    })
}

/// Receive function. The input parameter is the boolean variable `throw_error`.
///  If `throw_error == true`, the receive function will throw a custom error.
///  If `throw_error == false`, the receive function executes successfully.
#[receive(
    contract = "dthrift",
    name = "receive",
    parameter = "bool",
    error = "Error",
    mutable
)]
fn receive<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    _host: &mut impl HasHost<State, StateApiType = S>,
) -> Result<(), Error> {
    // Your code

    let throw_error = ctx.parameter_cursor().get()?; // Returns Error::ParseError on failure
    if throw_error {
        Err(Error::YourError)
    } else {
        Ok(())
    }
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
