[Prev](./page10.md) | [Next](./page12.md)

# Backend Refactoring: Separation of Concerns

This document details the architectural changes made to the backend to improve maintainability and scalability.

## Overview
The monolithic `src/main.rs` file has been decomposed into a modular structure, separating API handlers, business logic, and data models.

## New Structure

```text
src/
├── main.rs           # Entry point: App setup, dependency injection, and routing configuration
├── api/
│   ├── mod.rs        # Module exports
│   └── handlers.rs   # Request handlers (upload, list, delete, static assets)
├── service/
│   ├── mod.rs        # Module exports
│   └── transcription.rs # Business logic for Gemini API interaction
└── models/
    ├── mod.rs        # Module exports
    └── dtos.rs       # Data Transfer Objects (Request/Response structs)
```

## Benefits
- **Maintainability**: Smaller, focused files are easier to read and modify.
- **Testability**: Logic is isolated, making unit testing easier (e.g., testing `transcription.rs` independently of HTTP handlers).
- **Scalability**: New features can be added by creating new modules without cluttering `main.rs`.

[Prev](./page10.md) | [Next](./page12.md)
