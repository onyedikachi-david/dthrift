# Tanda Smart Contract

The Tanda smart contract is a blockchain-based implementation of a traditional financial model commonly known as Tanda, Osusu, or rotating savings and credit association (ROSCA).

## Contract instance and address

- Contract address {"index":3657,"subindex":0} 
- 'dthrift_instance'.

## Overview

A Tanda is a group of individuals who agree to contribute a fixed amount of money at regular intervals. Each member of the group takes turns receiving the pooled sum until every member has received it. The Tanda is complete when all members have received their share.

The Tanda smart contract automates the process of managing and distributing funds within a Tanda group. Members can join the Tanda, make their contributions, and receive their payouts automatically through the smart contract.

## How it works

The Tanda smart contract allows users to create a Tanda group by specifying the number of members, the amount to be contributed, and the payout interval. Once the group is created, users can join the Tanda group by paying their contribution amount. When all members have joined, the first member in the queue receives the payout, and the cycle repeats until all members have received their payout.

The Tanda smart contract uses a queue to keep track of the order in which members receive their payout. Once a member receives their payout, they are moved to the back of the queue. If a member misses a payment, they are removed from the queue and cannot receive their payout until they catch up on their payments.The Tanda smart contract allows users to create a Tanda group by specifying the number of members, the amount to be contributed, and the payout interval. Once the group is created, users can join the Tanda group by paying their contribution amount. When all members have joined, the first member in the queue receives the payout, and the cycle repeats until all members have received their payout.

The Tanda smart contract uses a queue to keep track of the order in which members receive their payout. Once a member receives their payout, they are moved to the back of the queue. If a member misses a payment, they are removed from the queue and cannot receive their payout until they catch up on their payments.
## Features

The Tanda smart contract includes the following features:

- Members can join the Tanda and make their contributions.
- The smart contract keeps track of the contributions made by each member.
- The smart contract calculates the payout amount for each member based on the Tanda's rules.
- Members can withdraw their payouts from the smart contract.
- The Tanda can be configured with custom rules, such as the contribution amount, payout frequency, and payout order.

## Usage

To use the Tanda smart contract, deploy it to a blockchain network that supports smart contracts. Then, members of the Tanda group can interact with the smart contract through a blockchain client or web interface.

To join the Tanda, a member sends a transaction to the smart contract with their contribution amount. The smart contract adds the contribution to the member's account and tracks their participation in the Tanda.

When it's time for a member to receive their payout, the smart contract calculates the payout amount and sends it to the member's account. The member can then withdraw their payout from the smart contract.

The Tanda smart contract can be customized to fit the needs of a specific Tanda group. The administrator of the smart contract can configure the contribution amount, payout frequency, and payout order according to the group's rules.

## Development

The Tanda smart contract is implemented in Rust using the concordium libraries. To develop the smart contract, you'll need to have Rust and the concordium cargo library installed on your machine.

To build the smart contract, run the following command:

To deploy the smart contract to the blockchain node for testing, run the following command:

## Potential Future Enhancements
Here are some potential future enhancements to the Tanda smart contract:

Support for different payout schedules (e.g. bi-weekly instead of monthly).
Ability to specify a different order for members to receive payouts (e.g. reverse order instead of sequential order).
Support for partial contributions (e.g. allow members to contribute less than the full cycle amount).

## License

This smart contract is licensed under the MIT License. See the `LICENSE` file for more information.
