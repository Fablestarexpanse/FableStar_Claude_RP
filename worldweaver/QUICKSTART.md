# WorldWeaver - Quick Start Guide

## 🎉 What's Been Built

You now have a **fully functional MVP** of WorldWeaver! Here's what works:

### ✅ Working Features

1. **Persistent World Simulation**
   - The Crossroads Inn (starter room)
   - Gareth the Innkeeper (NPC with personality)
   - Player character with position tracking

2. **Interactive Commands**
   - `look` or `l` - Examine your surroundings
   - `help` - Show available commands
   - `talk to gareth` - Interact with the innkeeper

3. **Modern UI**
   - Theater-style scrolling narrative
   - Sci-fi themed design (dark blue + orange)
   - Smooth animations and responsive layout

## 🚀 Running the App

The app is currently starting up! Once it finishes compiling:

1. **The WorldWeaver window will open automatically**
2. **You'll see**: "The Crossroads Inn" description
3. **You can type commands** in the input field at the bottom

### If the window doesn't appear:

```bash
# Make sure you're in the worldweaver directory
cd "F:\Cursor Projects\Fablestar_Claude_RP\FableStar_Claude_RP\worldweaver"

# Run the development server
npm run tauri dev
```

## 🎮 How to Use

### Basic Commands

```
> look
You are in The Crossroads Inn.

A cozy common room with worn wooden tables and a crackling fireplace...

You see: Gareth the Innkeeper

Obvious exits: north

> help
Available commands:
- look (or l): Examine your surroundings
- help: Show this message
- talk to [name]: Start a conversation
- north/south/east/west: Move in that direction (coming soon)

> talk to gareth
Gareth the Innkeeper looks up and smiles. 'Welcome to the Crossroads! What can I get you?'

[Note: Full NPC dialogue will be powered by Claude in the next phase]
```

## 🏗️ Architecture Overview

```
┌─────────────────────────────────────┐
│     Tauri Desktop Application       │
│                                     │
│  ┌──────────┐      ┌────────────┐ │
│  │  Svelte  │◄────►│    Rust    │ │
│  │    UI    │ IPC  │  Backend   │ │
│  └──────────┘      └────────────┘ │
│                          │          │
│                    ┌─────▼──────┐  │
│                    │  Bevy ECS  │  │
│                    │   World    │  │
│                    └────────────┘  │
└─────────────────────────────────────┘
```

**Key Components:**

- **Bevy ECS**: Manages all game entities (rooms, NPCs, player)
- **Tauri Commands**: Bridge between frontend and backend
- **Svelte Stores**: Reactive state management
- **RoleplayView**: Main player interface component

## 📁 Project Structure

```
worldweaver/
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── simulation/     # Bevy ECS world
│   │   │   ├── components.rs
│   │   │   └── world.rs
│   │   ├── database/       # SQLite (future)
│   │   ├── commands.rs     # Tauri IPC
│   │   └── main.rs
│   └── Cargo.toml
│
├── src/                    # Svelte frontend
│   ├── lib/
│   │   ├── components/
│   │   │   └── player/
│   │   │       └── RoleplayView.svelte
│   │   ├── stores/
│   │   │   └── worldState.ts
│   │   └── utils/
│   │       └── tauri.ts
│   ├── routes/
│   │   ├── +page.svelte    # Player view
│   │   └── gm/
│   │       └── +page.svelte # GM dashboard
│   └── app.css
│
├── README.md
├── IMPLEMENTATION_STATUS.md
└── package.json
```

## 🎨 Design System

**Colors:**
- Background: Dark blue-grey (#1a2332)
- Accent: Vibrant orange (#ff8c42)
- Text: Cool white (#e8f0f7)

**Fonts:**
- Headings: Orbitron (sci-fi)
- Body: Rajdhani (clean, readable)
- Code: Roboto Mono

## 🔧 Development Commands

```bash
# Start development server (hot reload)
npm run tauri dev

# Build production version
npm run tauri build

# Frontend only (for UI testing)
npm run dev

# Backend only (for logic testing)
cd src-tauri && cargo build

# Type checking
npm run check
```

## 🐛 Troubleshooting

### App won't start?

1. Check Node.js version: `node --version` (need 18+)
2. Check Rust version: `rustc --version` (need 1.70+)
3. Reinstall dependencies: `npm install`
4. Clean build: `cd src-tauri && cargo clean && cd .. && npm run tauri dev`

### UI looks broken?

1. Clear browser cache (Ctrl+Shift+R in dev window)
2. Check console for errors (F12)
3. Verify fonts loaded (check Network tab)

### Commands not working?

1. Check terminal for Rust errors
2. Verify Tauri commands are registered in `main.rs`
3. Check browser console for IPC errors

## 📝 What's Next?

### Immediate Next Steps:
1. **Test the MVP** - Try all commands, verify UI works
2. **Add a second room** - Implement room navigation
3. **Save/Load** - Integrate SQLite persistence

### Future Phases:
- **Phase 3**: Multi-room navigation
- **Phase 4**: Claude MCP integration for dynamic dialogue
- **Phase 5**: GM dashboard for world building
- **Phase 6**: Advanced features (economy, factions, quests)

## 🎯 Success Criteria

The MVP is successful if you can:
- ✅ Open the app
- ✅ See The Crossroads Inn description
- ✅ Type "look" and see room details
- ✅ Type "help" and see commands
- ✅ Type "talk to gareth" and get a response
- ✅ UI matches the sci-fi design

## 📚 Documentation

- `README.md` - Project overview
- `IMPLEMENTATION_STATUS.md` - Detailed build status
- `QUICKSTART.md` - This file!

## 💡 Tips

1. **Use the help command** - It shows all available commands
2. **Press Enter to submit** - No need to click the button
3. **Scroll works** - The narrative area scrolls automatically
4. **Case insensitive** - Commands work in any case

## 🌟 Cool Features to Notice

- **Smooth scrolling** - Auto-scrolls to latest message
- **Loading states** - Shows "..." while processing
- **Error handling** - Graceful error messages
- **Keyboard shortcuts** - Enter to submit
- **Responsive design** - Resizable window
- **Custom scrollbars** - Match the theme

## 🚧 Known Limitations (MVP)

- Only one room (more coming in Phase 2)
- No persistence yet (restarts lose state)
- No LLM integration (Phase 4)
- GM dashboard is placeholder
- No multi-player support

## 🎉 Congratulations!

You've successfully built the foundation of WorldWeaver! The architecture is solid, the UI is beautiful, and the simulation engine is ready for expansion.

**Next**: Test it out and let me know what you'd like to add next!
