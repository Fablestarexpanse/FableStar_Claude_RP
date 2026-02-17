use bevy_ecs::prelude::*;

/// Resource to track world events that affect simulation
#[derive(Resource, Default)]
pub struct WorldEvents {
    pub events: Vec<WorldEvent>,
}

#[derive(Clone, Debug)]
pub struct WorldEvent {
    pub event_type: String,
    pub description: String,
    pub tick: u64,
}

/// Resource for world clock tracking time progression
#[derive(Resource)]
pub struct WorldClock {
    pub ticks_elapsed: u64,
    pub current_time: GameTime,
}

impl Default for WorldClock {
    fn default() -> Self {
        Self {
            ticks_elapsed: 0,
            current_time: GameTime::default(),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct GameTime {
    pub hour: u32,      // 0-23
    pub day: u32,       // 1-30
    pub month: u32,     // 1-12
    pub year: u32,      // Starting year
    pub season: Season,
}

#[derive(Clone, Debug, Default)]
pub enum Season {
    #[default]
    Spring,
    Summer,
    Autumn,
    Winter,
}

impl GameTime {
    /// Advance time by one tick (e.g., 1 hour)
    pub fn advance(&mut self, hours: u32) {
        self.hour += hours;
        
        if self.hour >= 24 {
            self.day += self.hour / 24;
            self.hour %= 24;
        }
        
        if self.day > 30 {
            self.month += self.day / 30;
            self.day = (self.day % 30).max(1);
        }
        
        if self.month > 12 {
            self.year += self.month / 12;
            self.month = (self.month % 12).max(1);
        }
        
        // Update season based on month
        self.season = match self.month {
            3..=5 => Season::Spring,
            6..=8 => Season::Summer,
            9..=11 => Season::Autumn,
            _ => Season::Winter,
        };
    }
}

/// System: Advance the world clock by one tick
pub fn advance_world_clock(mut clock: ResMut<WorldClock>) {
    clock.ticks_elapsed += 1;
    clock.current_time.advance(1); // 1 hour per tick
}

/// System: Update NPC schedules based on current time
/// NPCs move to scheduled locations at specific times
pub fn update_npc_schedules(
    _clock: Res<WorldClock>,
    // For MVP, we don't have Schedule component yet, so this is a placeholder
    // In future: mut npcs: Query<(&Npc, &Schedule, &mut Position)>
) {
    // TODO: Implement when Schedule component is added
    // For each NPC:
    //   - Check current time against schedule
    //   - If time matches a scheduled event, update Position
    //   - Log the movement as a WorldEvent
}

/// System: Simulate economy based on world events and time
/// Adjusts shop prices based on supply/demand
pub fn simulate_economy(
    _clock: Res<WorldClock>,
    _events: Res<WorldEvents>,
    // For MVP, we don't have Shop component yet
    // In future: mut shops: Query<(&mut Shop, &Position)>
) {
    // TODO: Implement when Shop component is added
    // For each shop:
    //   - Check recent events affecting trade routes
    //   - Adjust price_modifier based on supply/demand
    //   - Update inventory availability
}

/// System: Update faction relationships based on world events
pub fn update_faction_relations(
    _events: Res<WorldEvents>,
    // For MVP, we don't have Faction component yet
    // In future: mut factions: Query<&mut Faction>
) {
    // TODO: Implement when Faction component is added
    // Process events that affect faction relationships:
    //   - Player kills bandit -> bandit faction -10, town guard +5
    //   - Player completes quest -> quest giver faction +20
    //   - Faction alliances/wars affect related factions
}

/// System: Clean up old events to prevent memory bloat
pub fn cleanup_old_events(
    clock: Res<WorldClock>,
    mut events: ResMut<WorldEvents>,
) {
    // Keep only events from the last 1000 ticks
    let cutoff_tick = clock.ticks_elapsed.saturating_sub(1000);
    events.events.retain(|event| event.tick >= cutoff_tick);
}
