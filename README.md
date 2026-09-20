# Player Stats

A Rust project for tracking and analyzing statistics from games, applications, and other event-driven systems.

## Purpose

Player Stats aims to turn structured events into totals, records, and historical insights. It is designed to be independent of a specific game or platform, so it can support any system that needs to track statistics.

## Current Status

The project is an early prototype. The current executable demonstrates incrementing a kill counter; event ingestion, storage, and analytics are planned.

## Planned Analytics

- Totals and counts for configurable statistics and event types.
- Averages, personal records, and rankings.
- Historical trends and consistency measures, including variance and standard deviation.
- Filtering and aggregation by tracked entity, session, or time period.

Possible uses include player performance, application activity, and other event-based statistics.

## Run Locally

Install Rust and Cargo, then run:

```sh
git clone git@github.com:csbabcock/player-stats.git
cd player-stats
cargo run
```
