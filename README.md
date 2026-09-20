# VTT Player Stats

A Rust telemetry and analytics project intended to turn recorded virtual tabletop gameplay events into player statistics, campaign records, and historical insights.

## Purpose

PlayerStats is designed to answer **"What can we learn from everything that happened?"** By processing events such as damage, healing, kills, critical hits, and character deaths, it aims to help players explore their characters' contributions and trends across encounters and campaigns.

The planned analytics include:

- Player totals for damage, healing, kills, and other combat events.
- Per-encounter averages and personal records, such as highest damage in a turn.
- Campaign rankings and records across players.
- Historical trends and damage consistency using measures such as variance and standard deviation.
- Filtering and aggregation by player, encounter, or campaign.

## Role in the VTT

The intended architecture separates gameplay, simulation, and historical analysis:

| Component | Responsibility |
| --- | --- |
| **Unity / C# VTT** | Run the game, capture gameplay events, and display results. |
| **C++ EncounterLab** | Simulate possible encounters to estimate outcomes and probabilities: "What might happen?" |
| **Rust PlayerStats** | Analyze recorded gameplay to produce statistics and insights: "What has happened?" |

Unity captures the events as they happen. PlayerStats processes those records into summaries, rankings, and analysis for the Unity UI.

## Learning Goals

The project provides a practical setting for learning Rust through structured event processing: enums and pattern matching, collections, iterators, ownership and borrowing, and explicit handling of missing data and errors with `Option` and `Result`.

Development is intended to begin as a standalone program or library, with Unity integration as a later milestone once the analytics work independently.
