# Phase 2 Testing Guide

## Quick Start

```bash
cd "F:\Cursor Projects\Fablestar_Claude_RP\FableStar_Claude_RP\worldweaver"
npm run tauri dev
```

## Test Scenarios

### ✅ Test 1: Basic Navigation
```
> look
Expected: See "The Crossroads Inn" with Gareth

> north
Expected: Move to "Town Square"

> look
Expected: See Town Square with 3 exits (south, east, west)
```

### ✅ Test 2: Invalid Movement
```
> south
Expected: Return to Crossroads Inn

> west
Expected: Error message "You can't go west from here"
```

### ✅ Test 3: Full World Tour
```
> north          (Inn → Square)
> east           (Square → Merchant District)
> west           (Merchant → Square)
> west           (Square → Forge, meet Kael)
> east           (Forge → Square)
> south          (Square → Inn)
```

### ✅ Test 4: Shortcuts
```
> n             (same as north)
> e             (same as east)
> w             (same as west)
> s             (same as south)
```

### ✅ Test 5: MiniMap
- Check sidebar shows current room
- Verify exits list updates
- Watch "Rooms explored" counter (should reach 4)

### ✅ Test 6: NPCs
```
> look
Expected: In Inn, see "Gareth the Innkeeper"

> north
> west
> look
Expected: In Forge, see "Kael the Blacksmith"

> talk to kael
Expected: Kael responds with greeting
```

### ✅ Test 7: Rapid Movement
```
> n
> e
> w
> w
> e
> s
```
Expected: No crashes, smooth transitions

## Visual Checks

### MiniMap Should Show:
- Current room name
- Available exits (→ north, → south, etc.)
- Rooms explored count

### Narrative Area Should:
- Display room descriptions
- List NPCs present
- Show movement messages
- Auto-scroll to bottom

### Input Area Should:
- Accept commands
- Disable during loading
- Clear after submit

## World Map Reference

```
        Merchant District
               |
               |
    Forge ← Square → (future)
               |
               |
              Inn
```

## Expected Room Descriptions

**Inn:** Cozy tavern, fireplace, Gareth polishing mugs
**Square:** Cobblestone plaza, fountain, merchants
**Merchant:** Narrow street, shops, spices
**Forge:** Sweltering workshop, hammer on steel, Kael at bellows

## Common Issues

**App won't start:**
```bash
# Kill existing processes
Stop-Process -Name "worldweaver", "node" -Force -ErrorAction SilentlyContinue

# Rebuild
cd src-tauri
cargo build

# Try again
cd ..
npm run tauri dev
```

**Commands not working:**
- Check console for errors (F12)
- Verify backend is running (check terminal output)
- Try "help" command to see available actions

**MiniMap not updating:**
- Check browser console for errors
- Verify visitedRooms store is updating
- Try refreshing the page

## Success Criteria

✅ All 4 rooms accessible
✅ Movement validated (can't go through walls)
✅ NPCs appear in correct rooms
✅ MiniMap tracks exploration
✅ Shortcuts work (n/s/e/w)
✅ No crashes or errors
✅ Smooth user experience

## Next Steps

Once all tests pass, you're ready for **Phase 3: Persistence**!
