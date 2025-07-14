# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Tauri + Vue 3 desktop application called "Tjimi". It uses:
- **Frontend**: Vue 3 with Vite for the web interface
- **Backend**: Rust with Tauri for the desktop application layer
- **Build System**: Vite for frontend bundling, Cargo for Rust compilation

## Architecture

The project follows a typical Tauri architecture:

- `src/` - Vue 3 frontend application
  - `App.vue` - Main Vue component with greet functionality
  - `main.js` - Vue app entry point
  - `assets/` - Static assets (images, etc.)

- `src-tauri/` - Rust backend application
  - `src/lib.rs` - Main Tauri application logic with command handlers
  - `src/main.rs` - Application entry point
  - `Cargo.toml` - Rust dependencies and configuration
  - `tauri.conf.json` - Tauri-specific configuration

- `public/` - Static assets served by Vite
- `vite.config.js` - Vite configuration optimized for Tauri development

## Common Commands

### Development
```bash
npm run dev          # Start development server (runs both frontend and Tauri)
npm run tauri dev    # Alternative way to start Tauri dev mode
```

### Building
```bash
npm run build        # Build the frontend for production
npm run tauri build  # Build the complete Tauri application
```

### Preview
```bash
npm run preview      # Preview the production build locally
```

### Rust Development
```bash
cd src-tauri
cargo build          # Build Rust backend
cargo run            # Run Rust backend directly
```

## Key Configuration

- **Vite Dev Server**: Runs on port 1420 (fixed port required by Tauri)
- **HMR**: Uses WebSocket on port 1421 for hot module replacement
- **Tauri Commands**: Defined in `src-tauri/src/lib.rs` and callable from Vue via `invoke()`

## Development Notes

- The `greet` command in `lib.rs:3` demonstrates the Tauri command pattern
- Vue components use `<script setup>` syntax
- Tauri API calls are made via `@tauri-apps/api/core` import
- The application uses Vue 3 Composition API with `ref()` for reactivity
- Dark mode styles are handled via CSS `@media (prefers-color-scheme: dark)`