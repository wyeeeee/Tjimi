# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Tauri + Vue 3 desktop application for managing Gemini API keys and proxying requests. It provides:
- **Frontend**: Vue 3 with Vue Router, Pinia, and responsive UI for API key management
- **Backend**: Rust with Tauri for desktop app, Axum for HTTP server, SQLite for data storage
- **Proxy Service**: HTTP server that forwards requests to Google's Gemini API with key rotation

## Architecture

### Frontend Structure (`src/`)
- `main.js` - Vue app entry point with Pinia and Router
- `App.vue` - Main app component with authentication routing
- `components/Layout.vue` - Main layout with navigation
- `views/` - Page components (Home, Login, ApiKeys, Logs)
- `stores/` - Pinia stores for state management (auth, apiKeys, logs)
- `router/` - Vue Router configuration

### Backend Structure (`src-tauri/src/`)
- `lib.rs` - Main Tauri application with command handlers
- `models/` - Data models (User, ApiKey, RequestLog)
- `services/` - Business logic (auth, api_key, gemini_proxy, key_rotation)
- `database/` - Database setup and migrations
- `server/` - HTTP server with Axum routes
- `commands/` - Tauri command handlers

## Common Commands

### Development
```bash
npm install          # Install frontend dependencies
npm run dev          # Start development server (frontend + Tauri + HTTP proxy)
```

### Building
```bash
npm run build        # Build the frontend for production
npm run tauri build  # Build the complete Tauri application
```

### Database
- SQLite database (`gemini_proxy.db`) is created automatically
- Tables: `users`, `api_keys`, `request_logs`

## Key Features

### Authentication System
- User registration and login with bcrypt password hashing
- Session management with local storage
- Protected routes with Vue Router guards

### API Key Management
- Create, read, update, delete API keys
- Enable/disable keys
- Usage tracking and statistics
- Key rotation for load balancing

### Proxy Service
- HTTP server runs on `http://127.0.0.1:5675`
- Forwards requests to `https://generativelanguage.googleapis.com`
- Supports both streaming and non-streaming responses
- Automatic key rotation and failure handling

### Supported Endpoints
- `GET /v1/models` - List available models
- `GET /v1/models/{model}` - Get model details
- `POST /v1/models/{model}/generateContent` - Generate content
- `POST /v1/models/{model}/streamGenerateContent` - Stream generate content
- `GET /health` - Health check

## Development Notes

- The HTTP proxy server starts automatically when the Tauri app launches
- API keys are rotated based on usage count and last used time
- Failed API keys (401/403 responses) are automatically disabled
- All requests are logged with response times and status codes
- Vue components use Composition API with `<script setup>`
- Responsive design with dark mode support
- SQLite database handles UUID primary keys as TEXT