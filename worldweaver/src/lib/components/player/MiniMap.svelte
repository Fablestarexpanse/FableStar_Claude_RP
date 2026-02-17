<script lang="ts">
  import { currentRoom, visitedRooms } from '$lib/stores/worldState';
  
  // Simple visualization of explored rooms
  $: roomName = $currentRoom?.name || 'Unknown';
  $: exits = $currentRoom?.exits || [];
  $: explored = $visitedRooms.size;
</script>

<div class="minimap">
  <div class="minimap-header">
    <h3>Location</h3>
  </div>
  
  <div class="current-room">
    <div class="room-indicator">●</div>
    <div class="room-name">{roomName}</div>
  </div>
  
  <div class="exits-list">
    <strong>Available exits:</strong>
    {#if exits.length > 0}
      <ul>
        {#each exits as exit}
          <li>{exit.direction}</li>
        {/each}
      </ul>
    {:else}
      <p class="no-exits">No obvious exits</p>
    {/if}
  </div>
  
  <div class="stats">
    <small>Rooms explored: {explored}</small>
  </div>
</div>

<style>
  .minimap {
    padding: 1.25rem;
    height: 100%;
    display: flex;
    flex-direction: column;
    gap: 1.25rem;
  }
  
  .minimap-header h3 {
    font-size: 0.875rem;
    font-weight: 600;
    color: rgb(243, 244, 246);
    margin: 0 0 0.75rem 0;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    border-bottom: 1px solid hsl(220, 10%, 22%);
    padding-bottom: 0.75rem;
  }
  
  .current-room {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.75rem;
    background: hsl(220, 14%, 10%);
    border: 1px solid hsl(220, 10%, 22%);
    border-radius: 0.375rem;
  }
  
  .room-indicator {
    color: rgb(59, 130, 246);
    font-size: 1.25rem;
    line-height: 1;
  }
  
  .room-name {
    font-weight: 500;
    color: rgb(243, 244, 246);
    font-size: 0.875rem;
  }
  
  .exits-list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }
  
  .exits-list strong {
    font-size: 0.75rem;
    color: rgb(156, 163, 175);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }
  
  .exits-list ul {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
  }
  
  .exits-list li {
    color: rgb(59, 130, 246);
    font-size: 0.875rem;
    padding: 0.5rem;
    background: hsl(220, 14%, 10%);
    border: 1px solid hsl(220, 10%, 22%);
    border-radius: 0.375rem;
    transition: all 0.15s ease;
  }
  
  .exits-list li:hover {
    border-color: rgb(59, 130, 246);
    background: rgba(59, 130, 246, 0.1);
  }
  
  .exits-list li::before {
    content: "→ ";
    margin-right: 0.5rem;
  }
  
  .no-exits {
    font-style: italic;
    color: rgb(107, 114, 128);
    font-size: 0.875rem;
    padding: 0.5rem;
  }
  
  .stats {
    color: rgb(156, 163, 175);
    font-size: 0.75rem;
    border-top: 1px solid hsl(220, 10%, 22%);
    padding-top: 0.75rem;
    margin-top: auto;
  }
</style>
