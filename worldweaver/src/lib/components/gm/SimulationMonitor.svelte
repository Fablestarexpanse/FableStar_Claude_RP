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
    memoryUsage: 12.4, // MB
    memoryTotal: 64.0, // MB
    uptime: '0h 0m',
    cpuLoad: 12.1,
    gpuLoad: 96.0,
    temperature: 44,
    fanSpeed: 30,
    clockSpeed: 2775,
    powerDraw: 312.9,
    powerLimit: 480.0
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
      // For now, increment mock data with some variation
      stats.totalEvents = Math.floor(currentTick * 0.5);
      stats.cpuLoad = 10 + Math.random() * 5;
      stats.gpuLoad = 94 + Math.random() * 4;
      stats.temperature = 42 + Math.random() * 4;
      stats.fanSpeed = 28 + Math.random() * 5;
      stats.clockSpeed = 2700 + Math.random() * 150;
      stats.powerDraw = 300 + Math.random() * 30;
      stats.memoryUsage = 12 + Math.random() * 2;
      
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
  <!-- Header Title -->
  <div class="monitor-header">
    <h2>WorldWeaver Simulation Engine</h2>
    <div class="header-badge"># {currentTick.toLocaleString()}</div>
  </div>

  <!-- Primary Stats with Progress Bars -->
  <div class="stats-grid">
    <!-- CPU Load -->
    <div class="stat-card">
      <div class="stat-header">
        <span class="stat-icon">🖥️</span>
        <span class="stat-label">CPU Load</span>
        <span class="stat-value">{stats.cpuLoad.toFixed(1)}%</span>
      </div>
      <div class="progress-bar">
        <div class="progress-fill cpu" style="width: {stats.cpuLoad}%"></div>
      </div>
      <div class="stat-detail">{stats.cpuLoad.toFixed(2)}% / 100%</div>
    </div>

    <!-- GPU Load -->
    <div class="stat-card">
      <div class="stat-header">
        <span class="stat-icon">🎮</span>
        <span class="stat-label">GPU Load</span>
        <span class="stat-value">{stats.gpuLoad.toFixed(1)}%</span>
      </div>
      <div class="progress-bar">
        <div class="progress-fill gpu" style="width: {stats.gpuLoad}%"></div>
      </div>
      <div class="stat-detail">{stats.gpuLoad.toFixed(2)}% / 100%</div>
    </div>

    <!-- Memory -->
    <div class="stat-card">
      <div class="stat-header">
        <span class="stat-icon">💾</span>
        <span class="stat-label">Memory</span>
        <span class="stat-value">{((stats.memoryUsage / stats.memoryTotal) * 100).toFixed(1)}%</span>
      </div>
      <div class="progress-bar">
        <div class="progress-fill memory" style="width: {(stats.memoryUsage / stats.memoryTotal) * 100}%"></div>
      </div>
      <div class="stat-detail">{stats.memoryUsage.toFixed(1)} GB / {stats.memoryTotal.toFixed(1)} GB</div>
    </div>
  </div>

  <!-- Technical Readouts Grid -->
  <div class="readouts-grid">
    <!-- Temperature -->
    <div class="readout-card">
      <div class="readout-icon">🌡️</div>
      <div class="readout-content">
        <div class="readout-label">Temperature</div>
        <div class="readout-value">{stats.temperature}°C</div>
      </div>
    </div>

    <!-- Fan Speed -->
    <div class="readout-card">
      <div class="readout-icon">🌀</div>
      <div class="readout-content">
        <div class="readout-label">Fan Speed</div>
        <div class="readout-value">{stats.fanSpeed}%</div>
      </div>
    </div>

    <!-- Clock Speed -->
    <div class="readout-card">
      <div class="readout-icon">⚡</div>
      <div class="readout-content">
        <div class="readout-label">Clock Speed</div>
        <div class="readout-value">{stats.clockSpeed} MHz</div>
      </div>
    </div>

    <!-- Power Draw -->
    <div class="readout-card">
      <div class="readout-icon">🔌</div>
      <div class="readout-content">
        <div class="readout-label">Power Draw</div>
        <div class="readout-value">{stats.powerDraw.toFixed(1)}W / {stats.powerLimit.toFixed(1)}W</div>
      </div>
    </div>

    <!-- NPCs -->
    <div class="readout-card">
      <div class="readout-icon">👥</div>
      <div class="readout-content">
        <div class="readout-label">NPCs</div>
        <div class="readout-value">{stats.activeNpcs} / {stats.totalNpcs}</div>
      </div>
    </div>

    <!-- Rooms -->
    <div class="readout-card">
      <div class="readout-icon">🗺️</div>
      <div class="readout-content">
        <div class="readout-label">Rooms</div>
        <div class="readout-value">{stats.totalRooms}</div>
      </div>
    </div>

    <!-- Events -->
    <div class="readout-card">
      <div class="readout-icon">📜</div>
      <div class="readout-content">
        <div class="readout-label">Events</div>
        <div class="readout-value">{stats.totalEvents.toLocaleString()}</div>
      </div>
    </div>

    <!-- Uptime -->
    <div class="readout-card">
      <div class="readout-icon">⏰</div>
      <div class="readout-content">
        <div class="readout-label">Uptime</div>
        <div class="readout-value">{stats.uptime}</div>
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
    gap: 1rem;
    height: 100%;
    overflow-y: auto;
    padding: 0.5rem;
  }

  .monitor-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.75rem 1rem;
    background: hsl(220, 14%, 8%);
    border: 1px solid hsl(220, 10%, 18%);
    border-radius: 0.375rem;
  }

  .monitor-header h2 {
    margin: 0;
    font-size: 1rem;
    font-weight: 600;
    color: rgb(243, 244, 246);
  }

  .header-badge {
    font-size: 0.75rem;
    color: rgb(156, 163, 175);
    font-family: 'Courier New', monospace;
  }

  .stats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 0.75rem;
  }

  .stat-card {
    background: hsl(220, 14%, 8%);
    border: 1px solid hsl(220, 10%, 18%);
    border-radius: 0.375rem;
    padding: 0.875rem;
  }

  .stat-header {
    display: flex;
    align-items: center;
    gap: 0.625rem;
    margin-bottom: 0.625rem;
  }

  .stat-icon {
    font-size: 1rem;
  }

  .stat-label {
    flex: 1;
    font-size: 0.875rem;
    color: rgb(156, 163, 175);
    font-weight: 500;
  }

  .stat-value {
    font-size: 1rem;
    font-weight: 600;
    color: rgb(243, 244, 246);
    font-family: 'Courier New', monospace;
  }

  .progress-bar {
    height: 1.25rem;
    background: hsl(220, 14%, 14%);
    border-radius: 0.25rem;
    overflow: hidden;
    margin-bottom: 0.5rem;
  }

  .progress-fill {
    height: 100%;
    transition: width 0.3s ease;
    border-radius: 0.25rem;
  }

  .progress-fill.cpu {
    background: linear-gradient(90deg, rgb(59, 130, 246), rgb(96, 165, 250));
  }

  .progress-fill.gpu {
    background: linear-gradient(90deg, rgb(239, 68, 68), rgb(248, 113, 113));
  }

  .progress-fill.memory {
    background: linear-gradient(90deg, rgb(59, 130, 246), rgb(147, 197, 253));
  }

  .stat-detail {
    font-size: 0.75rem;
    color: rgb(156, 163, 175);
    font-family: 'Courier New', monospace;
  }

  .readouts-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
    gap: 0.75rem;
  }

  .readout-card {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.875rem;
    background: hsl(220, 14%, 8%);
    border: 1px solid hsl(220, 10%, 18%);
    border-radius: 0.375rem;
    transition: all 0.15s ease;
  }

  .readout-card:hover {
    border-color: rgb(59, 130, 246);
    background: hsl(220, 14%, 10%);
  }

  .readout-icon {
    font-size: 1.25rem;
  }

  .readout-content {
    flex: 1;
    min-width: 0;
  }

  .readout-label {
    font-size: 0.75rem;
    color: rgb(156, 163, 175);
    margin-bottom: 0.25rem;
  }

  .readout-value {
    font-size: 0.875rem;
    font-weight: 600;
    color: rgb(243, 244, 246);
    font-family: 'Courier New', monospace;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .control-panel,
  .health-panel,
  .graph-panel {
    background: hsl(220, 14%, 8%);
    border: 1px solid hsl(220, 10%, 18%);
    border-radius: 0.375rem;
    padding: 1rem;
  }

  .control-panel h3,
  .health-panel h3,
  .graph-panel h3 {
    margin: 0 0 1rem 0;
    color: rgb(243, 244, 246);
    font-size: 0.875rem;
    font-weight: 600;
  }

  .controls-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1rem;
  }

  .control-group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .control-group label {
    font-size: 0.75rem;
    color: rgb(156, 163, 175);
    font-weight: 500;
  }

  .btn-toggle {
    padding: 0.625rem 1rem;
    background: hsl(220, 14%, 14%);
    border: 1px solid hsl(220, 10%, 22%);
    border-radius: 0.375rem;
    color: rgb(243, 244, 246);
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .btn-toggle.running {
    background: rgba(34, 197, 94, 0.1);
    border-color: rgb(34, 197, 94);
    color: rgb(34, 197, 94);
  }

  .btn-toggle:hover {
    background: hsl(220, 14%, 16%);
    border-color: rgb(59, 130, 246);
  }

  .tick-rate-controls {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .tick-rate-display {
    flex: 1;
    text-align: center;
    padding: 0.625rem;
    background: hsl(220, 14%, 14%);
    border: 1px solid hsl(220, 10%, 22%);
    border-radius: 0.375rem;
    color: rgb(243, 244, 246);
    font-weight: 600;
    font-family: 'Courier New', monospace;
    font-size: 0.875rem;
  }

  .btn-small {
    padding: 0.5rem 0.875rem;
    background: hsl(220, 14%, 14%);
    border: 1px solid hsl(220, 10%, 22%);
    border-radius: 0.375rem;
    color: rgb(243, 244, 246);
    font-size: 1rem;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .btn-small:hover {
    background: hsl(220, 14%, 16%);
    border-color: rgb(59, 130, 246);
  }

  .btn-danger {
    padding: 0.625rem 1rem;
    background: rgba(239, 68, 68, 0.1);
    border: 1px solid rgba(239, 68, 68, 0.3);
    border-radius: 0.375rem;
    color: rgb(248, 113, 113);
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .btn-danger:hover {
    background: rgba(239, 68, 68, 0.2);
    border-color: rgb(239, 68, 68);
  }

  .health-grid {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .health-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.75rem;
    background: hsl(220, 14%, 14%);
    border: 1px solid hsl(220, 10%, 22%);
    border-radius: 0.375rem;
  }

  .health-label {
    display: flex;
    align-items: center;
    gap: 0.625rem;
    font-size: 0.875rem;
    color: rgb(243, 244, 246);
  }

  .health-icon {
    font-size: 1rem;
    color: rgb(156, 163, 175);
  }

  .health-icon.healthy {
    color: rgb(34, 197, 94);
  }

  .health-status {
    font-size: 0.75rem;
    color: rgb(34, 197, 94);
    font-weight: 600;
    text-transform: uppercase;
    font-family: 'Courier New', monospace;
  }

  .graph-placeholder {
    min-height: 150px;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    background: hsl(220, 14%, 14%);
    border: 1px dashed hsl(220, 10%, 22%);
    border-radius: 0.375rem;
    color: rgb(156, 163, 175);
  }

  .graph-placeholder p {
    margin: 0.25rem 0;
    font-size: 0.875rem;
  }

  .graph-placeholder .hint {
    font-size: 0.75rem;
  }
</style>
