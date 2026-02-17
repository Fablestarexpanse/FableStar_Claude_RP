<script lang="ts">
  import { onMount } from 'svelte';
  
  interface GameEvent {
    id: string;
    tick: number;
    timestamp: string;
    type: string;
    description: string;
    tags: string[];
  }
  
  let events = $state<GameEvent[]>([]);
  let filteredEvents = $state<GameEvent[]>([]);
  let searchQuery = $state('');
  let selectedTags = $state<Set<string>>(new Set());
  let selectedEvent = $state<GameEvent | null>(null);
  
  // Available event types
  const eventTypes = [
    'player_moved',
    'npc_moved',
    'player_talked_to_npc',
    'item_picked_up',
    'combat_started',
    'time_advanced',
    'item_sold'
  ];
  
  // All tags found in events
  let availableTags = $state<string[]>([]);
  
  onMount(async () => {
    await loadEvents();
  });
  
  async function loadEvents() {
    try {
      // TODO: Add backend command to query events
      // Mock data for now
      events = [
        {
          id: '1',
          tick: 145,
          timestamp: new Date().toISOString(),
          type: 'player_moved',
          description: 'Player moved from Crossroads Inn to Town Square (north)',
          tags: ['player', 'movement']
        },
        {
          id: '2',
          tick: 146,
          timestamp: new Date().toISOString(),
          type: 'player_talked_to_npc',
          description: 'Player talked to Gareth the Innkeeper',
          tags: ['player', 'dialogue', 'npc:gareth']
        },
        {
          id: '3',
          tick: 150,
          timestamp: new Date().toISOString(),
          type: 'time_advanced',
          description: 'Time advanced from hour 10 to hour 11, day 1',
          tags: ['world', 'time']
        },
        {
          id: '4',
          tick: 152,
          timestamp: new Date().toISOString(),
          type: 'player_moved',
          description: 'Player moved from Town Square to Merchant District (east)',
          tags: ['player', 'movement']
        },
        {
          id: '5',
          tick: 160,
          timestamp: new Date().toISOString(),
          type: 'npc_moved',
          description: 'Kael the Blacksmith moved to Forge',
          tags: ['npc', 'movement', 'npc:kael']
        }
      ];
      
      // Extract all unique tags
      const tagSet = new Set<string>();
      events.forEach(event => {
        event.tags.forEach(tag => tagSet.add(tag));
      });
      availableTags = Array.from(tagSet).sort();
      
      filterEvents();
    } catch (error) {
      console.error('Failed to load events:', error);
    }
  }
  
  function filterEvents() {
    filteredEvents = events.filter(event => {
      // Search filter
      if (searchQuery) {
        const query = searchQuery.toLowerCase();
        const matchesSearch = 
          event.description.toLowerCase().includes(query) ||
          event.type.toLowerCase().includes(query) ||
          event.tags.some(tag => tag.toLowerCase().includes(query));
        
        if (!matchesSearch) return false;
      }
      
      // Tag filter
      if (selectedTags.size > 0) {
        const hasTag = event.tags.some(tag => selectedTags.has(tag));
        if (!hasTag) return false;
      }
      
      return true;
    });
  }
  
  function toggleTag(tag: string) {
    if (selectedTags.has(tag)) {
      selectedTags.delete(tag);
    } else {
      selectedTags.add(tag);
    }
    selectedTags = selectedTags; // Trigger reactivity
    filterEvents();
  }
  
  function clearFilters() {
    searchQuery = '';
    selectedTags.clear();
    selectedTags = selectedTags;
    filterEvents();
  }
  
  function selectEvent(event: GameEvent) {
    selectedEvent = event;
  }
  
  function getEventTypeColor(type: string): string {
    const colors: Record<string, string> = {
      player_moved: '#4caf50',
      npc_moved: '#2196f3',
      player_talked_to_npc: '#ff9800',
      item_picked_up: '#9c27b0',
      combat_started: '#f44336',
      time_advanced: '#607d8b',
      item_sold: '#ffc107'
    };
    return colors[type] || '#8b95a5';
  }
  
  $effect(() => {
    searchQuery;
    filterEvents();
  });
</script>

<div class="event-log-viewer">
  <div class="viewer-header">
    <div class="header-stats">
      <span class="stat">📜 {events.length} Total Events</span>
      <span class="stat">🔍 {filteredEvents.length} Filtered</span>
    </div>
    <button class="btn-secondary" onclick={loadEvents}>🔄 Refresh</button>
  </div>

  <div class="filters-panel">
    <div class="search-box">
      <input
        type="text"
        bind:value={searchQuery}
        placeholder="🔍 Search events..."
      />
    </div>

    <div class="tag-filters">
      <div class="filter-label">Filter by tags:</div>
      <div class="tag-list">
        {#each availableTags as tag}
          <button
            class="tag-button"
            class:selected={selectedTags.has(tag)}
            onclick={() => toggleTag(tag)}
          >
            {tag}
          </button>
        {/each}
      </div>
      {#if selectedTags.size > 0 || searchQuery}
        <button class="btn-clear" onclick={clearFilters}>Clear Filters</button>
      {/if}
    </div>
  </div>

  <div class="viewer-main">
    <div class="event-list">
      {#each filteredEvents as event}
        <div
          class="event-item"
          class:selected={selectedEvent?.id === event.id}
          onclick={() => selectEvent(event)}
        >
          <div class="event-header">
            <span
              class="event-type"
              style="background-color: {getEventTypeColor(event.type)}20; color: {getEventTypeColor(event.type)}"
            >
              {event.type}
            </span>
            <span class="event-tick">Tick {event.tick}</span>
          </div>
          <div class="event-description">{event.description}</div>
          <div class="event-tags">
            {#each event.tags as tag}
              <span class="event-tag">{tag}</span>
            {/each}
          </div>
        </div>
      {/each}
      
      {#if filteredEvents.length === 0}
        <div class="empty-state">
          <p>No events match your filters</p>
          <button class="btn-secondary" onclick={clearFilters}>Clear Filters</button>
        </div>
      {/if}
    </div>

    <aside class="event-details">
      {#if selectedEvent}
        <h3>Event Details</h3>
        
        <div class="detail-section">
          <label>ID:</label>
          <div class="detail-value mono">{selectedEvent.id}</div>
        </div>

        <div class="detail-section">
          <label>Tick:</label>
          <div class="detail-value">{selectedEvent.tick}</div>
        </div>

        <div class="detail-section">
          <label>Timestamp:</label>
          <div class="detail-value mono">{new Date(selectedEvent.timestamp).toLocaleString()}</div>
        </div>

        <div class="detail-section">
          <label>Type:</label>
          <div
            class="detail-value event-type-badge"
            style="background-color: {getEventTypeColor(selectedEvent.type)}20; color: {getEventTypeColor(selectedEvent.type)}"
          >
            {selectedEvent.type}
          </div>
        </div>

        <div class="detail-section">
          <label>Description:</label>
          <div class="detail-value">{selectedEvent.description}</div>
        </div>

        <div class="detail-section">
          <label>Tags:</label>
          <div class="detail-tags">
            {#each selectedEvent.tags as tag}
              <span class="detail-tag">{tag}</span>
            {/each}
          </div>
        </div>

        <div class="detail-actions">
          <button class="btn-secondary full-width">📋 Copy JSON</button>
        </div>
      {:else}
        <div class="detail-empty">
          <p>Select an event to view details</p>
        </div>
      {/if}
    </aside>
  </div>
</div>

<style>
  .event-log-viewer {
    display: flex;
    flex-direction: column;
    height: 100%;
    gap: 1rem;
  }

  .viewer-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem 1.5rem;
    background: rgba(20, 25, 35, 0.8);
    border-radius: 8px;
    border: 1px solid rgba(139, 149, 165, 0.2);
  }

  .header-stats {
    display: flex;
    gap: 2rem;
  }

  .stat {
    font-size: 0.95rem;
    color: #e0e6ed;
  }

  .filters-panel {
    background: rgba(20, 25, 35, 0.8);
    border-radius: 8px;
    border: 1px solid rgba(139, 149, 165, 0.2);
    padding: 1.5rem;
  }

  .search-box {
    margin-bottom: 1.5rem;
  }

  .search-box input {
    width: 100%;
    padding: 0.75rem;
    background: rgba(15, 20, 30, 0.6);
    border: 1px solid rgba(139, 149, 165, 0.3);
    border-radius: 6px;
    color: #e0e6ed;
    font-size: 0.95rem;
  }

  .tag-filters {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .filter-label {
    font-size: 0.9rem;
    color: #8b95a5;
    font-weight: 500;
  }

  .tag-list {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
  }

  .tag-button {
    padding: 0.4rem 0.9rem;
    background: rgba(139, 149, 165, 0.2);
    border: 1px solid rgba(139, 149, 165, 0.3);
    border-radius: 16px;
    color: #e0e6ed;
    font-size: 0.85rem;
    cursor: pointer;
    transition: all 0.2s;
  }

  .tag-button:hover {
    background: rgba(139, 149, 165, 0.3);
  }

  .tag-button.selected {
    background: rgba(255, 107, 53, 0.2);
    border-color: #ff6b35;
    color: #ff6b35;
  }

  .btn-clear {
    padding: 0.5rem 1rem;
    background: rgba(220, 53, 69, 0.2);
    border: 1px solid rgba(220, 53, 69, 0.3);
    border-radius: 6px;
    color: #ff6b6b;
    font-size: 0.85rem;
    cursor: pointer;
    align-self: flex-start;
  }

  .viewer-main {
    display: grid;
    grid-template-columns: 1fr 350px;
    gap: 1rem;
    flex: 1;
    overflow: hidden;
  }

  .event-list {
    background: rgba(20, 25, 35, 0.8);
    border-radius: 8px;
    border: 1px solid rgba(139, 149, 165, 0.2);
    padding: 1rem;
    overflow-y: auto;
  }

  .event-item {
    padding: 1rem;
    background: rgba(15, 20, 30, 0.4);
    border: 1px solid rgba(139, 149, 165, 0.2);
    border-radius: 6px;
    margin-bottom: 0.75rem;
    cursor: pointer;
    transition: all 0.2s;
  }

  .event-item:hover {
    background: rgba(15, 20, 30, 0.6);
    border-color: #ff6b35;
  }

  .event-item.selected {
    background: rgba(255, 107, 53, 0.15);
    border-color: #ff6b35;
  }

  .event-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.5rem;
  }

  .event-type {
    padding: 0.25rem 0.75rem;
    border-radius: 12px;
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
  }

  .event-tick {
    font-size: 0.8rem;
    color: #8b95a5;
    font-family: 'Courier New', monospace;
  }

  .event-description {
    margin-bottom: 0.5rem;
    font-size: 0.9rem;
    color: #e0e6ed;
    line-height: 1.4;
  }

  .event-tags {
    display: flex;
    flex-wrap: wrap;
    gap: 0.4rem;
  }

  .event-tag {
    padding: 0.2rem 0.6rem;
    background: rgba(139, 149, 165, 0.2);
    border-radius: 10px;
    font-size: 0.7rem;
    color: #8b95a5;
  }

  .event-details {
    background: rgba(20, 25, 35, 0.8);
    border-radius: 8px;
    border: 1px solid rgba(139, 149, 165, 0.2);
    padding: 1.5rem;
    overflow-y: auto;
  }

  .event-details h3 {
    margin: 0 0 1.5rem 0;
    color: #ff6b35;
    font-size: 1.1rem;
  }

  .detail-section {
    margin-bottom: 1.5rem;
  }

  .detail-section label {
    display: block;
    margin-bottom: 0.5rem;
    font-size: 0.85rem;
    color: #8b95a5;
    font-weight: 500;
  }

  .detail-value {
    padding: 0.6rem;
    background: rgba(15, 20, 30, 0.4);
    border-radius: 4px;
    color: #e0e6ed;
    font-size: 0.9rem;
  }

  .detail-value.mono {
    font-family: 'Courier New', monospace;
    font-size: 0.85rem;
  }

  .event-type-badge {
    display: inline-block;
    padding: 0.4rem 0.9rem;
    border-radius: 12px;
    font-weight: 600;
    font-size: 0.85rem;
  }

  .detail-tags {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
  }

  .detail-tag {
    padding: 0.4rem 0.8rem;
    background: rgba(139, 149, 165, 0.2);
    border-radius: 12px;
    font-size: 0.8rem;
    color: #e0e6ed;
  }

  .detail-actions {
    margin-top: 2rem;
  }

  .detail-empty {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: #8b95a5;
    text-align: center;
  }

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 3rem;
    color: #8b95a5;
    gap: 1rem;
  }

  .btn-secondary {
    padding: 0.6rem 1.2rem;
    background: rgba(139, 149, 165, 0.2);
    border: 1px solid rgba(139, 149, 165, 0.3);
    border-radius: 6px;
    color: #e0e6ed;
    font-size: 0.9rem;
    cursor: pointer;
    transition: all 0.2s;
  }

  .btn-secondary:hover {
    background: rgba(139, 149, 165, 0.3);
  }

  .full-width {
    width: 100%;
  }
</style>
