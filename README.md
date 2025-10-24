# GM-wakey-wakey dApp

GM your Mornings. Build your wake-up schedule.

<img width="299" height="168" alt="image" src="https://github.com/user-attachments/assets/8137d250-1d97-41f8-9ac7-f30453857e38" />
(With frens)




-------------------------------------------------------------------------------------------------------------------



GM-wakey-wakey is a social habit-building dApp on Solana that helps you and your team build a consistent morning routine. It uses the power of social accountability and on-chain streaks to turn the struggle of waking up into a collaborative game.

## The Problem

It’s hard to get out of bed on schedule. Traditional alarm clocks lack real incentives, and personal discipline often fails over time.

## The Solution

GM-wakey-wakey app introduces social accountability. By joining a team, you commit to a daily check-in within your personal wake-up window. Your success contributes to the team's on-chain streak. Failing to check-in doesn't just break your own habit—it breaks the streak for the entire team.

## Use Case: Social Habit Building

### User Story

> As a team member, I want to check-in on time every morning, so that I contribute to our team's streak and don't let my friends down.

### Acceptance Criteria

1.  **Successful check-in continues the streak:**
    *   **Given:** My team has an active streak, and today is the day after the last successful check-in.
    *   **When:** I check-in within my personal time window.
    *   **Then:** The team's streak should increase by 1.

2.  **Check-in after a missed day resets the streak:**
    *   **Given:** My team missed yesterday's check-in.
    *   **When:** I check-in today within my time window.
    *   **Then:** The team's streak should be reset to 1.

3.  **Late check-in is not allowed:**
    *   **Given:** I am a member of a team.
    *   **When:** I attempt to check-in after my personal time window has closed.
    *   **Then:** The transaction should fail with a "CheckInWindowClosed" error.

## How It Works: On-Chain Logic

The program leverages a two-account model to manage state:

*   **Team Account (PDA):** Stores team-wide data, including the team name and the all-important **current_streak**.
*   **Player Account (PDA):** Each team member gets their own Player account, which stores their personal settings like wake_up_time, linking them to a specific team.

All time-based logic is handled securely on-chain using Solana's `Clock` sysvar, ensuring that the check-in window is enforced by the protocol, not the client.

PDA Data Diagram:
<img width="1418" height="704" alt="image" src="https://github.com/user-attachments/assets/699a3f45-4dd2-4bd7-b49e-c6ee725f1dad" />

Data-Flow Diagram:
<img width="1245" height="802" alt="image" src="https://github.com/user-attachments/assets/d6f4515c-67b8-41c0-9d69-f529586868f4" />



## Quick Start & Testing

### Prerequisites

*   Install the [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
*   Install [Anchor](https://www.anchor-lang.com/docs/installation)
*   Install Node.js and Yarn

### Local Setup

1.  **Clone the repository and enter the `app` directory.**
2.  **Build the program:**
    ```bash
    anchor build
    ```
3.  **Run tests:**
    ```bash
    anchor test
    ```
 
