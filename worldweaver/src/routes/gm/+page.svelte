<script lang="ts">
  import { onMount } from 'svelte';
  import TerrainMapView from '$lib/components/terrain/TerrainMapView.svelte';
  import NpcManager from '$lib/components/gm/NpcManager.svelte';
  import SimulationMonitor from '$lib/components/gm/SimulationMonitor.svelte';
  import EventLogViewer from '$lib/components/gm/EventLogViewer.svelte';
  import TestingConsole from '$lib/components/gm/TestingConsole.svelte';
  
  let activeTab = $state<'map' | 'npcs' | 'monitor' | 'events' | 'console'>('map');
  
  const tabs: Array<{id: 'map' | 'npcs' | 'monitor' | 'events' | 'console', label: string, icon: string}> = [
    { id: 'map', label: '🗺️ World Map', icon: 'map' },
    { id: 'npcs', label: '👥 NPCs', icon: 'users' },
    { id: 'monitor', label: '📊 Monitor', icon: 'activity' },
    { id: 'events', label: '📜 Events', icon: 'list' },
    { id: 'console', label: '⚙️ Console', icon: 'terminal' }
  ];
</script>

<div class="gm-dashboard">
  <header class="dashboard-header">
    <div class="header-content">
      <h1>🎭 WorldWeaver GM Dashboard</h1>
      <p class="subtitle">Architect & Monitor Your Persistent World</p>
    </div>
    <div class="header-actions">
      <button class="btn-secondary" onclick={() => window.location.href = '/'}>
        🎮 Player View
      </button>
    </div>
  </header>

  <nav class="dashboard-tabs">
    {#each tabs as tab}
      <button
        class="tab"
        class:active={activeTab === tab.id}
        onclick={() => activeTab = tab.id}
      >
        {tab.label}
      </button>
    {/each}
  </nav>

  <main class="dashboard-content">
    {#if activeTab === 'map'}
      <TerrainMapView />
    {:else if activeTab === 'npcs'}
      <NpcManager />
    {:else if activeTab === 'monitor'}
      <SimulationMonitor />
    {:else if activeTab === 'events'}
      <EventLogViewer />
    {:else if activeTab === 'console'}
      <TestingConsole />
    {/if}
  </main>
</div>

<style>
  .gm-dashboard {
    display: flex;
    flex-direction: column;
    height: 100vh;
    background: hsl(220, 14%, 10%);
    color: rgb(243, 244, 246);
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', system-ui, sans-serif;
  }

  .dashboard-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem 1.5rem;
    background: hsl(220, 14%, 14%);
    border-bottom: 1px solid hsl(220, 10%, 22%);
  }

  .header-content h1 {
    margin: 0;
    font-size: 1.5rem;
    font-weight: 600;
    color: rgb(243, 244, 246);
  }

  .subtitle {
    margin: 0.25rem 0 0 0;
    font-size: 0.875rem;
    color: rgb(156, 163, 175);
  }

  .header-actions {
    display: flex;
    gap: 0.75rem;
  }

  .btn-secondary {
    padding: 0.5rem 1rem;
    background: hsl(220, 14%, 14%);
    border: 1px solid hsl(220, 10%, 22%);
    border-radius: 0.375rem;
    color: rgb(243, 244, 246);
    font-size: 0.875rem;
    cursor: pointer;
    transition: all 0.15s;
  }

  .btn-secondary:hover {
    background: hsl(220, 14%, 18%);
    border-color: rgb(59, 130, 246);
  }

  .btn-secondary:focus-visible {
    outline: none;
    box-shadow: 0 0 0 2px hsl(220, 14%, 10%), 0 0 0 4px rgb(59, 130, 246);
  }

  .dashboard-tabs {
    display: flex;
    gap: 0.25rem;
    padding: 0 1.5rem;
    background: hsl(220, 14%, 14%);
    border-bottom: 1px solid hsl(220, 10%, 22%);
  }

  .tab {
    padding: 0.75rem 1rem;
    background: transparent;
    border: none;
    border-bottom: 2px solid transparent;
    color: rgb(156, 163, 175);
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.15s;
    white-space: nowrap;
  }

  .tab:hover {
    color: rgb(243, 244, 246);
    background: hsl(220, 14%, 16%);
  }

  .tab.active {
    color: rgb(59, 130, 246);
    border-bottom-color: rgb(59, 130, 246);
  }

  .tab:focus-visible {
    outline: none;
    box-shadow: 0 0 0 2px hsl(220, 14%, 10%), 0 0 0 4px rgb(59, 130, 246);
  }

  .dashboard-content {
    flex: 1;
    overflow: hidden;
    padding: 1.5rem;
  }
</style>
