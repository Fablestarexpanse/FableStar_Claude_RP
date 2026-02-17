# WorldWeaver GM Dashboard - Complete Guide

## Overview

The GM Dashboard is a comprehensive world-building, monitoring, and testing interface for WorldWeaver. It provides visual tools for creating and managing your persistent RPG world.

**Access:** Navigate to `/gm` route in your browser

---

## Dashboard Components

### 1. 🗺️ World Map Editor

**Purpose:** Visual room creation and connection management

**Features:**
- **Interactive Canvas:** Drag-and-drop room nodes
- **Visual Connections:** See room exits as arrows
- **Room Inspector:** Edit room properties in sidebar
- **Create Rooms:** Add new locations to your world
- **Edit Exits:** Connect rooms with directional exits

**Workflow:**
1. Click "Create Room" to add a new location
2. Enter room name and description
3. Click on room nodes to select and edit
4. Add exits to connect rooms
5. Save changes to persist to database

**Canvas Controls:**
- Click room node to select
- Selected room shows in inspector
- Orange highlight indicates selection
- Grid background for alignment

**Room Properties:**
- Name (displayed on map)
- Description (full text)
- Exits (directional connections)
- Position (x, y coordinates)

---

### 2. 👥 NPC Manager

**Purpose:** Create, edit, and manage NPCs with schedules

**Features:**
- **NPC List:** Browse all NPCs with search
- **Full Editor:** Edit all NPC properties
- **Stats System:** Set RPG stats (Str, Dex, Int, Cha, Con)
- **Schedule Builder:** Create time-based NPC routines
- **Relationship Tracking:** (Coming soon)

**NPC Properties:**

**Basic Info:**
- Name
- Description (physical appearance)
- Personality (behavioral traits)
- Greeting (what they say when approached)
- Current Location

**Stats:**
- Strength (1-20)
- Dexterity (1-20)
- Intelligence (1-20)
- Charisma (1-20)
- Constitution (1-20)

**Schedule System:**
Each schedule entry has:
- **Priority:** 1-10 (higher = more important)
- **Time Range:** Start hour to end hour (0-23)
- **Activity:** What the NPC is doing
- **Location:** Where they should be

**Schedule Example:**
```
Priority 10: 8:00-18:00 → Forging at Blacksmith's Forge
Priority 5:  18:00-8:00 → Sleeping at Forge Quarters
```

**How Schedules Work:**
- System checks all schedule entries
- Picks highest priority that matches current time
- NPC moves to specified location
- NPC performs specified activity

**Creating an NPC:**
1. Click "Create NPC"
2. Fill in basic info
3. Set starting location
4. Click "Create"
5. Select NPC to edit stats and schedule

---

### 3. 📊 Simulation Monitor

**Purpose:** Real-time monitoring of world simulation state

**Metrics Displayed:**

**Current Tick:**
- Total ticks elapsed
- Ticks per second rate
- Highlighted in orange

**NPCs:**
- Active NPCs (currently simulated)
- Total NPCs in world

**Rooms:**
- Total rooms loaded

**Events:**
- Total events recorded in log

**Memory:**
- Current memory usage

**Uptime:**
- How long simulation has been running

**Controls:**

**Simulation State:**
- ⏸️ Pause - Stop tick progression
- ▶️ Resume - Continue simulation

**Tick Rate:**
- Adjust simulation speed
- Range: 0.1 to 10.0 ticks/second
- Default: 1.0 tick/second
- Use + / - buttons to adjust

**Actions:**
- 🔄 Reset Simulation - Clear all progress (with confirmation)

**System Health:**
- ● Simulation Engine - ECS tick status
- ● Database - SQLite connection status
- ● Event Log - Event recording status

**Colors:**
- 🟢 Green = Healthy
- 🟡 Yellow = Warning
- 🔴 Red = Error

---

### 4. 📜 Event Log Viewer

**Purpose:** Browse and filter complete event history

**Features:**
- **Search:** Text search across all events
- **Tag Filtering:** Filter by event tags
- **Event Details:** View full event information
- **Real-time Updates:** Auto-refresh available

**Event Types:**
- `player_moved` - Player navigation
- `npc_moved` - NPC movement
- `player_talked_to_npc` - Dialogue interactions
- `item_picked_up` - Item acquisition
- `combat_started` - Combat initiation
- `time_advanced` - World clock progression
- `item_sold` - Economic transactions

**Event Tags:**
- `player` - Player-related events
- `npc` - NPC-related events
- `movement` - Location changes
- `dialogue` - Conversations
- `combat` - Combat events
- `world` - World state changes
- `economy` - Economic events
- `npc:name` - Specific NPC events

**Filtering:**
1. Enter search query in search box
2. Click tags to filter by category
3. Multiple tags = AND logic
4. "Clear Filters" to reset

**Event Details Panel:**
- Event ID (UUID)
- Tick number
- Timestamp
- Event type (color-coded)
- Full description
- All tags
- Copy JSON button

**Use Cases:**
- Debug player actions
- Track NPC behavior
- Monitor world progression
- Analyze event patterns
- Generate narrative summaries

---

### 5. ⚙️ Testing Console

**Purpose:** Execute GM commands for world manipulation

**Features:**
- **Command History:** Arrow up/down to recall
- **Auto-complete:** Tab completion (coming soon)
- **Quick Actions:** Buttons for common commands
- **Command Reference:** Built-in help

**Available Commands:**

**help**
- Shows all available commands

**spawn_npc <name>**
- Spawns an NPC in current room
- Example: `spawn_npc Elara the Merchant`

**create_room <name>**
- Creates a new room
- Example: `create_room Ancient Library`

**teleport <room_name>**
- Teleports player to specified room
- Example: `teleport Town Square`

**give_item <item_name>**
- Gives item to player
- Example: `give_item Enchanted Sword`

**set_stat <stat> <value>**
- Sets player stat
- Stats: strength, dexterity, intelligence, charisma, constitution
- Example: `set_stat strength 15`

**advance_time <hours>**
- Advances world clock
- Example: `advance_time 5`

**trigger_event <event_type>**
- Manually triggers a world event
- Example: `trigger_event weather_change`

**clear**
- Clears console output

**save**
- Forces immediate world save to database

**load**
- Reloads world from database

**Console Features:**
- Timestamps on all output
- Color-coded messages:
  - 🟢 Green = Commands
  - ⚪ White = Output
  - 🔴 Red = Errors
- Command history (arrow keys)
- Auto-scroll to latest

**Quick Actions Sidebar:**
- 👤 Spawn NPC - Pre-fills spawn_npc command
- 🏠 Create Room - Pre-fills create_room command
- 🌀 Teleport - Pre-fills teleport command
- 🎁 Give Item - Pre-fills give_item command
- ⏰ Advance Time - Pre-fills advance_time command
- 💾 Save World - Executes save immediately

---

## Workflow Examples

### Creating a New Area

1. **World Map Editor:**
   - Click "Create Room" for each location
   - Name rooms: "Dark Forest", "Forest Clearing", "Ancient Ruins"
   - Add descriptions

2. **Connect Rooms:**
   - Select "Dark Forest"
   - Add exit: north → "Forest Clearing"
   - Select "Forest Clearing"
   - Add exit: east → "Ancient Ruins"

3. **Populate with NPCs:**
   - Go to NPC Manager
   - Create "Mysterious Hermit"
   - Set location: "Forest Clearing"
   - Add schedule: Always stay in clearing

4. **Test:**
   - Go to Testing Console
   - `teleport Dark Forest`
   - Verify connections work
   - `spawn_npc Forest Guardian` for additional NPC

### Monitoring Player Session

1. **Simulation Monitor:**
   - Watch tick counter advance
   - Monitor active NPCs
   - Check system health

2. **Event Log Viewer:**
   - Filter by "player" tag
   - See all player actions
   - Check for errors

3. **Testing Console:**
   - Use `save` to checkpoint progress
   - Use `advance_time` to test time-based events

### Debugging NPC Behavior

1. **Event Log Viewer:**
   - Filter by `npc:name` tag
   - See all NPC movements
   - Check schedule execution

2. **NPC Manager:**
   - Select problematic NPC
   - Review schedule priorities
   - Adjust time ranges

3. **Testing Console:**
   - `advance_time 1` to step through hours
   - Watch NPC behavior in Event Log

---

## Keyboard Shortcuts

**Testing Console:**
- `Enter` - Execute command
- `↑` - Previous command in history
- `↓` - Next command in history
- `Ctrl+L` - Clear console (coming soon)

**General:**
- `Ctrl+S` - Save world (coming soon)
- `Ctrl+R` - Refresh current view (coming soon)

---

## Best Practices

### World Building

1. **Start Small:**
   - Create 3-5 connected rooms
   - Add 2-3 NPCs
   - Test thoroughly before expanding

2. **Logical Connections:**
   - Use compass directions (north, south, east, west)
   - Make exits bidirectional when appropriate
   - Avoid dead ends unless intentional

3. **NPC Schedules:**
   - Always have a fallback schedule (priority 1, Always condition)
   - Higher priority for special events
   - Test time progression to verify

### Testing

1. **Incremental Testing:**
   - Test after each room creation
   - Verify exits work both ways
   - Check NPC spawns in correct location

2. **Event Monitoring:**
   - Keep Event Log open during testing
   - Filter by relevant tags
   - Look for unexpected events

3. **Save Frequently:**
   - Use `save` command after major changes
   - Test `load` to verify persistence
   - Keep backups of database file

### Performance

1. **Monitor Metrics:**
   - Watch tick rate stay consistent
   - Check memory usage doesn't grow unbounded
   - Verify event log doesn't get too large

2. **Optimize NPCs:**
   - Use LOD system for distant NPCs
   - Limit active NPCs in player's area
   - Simplify schedules when possible

---

## Troubleshooting

### "Room not found" errors
- Check room ID is correct
- Verify room was saved to database
- Refresh World Map Editor

### NPCs not following schedule
- Check schedule priorities
- Verify time ranges don't overlap incorrectly
- Ensure fallback schedule exists

### Events not appearing in log
- Refresh Event Log Viewer
- Check event recording is enabled
- Verify simulation is running (not paused)

### Console commands not working
- Check command syntax with `help`
- Verify backend commands are implemented
- Check browser console for errors

---

## Technical Details

### Data Flow

```
GM Dashboard (Frontend)
    ↓
Tauri IPC Commands
    ↓
Rust Backend (GameWorld)
    ↓
Bevy ECS + SQLite
```

### File Structure

```
src/routes/gm/+page.svelte          - Main dashboard layout
src/lib/components/gm/
  ├── WorldMapEditor.svelte          - Visual map editor
  ├── NpcManager.svelte              - NPC CRUD interface
  ├── SimulationMonitor.svelte       - Real-time metrics
  ├── EventLogViewer.svelte          - Event history browser
  └── TestingConsole.svelte          - Command interface
```

### Backend Commands (To Implement)

```rust
// Room management
create_room(name, description) -> RoomId
update_room(id, data) -> Result
delete_room(id) -> Result
get_all_rooms() -> Vec<RoomDetails>

// NPC management
create_npc(data) -> NpcId
update_npc(id, data) -> Result
delete_npc(id) -> Result
get_all_npcs() -> Vec<NpcInfo>

// World manipulation
teleport_player(room_id) -> Result
spawn_npc(npc_id, room_id) -> Result
give_item(item_id) -> Result
set_player_stat(stat, value) -> Result

// Simulation control
pause_simulation() -> Result
resume_simulation() -> Result
set_tick_rate(rate) -> Result
advance_time(hours) -> Result

// Event queries
query_events(filter) -> Vec<EventRecord>
get_event_stats() -> EventStats

// Persistence
force_save() -> Result
force_load() -> Result
get_database_stats() -> DbStats
```

---

## Future Enhancements

### Planned Features

1. **Visual Scripting:**
   - Node-based quest editor
   - Dialogue tree builder
   - Trigger system

2. **Analytics:**
   - Player behavior heatmaps
   - NPC interaction graphs
   - Economy flow visualization

3. **Collaboration:**
   - Multi-GM support
   - Change tracking
   - Approval workflows

4. **Import/Export:**
   - JSON world export
   - Template library
   - Community sharing

5. **Advanced Testing:**
   - Automated test scenarios
   - Performance profiling
   - Regression testing

---

## Conclusion

The GM Dashboard provides everything you need to build, monitor, and test your persistent RPG world. Start with the World Map Editor to create your geography, use the NPC Manager to populate it with characters, monitor everything with the Simulation Monitor, debug with the Event Log Viewer, and manipulate the world with the Testing Console.

**Next Steps:**
1. Create your first 3 rooms
2. Add 2 NPCs with schedules
3. Test navigation and interactions
4. Monitor events and simulation state
5. Expand your world!

**Happy World Building!** 🎭✨
