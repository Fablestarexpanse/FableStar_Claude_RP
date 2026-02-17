# WorldWeaver World Map - Phase 2

## Visual Map

```
                    ┌─────────────────────┐
                    │  Merchant District  │
                    │                     │
                    │  Shops & Stalls     │
                    │  No NPCs (yet)      │
                    └──────────┬──────────┘
                               │
                            west/east
                               │
    ┌──────────────────┐       │       ┌──────────────────┐
    │ Blacksmith Forge │───────┼───────│   (Future Room)  │
    │                  │  west │ east  │                  │
    │  Kael (NPC)      │       │       │                  │
    │  Weapons & Tools │       │       │                  │
    └──────────────────┘       │       └──────────────────┘
                               │
                        ┌──────┴──────┐
                        │ Town Square │
                        │             │
                        │  Fountain   │
                        │  Merchants  │
                        └──────┬──────┘
                               │
                          north/south
                               │
                    ┌──────────┴──────────┐
                    │  Crossroads Inn     │
                    │  (STARTING ROOM)    │
                    │  Gareth (NPC)       │
                    │  Fireplace & Tables │
                    └─────────────────────┘
```

## Room Connections

### The Crossroads Inn (Start)
- **North** → Town Square
- **NPCs:** Gareth the Innkeeper
- **Description:** Cozy tavern with fireplace, ale, and warm atmosphere

### Town Square (Hub)
- **South** → The Crossroads Inn
- **East** → Merchant District
- **West** → Blacksmith's Forge
- **NPCs:** None (busy marketplace)
- **Description:** Central plaza with fountain, cobblestones, merchant stalls

### Merchant District
- **West** → Town Square
- **NPCs:** None (future expansion)
- **Description:** Narrow street with shops, spices, leather goods

### Blacksmith's Forge
- **East** → Town Square
- **NPCs:** Kael the Blacksmith
- **Description:** Hot workshop with forge, weapons, hammer sounds

## NPC Locations

| NPC | Location | Personality | Greeting |
|-----|----------|-------------|----------|
| **Gareth the Innkeeper** | Crossroads Inn | Friendly, talkative, knows gossip | "Welcome to the Crossroads! What can I get you?" |
| **Kael the Blacksmith** | Blacksmith's Forge | Direct, no-nonsense, skilled | "Looking for quality steel? You've come to the right place." |

## Navigation Guide

### From Inn to All Locations
```
Inn → north → Square (hub)
Square → east → Merchant District
Square → west → Blacksmith's Forge
```

### Full Circuit
```
Inn → north → Square → east → Merchant → west → Square → west → Forge → east → Square → south → Inn
```

### Shortcuts
```
n = north
s = south
e = east
w = west
```

## Room Atmosphere

### The Crossroads Inn
**Time:** Evening
**Lighting:** Warm firelight
**Sounds:** Crackling fire, quiet conversation
**Smells:** Roasted meat, ale
**Mood:** Cozy, welcoming

### Town Square
**Time:** Midday
**Lighting:** Bright sunlight
**Sounds:** Merchants calling, fountain splashing
**Smells:** Fresh bread, various goods
**Mood:** Busy, energetic

### Merchant District
**Time:** Afternoon
**Lighting:** Shaded by awnings
**Sounds:** Shopkeepers, customers bargaining
**Smells:** Spices, leather, fresh bread
**Mood:** Commercial, crowded

### Blacksmith's Forge
**Time:** Any
**Lighting:** Forge glow, dim otherwise
**Sounds:** Hammer on steel, bellows
**Smells:** Hot metal, coal smoke
**Mood:** Hot, industrious

## Future Expansion Points

### Planned Additions (Phase 3+)
- **North of Merchant District:** Residential area
- **East of Town Square:** Temple or guild hall
- **South of Inn:** Stables or town gate
- **Underground:** Cellar, catacombs, or sewers
- **Upstairs in Inn:** Guest rooms

### NPC Additions
- Merchant in Merchant District
- Town guard in Square
- Stable master near Inn
- Quest givers in various locations

## Distance & Travel Time

Currently instant travel (no time system yet).

**Phase 3+ will add:**
- Time progression per movement
- Day/night cycle affecting NPC locations
- Weather affecting travel
- Random encounters

## World Size Statistics

- **Total Rooms:** 4
- **Total NPCs:** 2
- **Total Exits:** 8 (bidirectional)
- **Explorable Area:** 100% accessible from start
- **Dead Ends:** 2 (Merchant District, Blacksmith's Forge)
- **Hub Rooms:** 1 (Town Square)

## Navigation Tips

1. **Town Square is the hub** - Most rooms connect through here
2. **Use shortcuts** - Type `n`, `s`, `e`, `w` instead of full words
3. **Check minimap** - Shows available exits
4. **Use `look`** - Reminds you where you are and what exits exist
5. **NPCs are stationary** - They stay in their rooms (for now)

## Lore Notes

**The Crossroads Inn** is named for its location at the crossroads of major trade routes. Gareth has run it for 20 years and knows everyone's business.

**Town Square** is the heart of the settlement, where merchants from across the region gather to trade. The fountain is said to have healing properties.

**Merchant District** grew organically as traders set up permanent shops. It's always crowded during market days.

**Blacksmith's Forge** belongs to Kael, who learned her trade from her father. She's the best smith within three days' travel and knows it.

---

**Current World:** 4 rooms, 2 NPCs, fully navigable
**Next Phase:** Persistence, more rooms, dynamic NPCs
