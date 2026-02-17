# FablestarUI - AI-Powered World Building & Role-Playing Tool

A next-generation world-building and role-playing tool that combines traditional MUD (Multi-User Dungeon) gameplay with AI-powered game mastering through Claude AI.

## Features

### Dual Mode Operation

**Builder Mode** - Create and manage your worlds
- Character creation and management
- Location/place tracking with hierarchical organization
- Faction/organization systems
- Item/artifact cataloging
- Timeline and event tracking
- Relationship mapping and visualization
- Interactive map integration (Azgaar Fantasy Map Generator)
- Fully configurable world rules and systems

**Play Mode** - Experience your world as a text-based adventure
- MUD-style text interface with retro aesthetics
- Claude AI acts as dynamic Game Master
- Real-time narration and NPC interactions
- Command-based gameplay (look, go, talk, take, use, etc.)
- Persistent game state and save system
- Dynamic world that evolves based on your actions

### World Persistence & Configurability

- **Full real-time persistence** - All changes auto-save immediately
- **Configurable game systems** - Define your own stat systems, combat mechanics, magic/tech levels
- **Dynamic world evolution** - NPCs remember interactions, factions gain/lose power
- **Time systems** - Optional world clock with calendars, seasons, and weather
- **Import/Export** - Backup and share your worlds

### Design

Modern post-apocalyptic sci-fi aesthetic with:
- Weathered slate grays and warm amber/orange accents
- Industrial UI components with subtle wear textures
- Glowing interactive elements
- Scanline effects on MUD interface
- Monospace terminal font for gameplay

## Tech Stack

- **Frontend**: React + Vite
- **Backend**: Node.js + Express
- **Real-time**: WebSocket
- **AI Integration**: Claude via MCP (Model Context Protocol)
- **Map Generator**: Azgaar Fantasy Map Generator (embedded)
- **Storage**: File-based JSON

## Getting Started

### Prerequisites

- Node.js 18+ installed
- Claude Desktop (for AI integration)

### Installation

1. **Install dependencies**

```bash
# Install server dependencies
cd server
npm install

# Install client dependencies
cd ../client
npm install
```

2. **Run as Desktop App (Recommended)**

```bash
# From the client directory
cd client
npm run electron:dev
```

This will start both the backend server and the Electron desktop application.

3. **Or run in Browser (Development)**

```bash
# Terminal 1 - Start backend server
cd server
npm start

# Terminal 2 - Start frontend dev server
cd client
npm run dev
```

Then navigate to `http://localhost:5173` in your browser.

### Building Desktop App

Create standalone installers for distribution:

```bash
cd client

# Build for Windows
npm run electron:build:win

# Build for macOS
npm run electron:build:mac

# Build for Linux
npm run electron:build:linux

# Build for all platforms
npm run electron:build
```

Installers will be created in `client/release/`

## Project Structure

```
FablestarUI/
├── server/
│   ├── index.js            # Express API + WebSocket
│   ├── mcp.js              # MCP server for Claude (coming soon)
│   ├── data/
│   │   ├── worlds/         # World JSON files
│   │   └── maps/           # Map files and images
│   └── package.json
├── client/
│   ├── src/
│   │   ├── components/     # React components
│   │   ├── styles/         # Design system
│   │   └── App.jsx
│   ├── package.json
│   └── vite.config.js
└── README.md
```

## Usage

### Creating a World

1. Click "+ New World" in the header
2. Configure your world settings (genre, rules, time system, etc.)
3. Start building characters, locations, factions, and items

### Playing Your World

1. Select a world from the dropdown
2. Switch to "Play" mode
3. Start a game session with your player character
4. Type commands to interact with the world
5. Claude AI will narrate and respond to your actions

### Available Commands (Play Mode)

- `look [target]` - Examine surroundings or specific object/NPC
- `go <direction>` - Move to adjacent location
- `north/south/east/west/up/down` - Quick movement
- `talk <npc>` - Initiate conversation
- `take <item>` - Pick up item
- `drop <item>` - Drop item from inventory
- `use <item>` - Use item
- `inventory` - Show inventory
- `stats` - Show character stats
- `attack <target>` - Initiate combat
- `save` - Save game state
- `help` - Show available commands

## Development Status

### Completed ✅
- Modern UI with post-apocalyptic aesthetic
- Design system (theme, components)
- Header with mode switcher
- Sidebar navigation
- Dashboard
- MUD-style game interface
- Express server with world management API
- WebSocket real-time updates
- File-based JSON storage
- World configuration system

### In Progress 🚧
- Character/Location/Faction/Item CRUD interfaces
- MCP server for Claude integration
- Fantasy Map Generator embedding
- Relationship graph visualization
- Timeline view

### Planned 📋
- AI Game Master integration
- NPC conversation system
- Combat mechanics
- Quest system
- World import/export

## License

MIT

## Credits

- Inspired by classic MUDs and modern AI assistants
- Map generation powered by [Azgaar's Fantasy Map Generator](https://github.com/Azgaar/Fantasy-Map-Generator)
- AI integration via Claude (Anthropic)
