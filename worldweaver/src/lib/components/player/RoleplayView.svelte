<script lang="ts">
  import { onMount } from 'svelte';
  import { getCurrentRoom, getNpcsInCurrentRoom, movePlayer, sendPlayerAction } from '$lib/utils/tauri';
  import { currentRoom, currentNpcs, narrativeLog, isLoading, visitedRooms } from '$lib/stores/worldState';
  import MiniMap from './MiniMap.svelte';
  
  function navigateToGM() {
    window.location.href = '/gm';
  }
  
  let playerInput = $state('');
  let narrativeContainer: HTMLDivElement;
  
  // Movement directions
  const directions = ['north', 'n', 'south', 's', 'east', 'e', 'west', 'w', 'up', 'u', 'down', 'd'];
  
  async function loadRoom() {
    isLoading.set(true);
    try {
      const room = await getCurrentRoom();
      currentRoom.set(room);
      
      // Track visited room
      visitedRooms.update(visited => {
        visited.add(room.id);
        return visited;
      });
      
      // Load NPCs
      const npcs = await getNpcsInCurrentRoom();
      currentNpcs.set(npcs);
      
      // Build initial description
      let initialText = `${room.name}\n\n${room.description}`;
      
      if (room.exits.length > 0) {
        initialText += `\n\nObvious exits: ${room.exits.map(e => e.direction).join(', ')}`;
      }
      
      if (npcs.length > 0) {
        initialText += '\n\nYou see:';
        for (const npc of npcs) {
          initialText += `\n  - ${npc.name}`;
        }
      }
      
      narrativeLog.update(log => [...log, initialText]);
    } catch (error) {
      console.error('Failed to load room:', error);
      narrativeLog.update(log => [...log, `Error: ${error}`]);
    } finally {
      isLoading.set(false);
    }
  }
  
  async function handleMovement(direction: string) {
    isLoading.set(true);
    try {
      const newRoom = await movePlayer(direction);
      currentRoom.set(newRoom);
      
      // Track visited room
      visitedRooms.update(visited => {
        visited.add(newRoom.id);
        return visited;
      });
      
      // Load NPCs in new room
      const npcs = await getNpcsInCurrentRoom();
      currentNpcs.set(npcs);
      
      // Build movement narrative
      let movementText = `\nYou head ${direction}.\n\n${newRoom.name}\n\n${newRoom.description}`;
      
      if (newRoom.exits.length > 0) {
        movementText += `\n\nObvious exits: ${newRoom.exits.map(e => e.direction).join(', ')}`;
      }
      
      if (npcs.length > 0) {
        movementText += '\n\nYou see:';
        for (const npc of npcs) {
          movementText += `\n  - ${npc.name}`;
        }
      }
      
      narrativeLog.update(log => [...log, movementText]);
      
      // Scroll to bottom
      setTimeout(() => scrollToBottom(), 50);
      
    } catch (error) {
      const errorMsg = typeof error === 'string' ? error : String(error);
      narrativeLog.update(log => [...log, `\n${errorMsg}`]);
    } finally {
      isLoading.set(false);
    }
  }
  
  async function submitAction() {
    if (!playerInput.trim()) return;
    
    const input = playerInput.trim();
    const inputLower = input.toLowerCase();
    
    // Add player's action to the log
    narrativeLog.update(log => [...log, `\n> ${input}`]);
    
    // Check if it's a movement command
    if (directions.includes(inputLower)) {
      // Normalize shortened directions
      const fullDirection = inputLower === 'n' ? 'north' :
                           inputLower === 's' ? 'south' :
                           inputLower === 'e' ? 'east' :
                           inputLower === 'w' ? 'west' :
                           inputLower === 'u' ? 'up' :
                           inputLower === 'd' ? 'down' :
                           inputLower;
      
      playerInput = '';
      await handleMovement(fullDirection);
      return;
    }
    
    // Handle other actions
    isLoading.set(true);
    try {
      const response = await sendPlayerAction(input);
      narrativeLog.update(log => [...log, `\n${response}`]);
      
      setTimeout(() => scrollToBottom(), 50);
      
    } catch (error) {
      console.error('Action failed:', error);
      narrativeLog.update(log => [...log, `\nError: ${error}`]);
    } finally {
      playerInput = '';
      isLoading.set(false);
    }
  }
  
  function scrollToBottom() {
    if (narrativeContainer) {
      narrativeContainer.scrollTop = narrativeContainer.scrollHeight;
    }
  }
  
  function handleKeyPress(event: KeyboardEvent) {
    if (event.key === 'Enter' && !event.shiftKey) {
      event.preventDefault();
      submitAction();
    }
  }
  
  onMount(() => {
    loadRoom();
  });
</script>

<div class="roleplay-view">
  <header class="player-header">
    <div class="header-content">
      <h1 class="game-title">WorldWeaver</h1>
      <span class="header-subtitle">Persistent World RPG</span>
    </div>
    <button class="gm-button" onclick={navigateToGM}>
      🎭 GM Dashboard
    </button>
  </header>
  
  <div class="main-area">
    <div class="narrative-area" bind:this={narrativeContainer}>
      {#each $narrativeLog as entry}
        <p class="narrative-text">{entry}</p>
      {/each}
      
      {#if $isLoading}
        <p class="system-text">...</p>
      {/if}
    </div>
    
    <div class="input-area">
      <input
        type="text"
        bind:value={playerInput}
        onkeypress={handleKeyPress}
        placeholder="What do you do?"
        disabled={$isLoading}
        class="action-input"
      />
      <button onclick={submitAction} disabled={$isLoading || !playerInput.trim()} class="submit-btn">
        Send
      </button>
    </div>
  </div>
  
  <aside class="sidebar">
    <MiniMap />
  </aside>
</div>

<style>
  .roleplay-view {
    display: flex;
    flex-direction: column;
    height: 100vh;
    background: hsl(220, 14%, 10%);
    color: rgb(243, 244, 246);
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', system-ui, sans-serif;
  }
  
  .player-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.75rem 1.5rem;
    background: hsl(220, 14%, 14%);
    border-bottom: 1px solid hsl(220, 10%, 22%);
  }
  
  .header-content {
    display: flex;
    align-items: baseline;
    gap: 1rem;
  }
  
  .game-title {
    margin: 0;
    font-size: 1.25rem;
    font-weight: 600;
    color: rgb(243, 244, 246);
  }
  
  .header-subtitle {
    font-size: 0.875rem;
    color: rgb(156, 163, 175);
  }
  
  .gm-button {
    padding: 0.5rem 1rem;
    background: hsl(220, 14%, 14%);
    border: 1px solid hsl(220, 10%, 22%);
    border-radius: 0.375rem;
    color: rgb(243, 244, 246);
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.15s ease;
  }
  
  .gm-button:hover {
    background: hsl(220, 14%, 18%);
    border-color: rgb(59, 130, 246);
  }
  
  .gm-button:focus-visible {
    outline: none;
    box-shadow: 0 0 0 2px hsl(220, 14%, 10%), 0 0 0 4px rgb(59, 130, 246);
  }
  
  .main-area {
    display: grid;
    grid-template-columns: 1fr 280px;
    flex: 1;
    overflow: hidden;
    gap: 1px;
    background: hsl(220, 10%, 22%);
  }
  
  .narrative-area {
    background: hsl(220, 14%, 10%);
    overflow-y: auto;
    padding: 2rem;
    font-size: 1rem;
    line-height: 1.7;
    color: rgb(243, 244, 246);
  }
  
  .narrative-area::-webkit-scrollbar {
    width: 8px;
  }
  
  .narrative-area::-webkit-scrollbar-track {
    background: hsl(220, 14%, 10%);
  }
  
  .narrative-area::-webkit-scrollbar-thumb {
    background: hsl(220, 10%, 22%);
    border-radius: 4px;
  }
  
  .narrative-area::-webkit-scrollbar-thumb:hover {
    background: hsl(220, 10%, 28%);
  }
  
  .narrative-text {
    margin-bottom: 1.5rem;
    white-space: pre-wrap;
  }
  
  .system-text {
    color: rgb(156, 163, 175);
    font-style: italic;
  }
  
  .sidebar {
    background: hsl(220, 14%, 14%);
    overflow-y: auto;
  }
  
  .sidebar::-webkit-scrollbar {
    width: 8px;
  }
  
  .sidebar::-webkit-scrollbar-track {
    background: hsl(220, 14%, 14%);
  }
  
  .sidebar::-webkit-scrollbar-thumb {
    background: hsl(220, 10%, 22%);
    border-radius: 4px;
  }
  
  .input-area {
    display: flex;
    gap: 0.75rem;
    padding: 1rem 1.5rem;
    background: hsl(220, 14%, 14%);
    border-top: 1px solid hsl(220, 10%, 22%);
    grid-column: 1 / -1;
  }
  
  .action-input {
    flex: 1;
    padding: 0.625rem 0.875rem;
    background: hsl(220, 14%, 10%);
    border: 1px solid hsl(220, 10%, 22%);
    color: rgb(243, 244, 246);
    font-size: 0.875rem;
    border-radius: 0.375rem;
    transition: all 0.15s ease;
  }
  
  .action-input:hover {
    border-color: hsl(220, 10%, 28%);
  }
  
  .action-input:focus {
    outline: none;
    border-color: rgb(59, 130, 246);
    box-shadow: 0 0 0 2px hsl(220, 14%, 10%), 0 0 0 4px rgb(59, 130, 246);
  }
  
  .action-input::placeholder {
    color: rgb(107, 114, 128);
  }
  
  .submit-btn {
    padding: 0.625rem 1.5rem;
    background: rgb(59, 130, 246);
    color: white;
    border: none;
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
    border-radius: 0.375rem;
    transition: all 0.15s ease;
  }
  
  .submit-btn:hover:not(:disabled) {
    background: rgb(96, 165, 250);
  }
  
  .submit-btn:focus-visible {
    outline: none;
    box-shadow: 0 0 0 2px hsl(220, 14%, 10%), 0 0 0 4px rgb(59, 130, 246);
  }
  
  .submit-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>
