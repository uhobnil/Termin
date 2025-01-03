# Termin

A cross-platform desktop reminder application built with Tauri. It runs in the system tray and helps you manage your schedules with customizable notifications.

## Features

- 🔔 Schedule Management
  - Create and manage schedules
  - Multiple reminder types:
    - One-time
    - Daily
    - Weekly
    - Monthly
    - Yearly
- 💻 System Integration
- 🎯 User Interface
- 🌈 Cross-platform
  - Windows
  - macOS

## Key Features Implementation

- Schedule Checking: Implemented in Rust backend with precise time matching
- Notification System: Uses Tauri's notification API
- Data Persistence: SQLite database with Sea-ORM
- State Management: In-memory schedule state with thread-safe access

## Tech Stack

- ### Frontend

  - Vue 3
  - Naive UI
  - Tauri API

- ### Backend
  - Rust
  - Sea-ORM (SQLite)
  - Tauri
  - Tokio

## License

MIT
