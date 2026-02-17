<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { getWorldTick } from '$lib/utils/tauri';
  
  let currentTick = $state(0);
  let tickRate = $state(1.0); // ticks per second
  let isRunning = $state(true);
  let updateInterval: number;
  
  // Simulation stats
  let stats = $state({
    totalNpcs: 2,
    activeNpcs: 2,
    totalRooms: 4,
    totalEvents: 0,
    memoryUsage: '12.4 MB',
    uptime: '0h 0m'
  });
  
  // System health
  let health = $state({
    simulation: 'healthy',
    database: 'healthy',
    eventLog: 'healthy'
  });
  
  onMount(() => {
    loadSimulationState();
    updateInterval = setInterval(loadSimulationState, 1000);
  });
  
  onDestroy(() => {
    if (updateInterval) {
      clearInterval(updateInterval);
    }
  });
  
  async function loadSimulationState() {
    try {
      currentTick = await getWorldTick();
      
      // TODO: Add backend commands for these stats
      // For now, increment mock data
      stats.totalEvents = Math.floor(currentTick * 0.5);
      
      // Calculate uptime
      const uptimeSeconds = Math.floor(currentTick / tickRate);
      const hours = Math.floor(uptimeSeconds / 3600);
      const minutes = Math.floor((uptimeSeconds % 3600) / 60);
      stats.uptime = `${hours}h ${minutes}m`;
      
    } catch (error) {
      console.error('Failed to load simulation state:', error);
    }
  }
  
  function toggleSimulation() {
    isRunning = !isRunning;
    // TODO: Add backend command to pause/resume simulation
  }
  
  function resetSimulation() {
    if (confirm('Reset simulation? This will clear all progress.')) {
      // TODO: Add backend command to reset simulation
      currentTick = 0;
      stats.totalEvents = 0;
      stats.uptime = '0h 0m';
    }
  }
  
  function adjustTickRate(delta: number) {
    tickRate = Math.max(0.1, Math.min(10, tickRate + delta));
    // TODO: Add backend command to adjust tick rate
  }
</script>

<div class="simulation-monitor">
  <div class="monitor-grid">
    <!-- Tick Counter -->
    <div class="monitor-card highlight">
      <div class="card-icon">⏱️</div>
      <div class="card-content">
        <div class="card-label">Current Tick</div>
        <div class="card-value large">{currentTick.toLocaleString()}</div>
        <div class="card-sublabel">{tickRate.toFixed(1)} ticks/sec</div>
      </div>
    </div>

    <!-- NPCs -->
    <div class="monitor-card">
      <div class="card-icon">👥</div>
      <div class="card-content">
        <div class="card-label">NPCs</div>
        <div class="card-value">{stats.activeNpcs} / {stats.totalNpcs}</div>
        <div class="card-sublabel">Active / Total</div>
      </div>
    </div>

    <!-- Rooms -->
    <div class="monitor-card">
      <div class="card-icon">🗺️</div>
      <div class="card-content">
        <div class="card-label">Rooms</div>
        <div class="card-value">{stats.totalRooms}</div>
        <div class="card-sublabel">Loaded</div>
      </div>
    </div>

    <!-- Events -->
    <div class="monitor-card">
      <div class="card-icon">📜</div>
      <div class="card-content">
        <div class="card-label">Events</div>
        <div class="card-value">{stats.totalEvents.toLocaleString()}</div>
        <div class="card-sublabel">Total Recorded</div>
      </div>
    </div>

    <!-- Memory -->
    <div class="monitor-card">
      <div class="card-icon">💾</div>
      <div class="card-content">
        <div class="card-label">Memory</div>
        <div class="card-value">{stats.memoryUsage}</div>
        <div class="card-sublabel">In Use</div>
      </div>
    </div>

    <!-- Uptime -->
    <div class="monitor-card">
      <div class="card-icon">⏰</div>
      <div class="card-content">
        <div class="card-label">Uptime</div>
        <div class="card-value">{stats.uptime}</div>
        <div class="card-sublabel">Running Time</div>
      </div>
    </div>
  </div>

  <!-- Control Panel -->
  <div class="control-panel">
    <h3>🎮 Simulation Controls</h3>
    
    <div class="controls-grid">
      <div class="control-group">
        <label>Simulation State:</label>
        <button
          class="btn-toggle"
          class:running={isRunning}
          onclick={toggleSimulation}
        >
          {isRunning ? '⏸️ Pause' : '▶️ Resume'}
        </button>
      </div>

      <div class="control-group">
        <label>Tick Rate:</label>
        <div class="tick-rate-controls">
          <button class="btn-small" onclick={() => adjustTickRate(-0.5)}>−</button>
          <span class="tick-rate-display">{tickRate.toFixed(1)} t/s</span>
          <button class="btn-small" onclick={() => adjustTickRate(0.5)}>+</button>
        </div>
      </div>

      <div class="control-group">
        <label>Actions:</label>
        <button class="btn-danger" onclick={resetSimulation}>
          🔄 Reset Simulation
        </button>
      </div>
    </div>
  </div>

  <!-- System Health -->
  <div class="health-panel">
    <h3>💚 System Health</h3>
    
    <div class="health-grid">
      <div class="health-item">
        <div class="health-label">
          <span class="health-icon" class:healthy={health.simulation === 'healthy'}>●</span>
          Simulation Engine
        </div>
        <div class="health-status">{health.simulation}</div>
      </div>

      <div class="health-item">
        <div class="health-label">
          <span class="health-icon" class:healthy={health.database === 'healthy'}>●</span>
          Database
        </div>
        <div class="health-status">{health.database}</div>
      </div>

      <div class="health-item">
        <div class="health-label">
          <span class="health-icon" class:healthy={health.eventLog === 'healthy'}>●</span>
          Event Log
        </div>
        <div class="health-status">{health.eventLog}</div>
      </div>
    </div>
  </div>

  <!-- Performance Graph Placeholder -->
  <div class="graph-panel">
    <h3>📈 Performance</h3>
    <div class="graph-placeholder">
      <p>Tick performance graph will appear here</p>
      <p class="hint">(Real-time tick execution time over last 60 seconds)</p>
    </div>
  </div>
</div>

<style>
  .simulation-monitor {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    height: 100%;
    overflow-y: auto;
  }

  .monitor-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1rem;
  }

  .monitor-card {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 1.5rem;
    background: rgba(20, 25, 35, 0.8);
    border: 1px solid rgba(139, 149, 165, 0.2);
    border-radius: 8px;
    transition: all 0.2s;
  }

  .monitor-card:hover {
    border-color: #ff6b35;
    transform: translateY(-2px);
  }

  .monitor-card.highlight {
    border-color: #ff6b35;
    background: rgba(255, 107, 53, 0.1);
  }

  .card-icon {
    font-size: 2rem;
  }

  .card-content {
    flex: 1;
  }

  .card-label {
    font-size: 0.85rem;
    color: #8b95a5;
    margin-bottom: 0.25rem;
  }

  .card-value {
    font-size: 1.5rem;
    font-weight: 600;
    color: #e0e6ed;
    line-height: 1.2;
  }

  .card-value.large {
    font-size: 2rem;
    color: #ff6b35;
  }

  .card-sublabel {
    font-size: 0.75rem;
    color: #8b95a5;
    margin-top: 0.25rem;
  }

  .control-panel,
  .health-panel,
  .graph-panel {
    background: rgba(20, 25, 35, 0.8);
    border: 1px solid rgba(139, 149, 165, 0.2);
    border-radius: 8px;
    padding: 1.5rem;
  }

  .control-panel h3,
  .health-panel h3,
  .graph-panel h3 {
    margin: 0 0 1.5rem 0;
    color: #ff6b35;
    font-size: 1.1rem;
  }

  .controls-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 1.5rem;
  }

  .control-group {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .control-group label {
    font-size: 0.9rem;
    color: #8b95a5;
    font-weight: 500;
  }

  .btn-toggle {
    padding: 0.75rem 1.5rem;
    background: rgba(139, 149, 165, 0.2);
    border: 1px solid rgba(139, 149, 165, 0.3);
    border-radius: 6px;
    color: #e0e6ed;
    font-size: 0.95rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
  }

  .btn-toggle.running {
    background: rgba(76, 175, 80, 0.2);
    border-color: #4caf50;
    color: #4caf50;
  }

  .btn-toggle:hover {
    transform: translateY(-1px);
  }

  .tick-rate-controls {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .tick-rate-display {
    flex: 1;
    text-align: center;
    padding: 0.75rem;
    background: rgba(15, 20, 30, 0.6);
    border: 1px solid rgba(139, 149, 165, 0.3);
    border-radius: 6px;
    color: #e0e6ed;
    font-weight: 600;
  }

  .btn-small {
    padding: 0.5rem 1rem;
    background: rgba(139, 149, 165, 0.2);
    border: 1px solid rgba(139, 149, 165, 0.3);
    border-radius: 6px;
    color: #e0e6ed;
    font-size: 1.2rem;
    cursor: pointer;
    transition: all 0.2s;
  }

  .btn-small:hover {
    background: rgba(139, 149, 165, 0.3);
  }

  .btn-danger {
    padding: 0.75rem 1.5rem;
    background: rgba(220, 53, 69, 0.2);
    border: 1px solid rgba(220, 53, 69, 0.3);
    border-radius: 6px;
    color: #ff6b6b;
    font-size: 0.95rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
  }

  .btn-danger:hover {
    background: rgba(220, 53, 69, 0.3);
  }

  .health-grid {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .health-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem;
    background: rgba(15, 20, 30, 0.4);
    border-radius: 6px;
  }

  .health-label {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    font-size: 0.95rem;
    color: #e0e6ed;
  }

  .health-icon {
    font-size: 1.2rem;
    color: #8b95a5;
  }

  .health-icon.healthy {
    color: #4caf50;
  }

  .health-status {
    font-size: 0.85rem;
    color: #4caf50;
    font-weight: 600;
    text-transform: uppercase;
  }

  .graph-placeholder {
    min-height: 200px;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    background: rgba(15, 20, 30, 0.4);
    border: 2px dashed rgba(139, 149, 165, 0.3);
    border-radius: 6px;
    color: #8b95a5;
  }

  .graph-placeholder p {
    margin: 0.25rem 0;
  }

  .graph-placeholder .hint {
    font-size: 0.85rem;
  }
</style>
