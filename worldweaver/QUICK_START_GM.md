# Quick Start - GM Dashboard

## Accessing the GM Dashboard

The app is now running! Here's how to access the new GM Dashboard:

### Method 1: Direct URL
In the Tauri window, navigate to:
```
http://localhost:1420/gm
```

### Method 2: From Player View
1. The app opens to the player view by default
2. Look for the "🎭 GM Dashboard" button (if we add a link)
3. Or manually navigate to `/gm` in the address bar

---

## What You'll See

### Main Dashboard Layout

**Header:**
- Title: "🎭 WorldWeaver GM Dashboard"
- Subtitle: "Architect & Monitor Your Persistent World"
- Button: "🎮 Player View" (to switch back)

**Tabs:**
- 🗺️ World Map - Visual room editor
- 👥 NPCs - NPC management
- 📊 Monitor - Real-time simulation stats
- 📜 Events - Event log viewer
- ⚙️ Console - Testing console

---

## New Styling Features

### Color Scheme (Matching LoRA Studio)
- **Background:** Dark blue-grey `hsl(220, 14%, 10%)`
- **Elevated surfaces:** `hsl(220, 14%, 14%)`
- **Accent:** Blue `rgb(59, 130, 246)`
- **Text:** Clean grey tones

### Visual Elements
- **Cleaner borders:** Subtle grey instead of transparent overlays
- **Blue accents:** Replaced orange with professional blue
- **Minimal shadows:** Only on modals
- **Smooth transitions:** 0.15s ease
- **Focus rings:** Clear keyboard navigation

---

## Testing the Components

### 1. World Map Editor
- Click "🗺️ World Map" tab
- See the interactive canvas with room nodes
- Click "Create Room" to test the modal
- Select a room to see the inspector

### 2. NPC Manager
- Click "👥 NPCs" tab
- Browse the NPC list (Gareth, Kael)
- Click "Create NPC" to see the form
- Select an NPC to edit stats and schedules

### 3. Simulation Monitor
- Click "📊 Monitor" tab
- Watch the tick counter increment
- See NPC counts, room counts, events
- Test Pause/Resume button
- Adjust tick rate with +/- buttons

### 4. Event Log Viewer
- Click "📜 Events" tab
- Browse event history
- Try the search box
- Click tag filters
- Select an event to see details

### 5. Testing Console
- Click "⚙️ Console" tab
- Type "help" and press Enter
- Try commands like "spawn_npc Test"
- Use arrow keys for command history
- Click quick action buttons

---

## Comparing to LoRA Studio

### Similarities You'll Notice

**Color System:**
- Same dark background tone
- Same elevated surface color
- Same border subtlety
- Same text grey tones

**Typography:**
- Same font stack (system fonts)
- Same font sizes (0.875rem base)
- Same font weights (500-600)

**Interactions:**
- Same button styles
- Same input focus states
- Same hover effects
- Same transition timing (0.15s)

**Components:**
- Same card styling
- Same panel headers
- Same badge design
- Same modal overlays

---

## Keyboard Navigation

Test the improved keyboard navigation:

**General:**
- `Tab` - Move between elements
- `Shift+Tab` - Move backwards
- `Enter` - Activate buttons
- `Escape` - Close modals

**Testing Console:**
- `↑` - Previous command
- `↓` - Next command
- `Enter` - Execute command

**Focus Rings:**
- Notice the blue ring around focused elements
- Clear visual indicator for keyboard users
- Accessible and professional

---

## What to Look For

### Design Improvements

✅ **Cleaner appearance** - Less visual noise
✅ **Better contrast** - Easier to read
✅ **Professional look** - Matches LoRA Studio
✅ **Consistent spacing** - More polished
✅ **Subtle borders** - Not distracting
✅ **Blue accents** - Modern and clean

### Functional Features

✅ **All 5 components** - Fully implemented
✅ **Tab navigation** - Smooth switching
✅ **Mock data** - Ready for testing
✅ **Responsive layout** - Adapts to window size
✅ **Smooth animations** - Professional feel

---

## Current Limitations

**Mock Data:**
- All data is frontend-only
- Changes don't persist
- Backend integration pending

**Features Pending:**
- Real-time updates
- Database persistence
- Backend commands
- WebSocket sync

---

## Next Steps

1. **Explore each tab** - See all 5 components
2. **Test interactions** - Click, type, navigate
3. **Check styling** - Compare to LoRA Studio
4. **Provide feedback** - What works, what doesn't
5. **Identify priorities** - Which features to implement first

---

## Troubleshooting

### App won't open
```bash
# Stop and restart
Ctrl+C (in terminal)
npm run tauri dev
```

### Styling looks wrong
- Hard refresh: `Ctrl+Shift+R`
- Clear cache and reload

### Can't navigate to /gm
- Manually type in address bar: `http://localhost:1420/gm`
- Or add a navigation link from player view

---

## Development Commands

```bash
# Running (already started)
npm run tauri dev

# Stop
Ctrl+C

# Build for production
npm run tauri build

# Check frontend
npm run check

# Check backend
cd src-tauri && cargo check
```

---

## Screenshots to Take

Capture these views for reference:
1. Main dashboard with tabs
2. World Map Editor with canvas
3. NPC Manager with list and editor
4. Simulation Monitor with metrics
5. Event Log Viewer with filters
6. Testing Console with commands

---

Enjoy exploring the new GM Dashboard! The styling now matches your LoRA Dataset Studio for a consistent, professional experience across your tools. 🎨✨
