Advanced Staking Protocol (Rust/Anchor)
A high-performance, scalable staking engine designed for SPL/ERC-20 tokens. 
This contract implements a "Reward Per Share" accounting model, allowing for O(1) complexity when calculating rewards, regardless of the number of participants.

## Advanced Core Features
Fixed & Dynamic APR: Support for both constant reward rates and fluctuating rates based on total pool TVL (Total Value Locked).

Flexible Locking Tiers: Users can choose between "Liquid Staking" (no lock) or "Committed Staking" (fixed terms) for higher yield multipliers.

Reward Compounding: One-click functionality to re-stake earned rewards back into the principal amount.

Emergency Withdrawals: A "Circuit Breaker" mechanism that allows users to exit their principal in case of contract pauses, typically incurring a penalty.

Slashing Logic: Programmable hooks for governance-based slashing or early-withdrawal penalties.

## Program Architecture (Solana/Anchor Example)
Account Structures:

GlobalState: Stores the reward_duration, total_staked, and the last_update_time.

UserStakerEntry: A PDA (Program Derived Address) for every user storing their balance, reward_tally, and lock_expiry.

Vault (Token Account): The secure, program-owned account holding the staked assets.

