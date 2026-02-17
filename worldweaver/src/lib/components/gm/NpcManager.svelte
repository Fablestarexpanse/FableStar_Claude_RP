<script lang="ts">
  import { onMount } from 'svelte';
  import type { NpcInfo } from '$lib/utils/tauri';
  
  interface NpcData extends NpcInfo {
    id: string;
    location: string;
    stats?: {
      strength: number;
      dexterity: number;
      intelligence: number;
      charisma: number;
      constitution: number;
    };
    schedule?: ScheduleEntry[];
  }
  
  interface ScheduleEntry {
    priority: number;
    startHour: number;
    endHour: number;
    activity: string;
    location: string;
  }
  
  let npcs = $state<NpcData[]>([]);
  let selectedNpc = $state<NpcData | null>(null);
  let isCreatingNpc = $state(false);
  let searchQuery = $state('');
  
  // New NPC form
  let newNpc = $state({
    name: '',
    description: '',
    personality: '',
    greeting: '',
    location: '',
    stats: {
      strength: 10,
      dexterity: 10,
      intelligence: 10,
      charisma: 10,
      constitution: 10
    }
  });
  
  onMount(async () => {
    await loadNpcs();
  });
  
  async function loadNpcs() {
    try {
      // TODO: Add backend command to get all NPCs
      // Mock data for now
      npcs = [
        {
          id: '1',
          name: 'Gareth the Innkeeper',
          description: 'A burly man with a warm smile',
          personality: 'Friendly and welcoming, loves to chat with travelers',
          greeting: 'Welcome to the Crossroads Inn, friend!',
          location: 'Crossroads Inn',
          stats: {
            strength: 12,
            dexterity: 8,
            intelligence: 10,
            charisma: 14,
            constitution: 11
          },
          schedule: [
            { priority: 10, startHour: 6, endHour: 23, activity: 'Tending bar', location: 'Crossroads Inn' },
            { priority: 5, startHour: 23, endHour: 6, activity: 'Sleeping', location: 'Innkeeper Quarters' }
          ]
        },
        {
          id: '2',
          name: 'Kael the Blacksmith',
          description: 'A muscular dwarf covered in soot',
          personality: 'Gruff but fair, takes pride in his craft',
          greeting: 'Need something forged?',
          location: "Blacksmith's Forge",
          stats: {
            strength: 16,
            dexterity: 12,
            intelligence: 11,
            charisma: 8,
            constitution: 15
          },
          schedule: [
            { priority: 10, startHour: 8, endHour: 18, activity: 'Forging', location: "Blacksmith's Forge" },
            { priority: 5, startHour: 18, endHour: 8, activity: 'Resting', location: 'Forge Quarters' }
          ]
        }
      ];
    } catch (error) {
      console.error('Failed to load NPCs:', error);
    }
  }
  
  function selectNpc(npc: NpcData) {
    selectedNpc = npc;
  }
  
  function createNpc() {
    const npc: NpcData = {
      id: crypto.randomUUID(),
      name: newNpc.name,
      description: newNpc.description,
      personality: newNpc.personality,
      greeting: newNpc.greeting,
      location: newNpc.location,
      stats: { ...newNpc.stats },
      schedule: []
    };
    
    npcs.push(npc);
    selectedNpc = npc;
    isCreatingNpc = false;
    
    // Reset form
    newNpc = {
      name: '',
      description: '',
      personality: '',
      greeting: '',
      location: '',
      stats: {
        strength: 10,
        dexterity: 10,
        intelligence: 10,
        charisma: 10,
        constitution: 10
      }
    };
  }
  
  function deleteNpc() {
    if (!selectedNpc) return;
    npcs = npcs.filter(n => n.id !== selectedNpc!.id);
    selectedNpc = null;
  }
  
  function addScheduleEntry() {
    if (!selectedNpc) return;
    
    if (!selectedNpc.schedule) {
      selectedNpc.schedule = [];
    }
    
    selectedNpc.schedule.push({
      priority: 5,
      startHour: 9,
      endHour: 17,
      activity: 'Working',
      location: selectedNpc.location
    });
  }
  
  function removeScheduleEntry(index: number) {
    if (!selectedNpc?.schedule) return;
    selectedNpc.schedule.splice(index, 1);
  }
  
  let filteredNpcs = $derived(npcs.filter(npc => 
    npc.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
    npc.location.toLowerCase().includes(searchQuery.toLowerCase())
  ));
</script>

<div class="npc-manager">
  <div class="npc-list-panel">
    <div class="panel-header">
      <h3>👥 NPCs ({npcs.length})</h3>
      <button class="btn-primary" onclick={() => isCreatingNpc = true}>
        ➕ Create NPC
      </button>
    </div>
    
    <div class="search-box">
      <input
        type="text"
        bind:value={searchQuery}
        placeholder="🔍 Search NPCs..."
      />
    </div>
    
    <div class="npc-list">
      {#each filteredNpcs as npc}
        <div
          class="npc-card"
          class:selected={selectedNpc?.id === npc.id}
          onclick={() => selectNpc(npc)}
        >
          <div class="npc-card-header">
            <strong>{npc.name}</strong>
            <span class="npc-location">📍 {npc.location}</span>
          </div>
          <p class="npc-description">{npc.description}</p>
        </div>
      {/each}
    </div>
  </div>

  <div class="npc-details-panel">
    {#if selectedNpc}
      <div class="details-header">
        <h2>{selectedNpc.name}</h2>
        <button class="btn-danger" onclick={deleteNpc}>🗑️ Delete</button>
      </div>

      <div class="details-tabs">
        <div class="tab active">Basic Info</div>
        <div class="tab">Stats</div>
        <div class="tab">Schedule</div>
        <div class="tab">Relationships</div>
      </div>

      <div class="details-content">
        <!-- Basic Info -->
        <div class="form-section">
          <label>Name:</label>
          <input type="text" bind:value={selectedNpc.name} />
        </div>

        <div class="form-section">
          <label>Description:</label>
          <textarea bind:value={selectedNpc.description} rows="3"></textarea>
        </div>

        <div class="form-section">
          <label>Personality:</label>
          <textarea bind:value={selectedNpc.personality} rows="3"></textarea>
        </div>

        <div class="form-section">
          <label>Greeting:</label>
          <input type="text" bind:value={selectedNpc.greeting} />
        </div>

        <div class="form-section">
          <label>Current Location:</label>
          <input type="text" bind:value={selectedNpc.location} />
        </div>

        <!-- Stats -->
        {#if selectedNpc.stats}
          <div class="stats-grid">
            <div class="stat-item">
              <label>💪 Strength:</label>
              <input type="number" bind:value={selectedNpc.stats.strength} min="1" max="20" />
            </div>
            <div class="stat-item">
              <label>🏃 Dexterity:</label>
              <input type="number" bind:value={selectedNpc.stats.dexterity} min="1" max="20" />
            </div>
            <div class="stat-item">
              <label>🧠 Intelligence:</label>
              <input type="number" bind:value={selectedNpc.stats.intelligence} min="1" max="20" />
            </div>
            <div class="stat-item">
              <label>💬 Charisma:</label>
              <input type="number" bind:value={selectedNpc.stats.charisma} min="1" max="20" />
            </div>
            <div class="stat-item">
              <label>❤️ Constitution:</label>
              <input type="number" bind:value={selectedNpc.stats.constitution} min="1" max="20" />
            </div>
          </div>
        {/if}

        <!-- Schedule -->
        <div class="schedule-section">
          <div class="section-header">
            <h4>📅 Daily Schedule</h4>
            <button class="btn-small" onclick={addScheduleEntry}>➕ Add Entry</button>
          </div>
          
          {#if selectedNpc.schedule && selectedNpc.schedule.length > 0}
            <div class="schedule-list">
              {#each selectedNpc.schedule as entry, index}
                <div class="schedule-entry">
                  <div class="schedule-time">
                    <input type="number" bind:value={entry.startHour} min="0" max="23" />
                    <span>to</span>
                    <input type="number" bind:value={entry.endHour} min="0" max="23" />
                  </div>
                  <input
                    type="text"
                    bind:value={entry.activity}
                    placeholder="Activity"
                    class="schedule-activity"
                  />
                  <input
                    type="text"
                    bind:value={entry.location}
                    placeholder="Location"
                    class="schedule-location"
                  />
                  <input
                    type="number"
                    bind:value={entry.priority}
                    min="1"
                    max="10"
                    class="schedule-priority"
                    title="Priority (1-10)"
                  />
                  <button class="btn-icon-danger" onclick={() => removeScheduleEntry(index)}>
                    🗑️
                  </button>
                </div>
              {/each}
            </div>
          {:else}
            <p class="empty-state">No schedule entries. Add one to get started.</p>
          {/if}
        </div>

        <button class="btn-primary full-width save-btn">💾 Save Changes</button>
      </div>
    {:else}
      <div class="empty-selection">
        <p>Select an NPC to view and edit details</p>
      </div>
    {/if}
  </div>
</div>

{#if isCreatingNpc}
  <div class="modal-overlay" onclick={() => isCreatingNpc = false}>
    <div class="modal" onclick={(e) => e.stopPropagation()}>
      <h2>Create New NPC</h2>
      
      <div class="form-group">
        <label>Name:</label>
        <input type="text" bind:value={newNpc.name} placeholder="Elara the Merchant" />
      </div>

      <div class="form-group">
        <label>Description:</label>
        <textarea bind:value={newNpc.description} rows="2" placeholder="A shrewd trader with keen eyes"></textarea>
      </div>

      <div class="form-group">
        <label>Personality:</label>
        <textarea bind:value={newNpc.personality} rows="2" placeholder="Cunning and business-minded"></textarea>
      </div>

      <div class="form-group">
        <label>Greeting:</label>
        <input type="text" bind:value={newNpc.greeting} placeholder="Looking to buy or sell?" />
      </div>

      <div class="form-group">
        <label>Starting Location:</label>
        <input type="text" bind:value={newNpc.location} placeholder="Merchant District" />
      </div>

      <div class="modal-actions">
        <button class="btn-secondary" onclick={() => isCreatingNpc = false}>Cancel</button>
        <button class="btn-primary" onclick={createNpc}>Create</button>
      </div>
    </div>
  </div>
{/if}

<style>
  .npc-manager {
    display: grid;
    grid-template-columns: 350px 1fr;
    gap: 1.5rem;
    height: 100%;
  }

  .npc-list-panel {
    display: flex;
    flex-direction: column;
    background: rgba(20, 25, 35, 0.8);
    border-radius: 8px;
    border: 1px solid rgba(139, 149, 165, 0.2);
    overflow: hidden;
  }

  .panel-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1.25rem;
    border-bottom: 1px solid rgba(139, 149, 165, 0.2);
  }

  .panel-header h3 {
    margin: 0;
    color: #ff6b35;
    font-size: 1.1rem;
  }

  .search-box {
    padding: 1rem;
    border-bottom: 1px solid rgba(139, 149, 165, 0.2);
  }

  .search-box input {
    width: 100%;
    padding: 0.6rem;
    background: rgba(15, 20, 30, 0.6);
    border: 1px solid rgba(139, 149, 165, 0.3);
    border-radius: 6px;
    color: #e0e6ed;
    font-size: 0.9rem;
  }

  .npc-list {
    flex: 1;
    overflow-y: auto;
    padding: 0.75rem;
  }

  .npc-card {
    padding: 1rem;
    background: rgba(15, 20, 30, 0.4);
    border: 1px solid rgba(139, 149, 165, 0.2);
    border-radius: 6px;
    margin-bottom: 0.75rem;
    cursor: pointer;
    transition: all 0.2s;
  }

  .npc-card:hover {
    background: rgba(15, 20, 30, 0.6);
    border-color: #ff6b35;
  }

  .npc-card.selected {
    background: rgba(255, 107, 53, 0.15);
    border-color: #ff6b35;
  }

  .npc-card-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.5rem;
  }

  .npc-card-header strong {
    color: #e0e6ed;
    font-size: 0.95rem;
  }

  .npc-location {
    font-size: 0.75rem;
    color: #8b95a5;
  }

  .npc-description {
    margin: 0;
    font-size: 0.85rem;
    color: #8b95a5;
    line-height: 1.4;
  }

  .npc-details-panel {
    background: rgba(20, 25, 35, 0.8);
    border-radius: 8px;
    border: 1px solid rgba(139, 149, 165, 0.2);
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }

  .details-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1.5rem;
    border-bottom: 1px solid rgba(139, 149, 165, 0.2);
  }

  .details-header h2 {
    margin: 0;
    color: #ff6b35;
    font-size: 1.4rem;
  }

  .details-tabs {
    display: flex;
    gap: 0.5rem;
    padding: 1rem 1.5rem 0 1.5rem;
    border-bottom: 1px solid rgba(139, 149, 165, 0.2);
  }

  .tab {
    padding: 0.6rem 1rem;
    background: transparent;
    border: none;
    border-bottom: 2px solid transparent;
    color: #8b95a5;
    font-size: 0.9rem;
    cursor: pointer;
    transition: all 0.2s;
  }

  .tab.active {
    color: #ff6b35;
    border-bottom-color: #ff6b35;
  }

  .details-content {
    flex: 1;
    overflow-y: auto;
    padding: 1.5rem;
  }

  .form-section {
    margin-bottom: 1.5rem;
  }

  .form-section label {
    display: block;
    margin-bottom: 0.5rem;
    color: #8b95a5;
    font-size: 0.9rem;
    font-weight: 500;
  }

  .form-section input,
  .form-section textarea {
    width: 100%;
    padding: 0.6rem;
    background: rgba(15, 20, 30, 0.6);
    border: 1px solid rgba(139, 149, 165, 0.3);
    border-radius: 4px;
    color: #e0e6ed;
    font-family: inherit;
    font-size: 0.9rem;
  }

  .stats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
    gap: 1rem;
    margin: 1.5rem 0;
  }

  .stat-item label {
    display: block;
    margin-bottom: 0.4rem;
    color: #8b95a5;
    font-size: 0.85rem;
  }

  .stat-item input {
    width: 100%;
    padding: 0.5rem;
    background: rgba(15, 20, 30, 0.6);
    border: 1px solid rgba(139, 149, 165, 0.3);
    border-radius: 4px;
    color: #e0e6ed;
    text-align: center;
  }

  .schedule-section {
    margin-top: 2rem;
  }

  .section-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }

  .section-header h4 {
    margin: 0;
    color: #e0e6ed;
    font-size: 1rem;
  }

  .schedule-list {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .schedule-entry {
    display: grid;
    grid-template-columns: 120px 1fr 1fr 60px 40px;
    gap: 0.5rem;
    align-items: center;
    padding: 0.75rem;
    background: rgba(15, 20, 30, 0.4);
    border-radius: 4px;
  }

  .schedule-time {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    font-size: 0.85rem;
  }

  .schedule-time input {
    width: 45px;
    padding: 0.3rem;
    text-align: center;
    background: rgba(15, 20, 30, 0.6);
    border: 1px solid rgba(139, 149, 165, 0.3);
    border-radius: 3px;
    color: #e0e6ed;
  }

  .schedule-activity,
  .schedule-location {
    padding: 0.4rem;
    background: rgba(15, 20, 30, 0.6);
    border: 1px solid rgba(139, 149, 165, 0.3);
    border-radius: 3px;
    color: #e0e6ed;
    font-size: 0.85rem;
  }

  .schedule-priority {
    width: 100%;
    padding: 0.4rem;
    text-align: center;
    background: rgba(15, 20, 30, 0.6);
    border: 1px solid rgba(139, 149, 165, 0.3);
    border-radius: 3px;
    color: #e0e6ed;
  }

  .empty-state {
    text-align: center;
    padding: 2rem;
    color: #8b95a5;
    font-size: 0.9rem;
  }

  .empty-selection {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: #8b95a5;
    font-size: 1.1rem;
  }

  .btn-primary, .btn-secondary, .btn-danger, .btn-small, .btn-icon-danger {
    padding: 0.6rem 1.2rem;
    border: none;
    border-radius: 6px;
    font-size: 0.9rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
  }

  .btn-primary {
    background: #ff6b35;
    color: white;
  }

  .btn-primary:hover {
    background: #ff8555;
  }

  .btn-secondary {
    background: rgba(139, 149, 165, 0.2);
    color: #e0e6ed;
  }

  .btn-danger {
    background: rgba(220, 53, 69, 0.2);
    color: #ff6b6b;
  }

  .btn-small {
    padding: 0.4rem 0.8rem;
    font-size: 0.85rem;
    background: rgba(139, 149, 165, 0.2);
    color: #e0e6ed;
  }

  .btn-icon-danger {
    padding: 0.3rem 0.6rem;
    background: transparent;
    color: #ff6b6b;
  }

  .full-width {
    width: 100%;
  }

  .save-btn {
    margin-top: 2rem;
  }

  .modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.7);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .modal {
    background: #1a1f2e;
    border-radius: 12px;
    border: 2px solid #ff6b35;
    padding: 2rem;
    width: 90%;
    max-width: 600px;
    max-height: 90vh;
    overflow-y: auto;
  }

  .modal h2 {
    margin: 0 0 1.5rem 0;
    color: #ff6b35;
  }

  .form-group {
    margin-bottom: 1.25rem;
  }

  .form-group label {
    display: block;
    margin-bottom: 0.5rem;
    color: #8b95a5;
    font-weight: 500;
  }

  .form-group input,
  .form-group textarea {
    width: 100%;
    padding: 0.7rem;
    background: rgba(15, 20, 30, 0.6);
    border: 1px solid rgba(139, 149, 165, 0.3);
    border-radius: 6px;
    color: #e0e6ed;
    font-family: inherit;
  }

  .modal-actions {
    display: flex;
    gap: 1rem;
    justify-content: flex-end;
    margin-top: 2rem;
  }
</style>
