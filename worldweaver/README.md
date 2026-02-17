# WorldWeaver

A persistent world RPG platform combining MUD-style simulation with LLM-powered narrative.

## Architecture

- **Backend**: Rust + Bevy ECS for deterministic world simulation
- **Frontend**: Svelte 5 + SvelteKit for modern UI
- **Desktop**: Tauri 2.x for cross-platform distribution
- **AI Integration**: Claude via MCP (Model Context Protocol)

## Core Concept

WorldWeaver separates **mechanical simulation** from **narrative generation**:

- **Bevy ECS** handles world state, time progression, NPC schedules, economy
- **Claude/LLM** generates natural language descriptions and dialogue
- The LLM is the **voice** of the world, not its brain

## Development

### Prerequisites

- Rust 1.70+
- Node.js 18+
- npm or pnpm

### Setup

```bash
# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

## Current Status: MVP Phase 1

✅ **Implemented:**
- ECS component system (Room, NPC, Player, Position)
- Starter world with The Crossroads Inn
- Basic player commands (look, help, talk to)
- Theater-style scrolling narrative UI
- Sci-fi themed design system

🚧 **Coming Next:**
- Multi-room navigation
- SQLite persistence
- Claude MCP integration for dynamic dialogue
- GM dashboard for world building

## Project Structure

```
worldweaver/
├── src-tauri/          # Rust backend
│   ├── src/
│   │   ├── simulation/ # Bevy ECS world simulation
│   │   ├── database/   # SQLite persistence
│   │   ├── commands.rs # Tauri IPC commands
│   │   └── main.rs     # Entry point
│   └── Cargo.toml
├── src/                # Svelte frontend
│   ├── lib/
│   │   ├── components/ # UI components
│   │   ├── stores/     # State management
│   │   └── utils/      # Tauri API wrapper
│   └── routes/         # SvelteKit routes
└── package.json
```

## License

MIT
