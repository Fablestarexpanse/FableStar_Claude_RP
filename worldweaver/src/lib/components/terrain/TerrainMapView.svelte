<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import { TerrainRenderer } from './TerrainRenderer';
  import { TerrainViewport } from './TerrainViewport';
  import { BrushPreview } from './BrushPreview';
  import type { TerrainConfig, GenerateTerrainRequest, GenerateTerrainResponse, ViewTransform, VisibleChunks, BrushType } from './types';

  let terrainCanvas: HTMLCanvasElement;
  let previewCanvas: HTMLCanvasElement;
  let renderer: TerrainRenderer | null = null;
  let viewport: TerrainViewport | null = null;
  let brushPreview: BrushPreview | null = null;

  let config: TerrainConfig | null = null;
  let isGenerating = $state(false);
  let generationMessage = $state('');
  let generationProgress = $state(0);
  let generationStage = $state('');
  let activeTool: BrushType | null = $state(null);
  let brushRadius = $state(20);
  let brushStrength = $state(0.5);
  let sunAngle = $state(45);
  let contourInterval = $state(100);
  let hideUnderwater = $state(false);  // Toggle to hide underwater terrain
  let showRivers = $state(true);  // Toggle to show rivers/lakes
  let riverThreshold = $state(1000);  // Flow accumulation threshold for rivers
  
  // Hydrology simulation
  let numWaterSources = $state(20);
  let waterSourceType = $state<'random' | 'peaks' | 'ridges' | 'uniform'>('peaks');
  let waterSourceCount = $state(0);
  let simulationSteps = $state(100);
  let enableLakeFormation = $state(true);
  let enableStreamCapture = $state(true);
  let isSimulating = $state(false);

  // Generation parameters
  let worldWidth = $state(1536);   // Match canvas width
  let worldHeight = $state(768);   // Match canvas height
  let worldSeed = $state(12345);
  let worldTheme = $state<'Fantasy' | 'Modern' | 'SciFi'>('Fantasy');
  let useErosion = $state(true);
  let erosionIterations = $state(200);
  
  // Noise parameters (exposed for tuning)
  let continentFrequency = $state(0.00005);
  let continentOctaves = $state(3);
  let mountainFrequency = $state(0.0002);
  let mountainOctaves = $state(4);
  let hillFrequency = $state(0.0005);
  let hillOctaves = $state(3);
  let detailFrequency = $state(0.001);
  let detailOctaves = $state(2);
  let landCoverage = $state(0.45);  // Threshold for land vs ocean (0.0 = all ocean, 1.0 = all land)
  
  // Helper to set continent frequency based on target count
  function setContinentCount(count: number) {
    // Approximate frequency mapping for 1536x768 map
    const frequencyMap: Record<number, number> = {
      1: 0.00004,   // One big continent (Pangaea)
      2: 0.00006,   // Two large continents
      3: 0.00008,   // Three continents (realistic)
      4: 0.00010,   // Four continents
      5: 0.00012,   // Many islands (archipelago)
    };
    continentFrequency = frequencyMap[count] || 0.00008;
  }
  
  // Show advanced settings
  let showAdvancedSettings = $state(false);
  
  // Continent painting mode
  let paintMode = $state(false);
  let paintBrushSize = $state(100);
  let paintHeight = $state(0.7);
  let isPainting = $state(false);

  onMount(async () => {
    if (terrainCanvas && previewCanvas) {
      // Initialize renderer
      renderer = new TerrainRenderer(terrainCanvas);
      
      // Initialize viewport
      viewport = new TerrainViewport(
        terrainCanvas,
        handleTransformChange,
        handleVisibleChunksChange
      );

      // Initialize brush preview
      brushPreview = new BrushPreview(previewCanvas);

      // Setup canvas event listeners for brush
      terrainCanvas.addEventListener('mousemove', handleCanvasMouseMove);
      terrainCanvas.addEventListener('mousedown', handleCanvasMouseDown);
      terrainCanvas.addEventListener('mouseup', handleCanvasMouseUp);
      terrainCanvas.addEventListener('mouseleave', handleCanvasMouseLeave);

      // Try to load existing terrain
      try {
        await loadTerrain();
      } catch (e) {
        console.log('No existing terrain to load');
      }
    }
  });

  onDestroy(() => {
    renderer?.destroy();
    viewport?.destroy();
  });

  function handleTransformChange(transform: ViewTransform) {
    console.log('Transform changed:', transform);
    if (renderer && config) {
      renderer.render(transform, sunAngle, contourInterval, hideUnderwater, showRivers, riverThreshold);
    }
  }

  function handleVisibleChunksChange(chunks: VisibleChunks) {
    // TODO: Implement chunk streaming
    console.log('Visible chunks:', chunks.chunks.size);
  }

  async function generateTerrain() {
    isGenerating = true;
    generationProgress = 0;
    generationStage = '🌍 Starting...';
    generationMessage = 'Initializing terrain generation...';

    try {
      // Listen for progress events from Rust backend
      const unlisten = await listen<any>('terrain-progress', (event) => {
        generationProgress = event.payload.progress;
        generationStage = event.payload.stage;
        generationMessage = event.payload.message;
      });

      // Start generation
      const request: GenerateTerrainRequest = {
        width: worldWidth,
        height: worldHeight,
        seed: worldSeed,
        theme: worldTheme,
        use_erosion: useErosion,
        erosion_iterations: erosionIterations,
          noise_params: {
            continent_frequency: continentFrequency,
            continent_octaves: continentOctaves,
            mountain_frequency: mountainFrequency,
            mountain_octaves: mountainOctaves,
            hill_frequency: hillFrequency,
            hill_octaves: hillOctaves,
            detail_frequency: detailFrequency,
            detail_octaves: detailOctaves,
            land_coverage: landCoverage
          }
      };

      const response = await invoke<GenerateTerrainResponse>('generate_terrain', { request });
      unlisten(); // Stop listening to progress events
      
      if (response.success) {
        generationProgress = 1.0;
        generationStage = '✨ Finalizing world...';
        generationMessage = response.message;
        
        // Load config
        config = await invoke<TerrainConfig>('get_terrain_config');
        if (renderer) {
          renderer.setConfig(config);
        }

        // Load heightmap
        await loadFullHeightmap();
        
        generationProgress = 1.0;
        generationStage = '✅ Complete!';
        generationMessage = 'Terrain generated successfully!';
        
        // Clear progress after a moment
        setTimeout(() => {
          if (!isGenerating) {
            generationProgress = 0;
            generationStage = '';
          }
        }, 2000);
      }
    } catch (error) {
      console.error('Failed to generate terrain:', error);
      generationMessage = `Error: ${error}`;
      generationStage = '❌ Generation failed';
    } finally {
      isGenerating = false;
    }
  }

  async function loadFlowData() {
    if (!config || !renderer) return;
    
    try {
      const flowBytes = await invoke<number[]>('get_flow_data');
      const flowData = new Uint8Array(flowBytes);
      
      console.log('Flow data loaded:', flowData.length, 'bytes');
      renderer.updateFlowData(config.world_width, config.world_height, flowData);
    } catch (error) {
      console.log('No flow data available (generate with erosion first)');
    }
  }

  async function loadFullHeightmap() {
    if (!config || !renderer) return;

    const chunkCountX = Math.ceil(config.world_width / config.chunk_size);
    const chunkCountZ = Math.ceil(config.world_height / config.chunk_size);

    // Create full heightmap array
    const fullHeightmap = new Float32Array(config.world_width * config.world_height);

    // Load all chunks
    for (let cz = 0; cz < chunkCountZ; cz++) {
      for (let cx = 0; cx < chunkCountX; cx++) {
        try {
          const chunkBytes = await invoke<number[]>('get_chunk', {
            request: { chunk_x: cx, chunk_z: cz, lod: 0 }
          });

          // Convert byte array to Float32Array
          const byteArray = new Uint8Array(chunkBytes);
          const chunkHeights = new Float32Array(byteArray.buffer);

          // Copy to full heightmap
          const vertexCount = config.vertex_count;
          for (let lz = 0; lz < vertexCount; lz++) {
            for (let lx = 0; lx < vertexCount; lx++) {
              const globalX = cx * config.chunk_size + lx;
              const globalZ = cz * config.chunk_size + lz;
              
              if (globalX < config.world_width && globalZ < config.world_height) {
                const chunkIdx = lz * vertexCount + lx;
                const globalIdx = globalZ * config.world_width + globalX;
                fullHeightmap[globalIdx] = chunkHeights[chunkIdx];
              }
            }
          }
        } catch (error) {
          console.error(`Failed to load chunk (${cx}, ${cz}):`, error);
        }
      }
    }

    // Debug: Check heightmap data
    console.log('Heightmap size:', config.world_width, 'x', config.world_height);
    console.log('Heightmap sample values:', fullHeightmap.slice(0, 10));
    
    // Calculate min/max without spreading (avoid stack overflow)
    let minH = fullHeightmap[0];
    let maxH = fullHeightmap[0];
    for (let i = 0; i < fullHeightmap.length; i++) {
      if (fullHeightmap[i] < minH) minH = fullHeightmap[i];
      if (fullHeightmap[i] > maxH) maxH = fullHeightmap[i];
    }
    console.log('Min height:', minH, 'Max height:', maxH);

    // Upload to GPU
    renderer.updateHeightmap(config.world_width, config.world_height, fullHeightmap);
    
    // Load flow data for rivers/lakes
    await loadFlowData();

    // CRITICAL: Reset viewport to show FULL world
    if (viewport && terrainCanvas) {
      console.log('Canvas dimensions:', terrainCanvas.width, 'x', terrainCanvas.height);
      console.log('World dimensions:', config.world_width, 'x', config.world_height);
      
      // Update viewport with world dimensions for zoom limits
      viewport.setWorldDimensions(config.world_width, config.world_height);
      
      // Reset to identity transform first (no zoom, no pan)
      viewport.resetView();
      
      // Force render with reset view
      renderer.render(viewport.getTransform(), sunAngle, contourInterval);
      
      console.log('Viewport reset to show full world');
    }
  }

  async function saveTerrain() {
    try {
      const result = await invoke<string>('save_terrain');
      alert(result);
    } catch (error) {
      alert(`Failed to save: ${error}`);
    }
  }

  async function loadTerrain() {
    try {
      const result = await invoke<string>('load_terrain');
      config = await invoke<TerrainConfig>('get_terrain_config');
      
      if (renderer) {
        renderer.setConfig(config);
      }

      await loadFullHeightmap();
      alert(result);
    } catch (error) {
      throw error;
    }
  }

  function handleCanvasMouseMove(e: MouseEvent) {
    if (!brushPreview) return;

    const rect = terrainCanvas.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;

    if (activeTool) {
      brushPreview.update(x, y);
    }
  }

  function handleCanvasMouseDown(e: MouseEvent) {
    if (activeTool && e.button === 0) {
      // TODO: Apply brush
      console.log('Apply brush:', activeTool);
    }
  }

  function handleCanvasMouseUp() {
    // TODO: Finish brush stroke
  }

  function handleCanvasMouseLeave() {
    brushPreview?.hide();
  }

  function selectTool(tool: BrushType) {
    activeTool = tool;
    if (brushPreview) {
      brushPreview.show(0, 0, brushRadius);
    }
  }

  function deselectTool() {
    activeTool = null;
    brushPreview?.hide();
  }

  function useRealisticPreset() {
    // Set proven values for realistic continent generation (Azgaar-style)
    // For 1536x768 map
    continentFrequency = 0.00008;  // Creates 2-4 distinct continents
    continentOctaves = 2;
    mountainFrequency = 0.0003;    // Large mountain ranges
    mountainOctaves = 4;
    hillFrequency = 0.001;         // Regional variation
    hillOctaves = 3;
    detailFrequency = 0.003;       // Local features
    detailOctaves = 2;
    landCoverage = 0.45;           // Balanced land/ocean
    useErosion = true;
    erosionIterations = 150;
    
    // Ensure world matches canvas
    worldWidth = 1536;
    worldHeight = 768;
    
    generationMessage = '🌍 Realistic preset loaded! Creates 2-4 distinct continents. Click Generate Terrain.';
  }

  function useArchipelagoPreset() {
    // Many small islands
    continentFrequency = 0.00012;
    continentOctaves = 3;
    mountainFrequency = 0.0005;
    mountainOctaves = 3;
    hillFrequency = 0.002;
    hillOctaves = 2;
    detailFrequency = 0.005;
    detailOctaves = 2;
    landCoverage = 0.35;  // Less land = more islands
    useErosion = true;
    erosionIterations = 100;
    
    worldWidth = 1536;
    worldHeight = 768;
    
    generationMessage = '🏝️ Archipelago preset loaded! Creates scattered island chains.';
  }

  function usePangaeaPreset() {
    // One large supercontinent
    continentFrequency = 0.00004;
    continentOctaves = 2;
    mountainFrequency = 0.0002;
    mountainOctaves = 5;
    hillFrequency = 0.0008;
    hillOctaves = 4;
    detailFrequency = 0.002;
    detailOctaves = 3;
    landCoverage = 0.55;  // More land
    useErosion = true;
    erosionIterations = 200;
    
    worldWidth = 1536;
    worldHeight = 768;
    
    generationMessage = '🌏 Pangaea preset loaded! Creates one massive supercontinent.';
  }

  async function createBlankWorld() {
    try {
      isGenerating = true;
      generationMessage = 'Creating blank canvas...';

      // Generate a flat world at sea level
      const response = await invoke<GenerateTerrainResponse>('generate_terrain', {
        request: {
          width: worldWidth,
          height: worldHeight,
          seed: worldSeed,
          theme: worldTheme,
          use_erosion: false,
          erosion_iterations: 0,
          noise_params: {
            continent_frequency: 0.0,
            continent_octaves: 0,
            mountain_frequency: 0.0,
            mountain_octaves: 0,
            hill_frequency: 0.0,
            hill_octaves: 0,
            detail_frequency: 0.0,
            detail_octaves: 0
          }
        }
      });

      if (response.success) {
        config = await invoke<TerrainConfig>('get_terrain_config');
        if (renderer) {
          renderer.setConfig(config);
        }
        await loadFullHeightmap();
        generationMessage = 'Blank canvas ready! Use brush tools to paint continents.';
      }
    } catch (error) {
      console.error('Failed to create blank world:', error);
      generationMessage = `Error: ${error}`;
    } finally {
      isGenerating = false;
    }
  }

  async function placeWaterSources() {
    if (!config) return;
    
    try {
      generationMessage = 'Placing water sources...';
      
      const response = await invoke<any>('place_water_sources', {
        count: numWaterSources,
        sourceType: waterSourceType
      });
      
      if (response.success) {
        waterSourceCount = response.sources_placed;
        generationMessage = `Placed ${waterSourceCount} water sources at ${waterSourceType} locations`;
      }
    } catch (error) {
      console.error('Failed to place water sources:', error);
      generationMessage = `Error: ${error}`;
    }
  }

  async function runHydrologySimulation() {
    if (!config || waterSourceCount === 0) return;
    
    try {
      isSimulating = true;
      generationMessage = 'Simulating water flow...';
      
      // Listen for progress events
      const unlisten = await listen<any>('terrain-progress', (event) => {
        generationProgress = event.payload.progress;
        generationStage = event.payload.stage;
        generationMessage = event.payload.message;
      });
      
      // Call Rust backend to simulate hydrology
      const response = await invoke<GenerateTerrainResponse>('simulate_hydrology', {
        steps: simulationSteps,
        enableLakes: enableLakeFormation,
        enableCapture: enableStreamCapture
      });
      
      unlisten();
      
      if (response.success) {
        // Reload the heightmap to see changes
        await loadFullHeightmap();
        generationMessage = `Hydrology simulation complete! ${response.message}`;
        
        // Clear progress after a moment
        setTimeout(() => {
          generationProgress = 0;
          generationStage = '';
        }, 2000);
      }
    } catch (error) {
      console.error('Failed to run hydrology simulation:', error);
      generationMessage = `Error: ${error}`;
    } finally {
      isSimulating = false;
    }
  }
</script>

<div class="terrain-map-view">
  <div class="toolbar">
    <div class="toolbar-section">
      <h3>🗺️ World Generation</h3>
      
      <!-- Quick Presets -->
      <div style="display: flex; gap: 0.5rem; flex-wrap: wrap; margin-bottom: 1rem;">
        <button class="btn-secondary" onclick={useRealisticPreset} style="flex: 1;">
          🌍 Realistic
        </button>
        <button class="btn-secondary" onclick={useArchipelagoPreset} style="flex: 1;">
          🏝️ Archipelago
        </button>
        <button class="btn-secondary" onclick={usePangaeaPreset} style="flex: 1;">
          🌏 Pangaea
        </button>
      </div>
      
      <!-- Seed -->
      <div class="form-row">
        <label>Seed:</label>
        <input type="number" bind:value={worldSeed} style="width: 120px;" />
        <button class="btn-icon" onclick={() => worldSeed = Math.floor(Math.random() * 999999999)} title="Randomize Seed">
          🎲
        </button>
      </div>
      
      <!-- Land Coverage -->
      <div class="form-row">
        <label>Land Coverage:</label>
        <input type="range" min="0.2" max="0.7" step="0.05" bind:value={landCoverage} />
        <span>{Math.round(landCoverage * 100)}%</span>
      </div>
      
      <!-- Erosion -->
      <div class="form-row">
        <label>
          <input type="checkbox" bind:checked={useErosion} />
          Erosion
        </label>
      </div>
      {#if useErosion}
        <div class="form-row">
          <label>Intensity:</label>
          <input type="range" min="50" max="500" bind:value={erosionIterations} />
          <span>{erosionIterations}</span>
        </div>
      {/if}
      
      <!-- Noise Settings (Expanded by default) -->
      <h4 style="font-size: 0.875rem; color: #ff6b35; margin: 1rem 0 0.5rem; border-top: 1px solid rgba(255,107,53,0.2); padding-top: 1rem;">Terrain Scale</h4>
      
      <div class="form-row">
        <label title="Target number of continents">Continents:</label>
        <div style="display: flex; gap: 0.25rem; flex: 1;">
          <button class="btn-mini" onclick={() => setContinentCount(1)}>1</button>
          <button class="btn-mini" onclick={() => setContinentCount(2)}>2</button>
          <button class="btn-mini" onclick={() => setContinentCount(3)}>3</button>
          <button class="btn-mini" onclick={() => setContinentCount(4)}>4</button>
          <button class="btn-mini" onclick={() => setContinentCount(5)}>5+</button>
        </div>
      </div>
      
      <div class="form-row">
        <label title="Fine-tune continent size">Size:</label>
        <input type="range" min="0.00003" max="0.00015" step="0.00001" bind:value={continentFrequency} />
        <span style="font-size: 0.75rem;">{(continentFrequency * 100000).toFixed(1)}</span>
      </div>
      
      <div class="form-row">
        <label title="Controls mountain range size">Mountain Size:</label>
        <input type="range" min="0.0001" max="0.001" step="0.0001" bind:value={mountainFrequency} />
        <span style="font-size: 0.75rem;">{(mountainFrequency * 10000).toFixed(1)}</span>
      </div>
      
      <div class="form-row">
        <label title="Mountain detail/roughness">Roughness:</label>
        <input type="range" min="1" max="6" bind:value={mountainOctaves} />
        <span>{mountainOctaves}</span>
      </div>
      
      <div class="form-row">
        <label title="Overall terrain detail">Detail:</label>
        <input type="range" min="1" max="5" bind:value={continentOctaves} />
        <span>{continentOctaves}</span>
      </div>
      
      <!-- Advanced Settings (Collapsed) -->
      <button class="btn-secondary" onclick={() => showAdvancedSettings = !showAdvancedSettings} style="margin-top: 0.5rem;">
        {showAdvancedSettings ? '▼' : '▶'} Advanced Noise
      </button>
      
      {#if showAdvancedSettings}
        <div style="margin-top: 0.5rem; padding: 0.75rem; background: rgba(0,0,0,0.2); border-radius: 4px;">
          <p style="font-size: 0.75rem; color: #8b95a5; margin-bottom: 0.5rem;">
            Fine-tune individual noise layers
          </p>
          
          <div class="form-row">
            <label>Continent Octaves:</label>
            <input type="range" min="1" max="5" bind:value={continentOctaves} />
            <span>{continentOctaves}</span>
          </div>

          <div class="form-row">
            <label>Mountain Octaves:</label>
            <input type="range" min="1" max="8" bind:value={mountainOctaves} />
            <span>{mountainOctaves}</span>
          </div>

          <div class="form-row">
            <label>Hill Frequency:</label>
            <input type="range" min="0.0005" max="0.002" step="0.0001" bind:value={hillFrequency} />
            <span style="font-size: 0.75rem;">{(hillFrequency * 10000).toFixed(1)}</span>
          </div>
          
          <div class="form-row">
            <label>Hill Octaves:</label>
            <input type="range" min="1" max="5" bind:value={hillOctaves} />
            <span>{hillOctaves}</span>
          </div>

          <div class="form-row">
            <label>Detail Frequency:</label>
            <input type="range" min="0.001" max="0.005" step="0.0005" bind:value={detailFrequency} />
            <span style="font-size: 0.75rem;">{(detailFrequency * 10000).toFixed(1)}</span>
          </div>
          
          <div class="form-row">
            <label>Detail Octaves:</label>
            <input type="range" min="1" max="4" bind:value={detailOctaves} />
            <span>{detailOctaves}</span>
          </div>
        </div>
      {/if}
      
      <!-- Map Size (Collapsed) -->
      <h4 style="font-size: 0.875rem; color: #8b95a5; margin: 1rem 0 0.5rem; border-top: 1px solid rgba(139,149,165,0.2); padding-top: 1rem;">Map Dimensions</h4>
      <div class="form-row">
        <label>Width:</label>
        <input type="number" bind:value={worldWidth} step="128" min="512" max="4096" style="width: 80px;" />
      </div>
      <div class="form-row">
        <label>Height:</label>
        <input type="number" bind:value={worldHeight} step="128" min="512" max="4096" style="width: 80px;" />
      </div>
      <button class="btn-secondary" onclick={() => { worldWidth = 1536; worldHeight = 768; }} style="font-size: 0.75rem; padding: 0.25rem 0.5rem;">
        Reset (1536×768)
      </button>
      
      <!-- Generate Button -->
      <button class="btn-primary" onclick={generateTerrain} disabled={isGenerating} style="margin-top: 1rem; width: 100%;">
        {isGenerating ? 'Generating...' : '🗺️ Generate World'}
      </button>
      
      {#if isGenerating || generationProgress > 0}
        <div class="progress-container">
          <div class="progress-bar">
            <div class="progress-fill" style="width: {generationProgress * 100}%"></div>
          </div>
          <div class="progress-stage">{generationStage}</div>
          <div class="progress-percent">{Math.round(generationProgress * 100)}%</div>
        </div>
      {/if}
      
      {#if generationMessage && !isGenerating}
        <p style="margin-top: 0.5rem; font-size: 0.875rem; color: #10b981;">{generationMessage}</p>
      {/if}
    </div>

    <div class="toolbar-section">
      <h3>🎨 View Settings</h3>
      <div class="form-row">
        <label>
          <input type="checkbox" bind:checked={hideUnderwater} onchange={() => renderer && viewport && renderer.render(viewport.getTransform(), sunAngle, contourInterval, hideUnderwater, showRivers, riverThreshold)} />
          Hide Underwater
        </label>
      </div>
      <div class="form-row">
        <label>
          <input type="checkbox" bind:checked={showRivers} onchange={() => renderer && viewport && renderer.render(viewport.getTransform(), sunAngle, contourInterval, hideUnderwater, showRivers, riverThreshold)} />
          Show Rivers/Lakes
        </label>
      </div>
      {#if showRivers}
        <div class="form-row">
          <label>River Threshold:</label>
          <input type="range" min="100" max="5000" step="100" bind:value={riverThreshold} oninput={() => renderer && viewport && renderer.render(viewport.getTransform(), sunAngle, contourInterval, hideUnderwater, showRivers, riverThreshold)} />
          <span>{riverThreshold}</span>
        </div>
      {/if}
      <div class="form-row">
        <label>Sun Angle:</label>
        <input type="range" min="0" max="90" bind:value={sunAngle} oninput={() => renderer && viewport && renderer.render(viewport.getTransform(), sunAngle, contourInterval, hideUnderwater, showRivers, riverThreshold)} />
        <span>{sunAngle}°</span>
      </div>
      <div class="form-row">
        <label>Contours:</label>
        <input type="range" min="50" max="500" step="50" bind:value={contourInterval} oninput={() => renderer && viewport && renderer.render(viewport.getTransform(), sunAngle, contourInterval, hideUnderwater, showRivers, riverThreshold)} />
        <span>{contourInterval}m</span>
      </div>
      <button class="btn-secondary" onclick={() => viewport?.resetView()}>🔄 Reset View</button>
    </div>

    <div class="toolbar-section">
      <h3>⏱️ Hydrology Simulation</h3>
      <p style="font-size: 0.75rem; color: #8b95a5; margin-bottom: 0.5rem;">
        Add water sources and simulate flow
      </p>
      
      <!-- Water Source Controls -->
      <div class="form-row">
        <label>Water Sources:</label>
        <input type="range" min="5" max="100" step="5" bind:value={numWaterSources} />
        <span>{numWaterSources}</span>
      </div>
      
      <div class="form-row">
        <label>Source Type:</label>
        <select bind:value={waterSourceType}>
          <option value="random">Random (Springs)</option>
          <option value="peaks">Mountain Peaks</option>
          <option value="ridges">Ridge Lines</option>
          <option value="uniform">Uniform Grid</option>
        </select>
      </div>
      
      <button class="btn-secondary" onclick={placeWaterSources} disabled={!config}>
        💧 Place Water Sources
      </button>
      
      <!-- Simulation Controls -->
      <h4 style="font-size: 0.875rem; color: #ff6b35; margin: 1rem 0 0.5rem; border-top: 1px solid rgba(255,107,53,0.2); padding-top: 1rem;">Run Simulation</h4>
      
      <div class="form-row">
        <label>Time Steps:</label>
        <input type="range" min="10" max="500" step="10" bind:value={simulationSteps} />
        <span>{simulationSteps}</span>
      </div>
      
      <div class="form-row">
        <label>
          <input type="checkbox" bind:checked={enableLakeFormation} />
          Lake Formation
        </label>
      </div>
      
      <div class="form-row">
        <label>
          <input type="checkbox" bind:checked={enableStreamCapture} />
          Stream Capture
        </label>
      </div>
      
      <button class="btn-primary" onclick={runHydrologySimulation} disabled={isSimulating || !config || waterSourceCount === 0} style="width: 100%; margin-top: 0.5rem;">
        {isSimulating ? '⏳ Simulating...' : '🌊 Simulate Hydrology'}
      </button>
      
      {#if waterSourceCount > 0}
        <p style="font-size: 0.75rem; color: #10b981; margin-top: 0.5rem;">
          ✓ {waterSourceCount} water sources placed
        </p>
      {/if}
      
      <p style="font-size: 0.75rem; color: #8b95a5; margin-top: 0.5rem;">
        Water flows from sources, carves valleys, forms rivers and lakes
      </p>
    </div>

    <div class="toolbar-section">
      <h3>💾 File</h3>
      <button class="btn-secondary" onclick={saveTerrain}>💾 Save World</button>
      <button class="btn-secondary" onclick={loadTerrain}>📂 Load World</button>
    </div>
  </div>

  <div class="canvas-container">
    <canvas bind:this={terrainCanvas} width={1536} height={768}></canvas>
    <canvas bind:this={previewCanvas} width={1536} height={768} class="preview-canvas"></canvas>
    
    <!-- Zoom controls overlay -->
    <div class="zoom-controls">
      <div class="zoom-hint">🖱️ Scroll to zoom • Drag to pan</div>
      {#if viewport}
        <button class="zoom-btn" onclick={() => viewport?.resetView()} title="Reset View">
          🔄
        </button>
      {/if}
    </div>
  </div>
</div>

<style>
  .terrain-map-view {
    display: flex;
    height: 100%;
    background: hsl(220, 14%, 10%);
    color: rgb(243, 244, 246);
  }

  .toolbar {
    width: 300px;
    background: hsl(220, 14%, 14%);
    border-right: 1px solid hsl(220, 10%, 22%);
    padding: 1rem;
    overflow-y: auto;
  }

  .toolbar-section {
    margin-bottom: 2rem;
    padding-bottom: 1rem;
    border-bottom: 1px solid hsl(220, 10%, 22%);
  }

  .toolbar-section:last-child {
    border-bottom: none;
  }

  .toolbar-section h3 {
    margin: 0 0 1rem 0;
    color: #ff6b35;
    font-size: 1rem;
  }

  .form-row {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-bottom: 0.75rem;
  }

  .form-row label {
    flex: 0 0 auto;
    font-size: 0.875rem;
    color: rgb(156, 163, 175);
  }

  .form-row input[type="number"],
  .form-row select {
    flex: 1;
    padding: 0.4rem;
    background: rgba(15, 20, 30, 0.6);
    border: 1px solid rgba(139, 149, 165, 0.3);
    border-radius: 4px;
    color: #e0e6ed;
    font-size: 0.875rem;
  }

  .form-row input[type="range"] {
    flex: 1;
  }

  .form-row span {
    flex: 0 0 auto;
    min-width: 3rem;
    text-align: right;
    font-size: 0.875rem;
  }

  .tool-buttons {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 0.5rem;
    margin-bottom: 1rem;
  }

  .tool-buttons button {
    padding: 0.5rem;
    background: rgba(139, 149, 165, 0.2);
    border: 1px solid rgba(139, 149, 165, 0.3);
    border-radius: 4px;
    color: #e0e6ed;
    font-size: 0.875rem;
    cursor: pointer;
    transition: all 0.2s;
  }

  .tool-buttons button:hover {
    background: rgba(139, 149, 165, 0.3);
  }

  .tool-buttons button.active {
    background: #ff6b35;
    border-color: #ff8555;
    color: white;
  }

  .btn-primary, .btn-secondary {
    width: 100%;
    padding: 0.6rem 1rem;
    border: none;
    border-radius: 6px;
    font-size: 0.9rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
    margin-bottom: 0.5rem;
  }

  .btn-primary {
    background: #ff6b35;
    color: white;
  }

  .btn-primary:hover:not(:disabled) {
    background: #ff8555;
    transform: translateY(-1px);
  }

  .btn-primary:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-secondary {
    background: rgba(139, 149, 165, 0.2);
    color: #e0e6ed;
    border: 1px solid rgba(139, 149, 165, 0.3);
  }

  .btn-secondary:hover {
    background: rgba(139, 149, 165, 0.3);
  }

  .btn-icon {
    padding: 0.25rem 0.5rem;
    background: rgba(139, 149, 165, 0.2);
    color: #e0e6ed;
    border: 1px solid rgba(139, 149, 165, 0.3);
    border-radius: 4px;
    cursor: pointer;
    font-size: 1rem;
    margin-left: 0.5rem;
  }

  .btn-icon:hover {
    background: rgba(139, 149, 165, 0.3);
  }

  .btn-mini {
    padding: 0.25rem 0.5rem;
    background: rgba(139, 149, 165, 0.2);
    color: #e0e6ed;
    border: 1px solid rgba(139, 149, 165, 0.3);
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.75rem;
    flex: 1;
    transition: all 0.2s;
  }

  .btn-mini:hover {
    background: rgba(255, 107, 53, 0.3);
    border-color: rgba(255, 107, 53, 0.5);
  }

  .message {
    margin-top: 0.5rem;
    font-size: 0.875rem;
    color: rgb(156, 163, 175);
  }

  .progress-container {
    margin-top: 1rem;
    padding: 1rem;
    background: rgba(255, 107, 53, 0.05);
    border: 1px solid rgba(255, 107, 53, 0.2);
    border-radius: 8px;
  }

  .progress-bar {
    width: 100%;
    height: 8px;
    background: rgba(255, 255, 255, 0.1);
    border-radius: 4px;
    overflow: hidden;
    margin-bottom: 0.75rem;
  }

  .progress-fill {
    height: 100%;
    background: linear-gradient(90deg, #ff6b35, #ffa500, #10b981);
    border-radius: 4px;
    transition: width 0.3s ease;
    animation: shimmer 2s infinite;
  }

  @keyframes shimmer {
    0% { opacity: 0.8; }
    50% { opacity: 1; }
    100% { opacity: 0.8; }
  }

  .progress-stage {
    font-size: 0.875rem;
    color: #ff6b35;
    font-weight: 500;
    margin-bottom: 0.25rem;
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .progress-percent {
    font-size: 0.75rem;
    color: rgb(156, 163, 175);
    text-align: right;
  }

  .canvas-container {
    flex: 1;
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
    background: hsl(220, 14%, 8%);
  }

  canvas {
    display: block;
    border-radius: 4px;
    cursor: grab;
  }

  canvas:active {
    cursor: grabbing;
  }

  .preview-canvas {
    position: absolute;
    pointer-events: none;
  }

  .zoom-controls {
    position: absolute;
    bottom: 1rem;
    left: 50%;
    transform: translateX(-50%);
    display: flex;
    align-items: center;
    gap: 1rem;
    background: rgba(0, 0, 0, 0.7);
    padding: 0.5rem 1rem;
    border-radius: 8px;
    backdrop-filter: blur(4px);
  }

  .zoom-hint {
    font-size: 0.875rem;
    color: #e0e6ed;
  }

  .zoom-btn {
    padding: 0.5rem;
    background: rgba(139, 149, 165, 0.2);
    color: #e0e6ed;
    border: 1px solid rgba(139, 149, 165, 0.3);
    border-radius: 4px;
    cursor: pointer;
    font-size: 1rem;
    transition: all 0.2s;
  }

  .zoom-btn:hover {
    background: rgba(139, 149, 165, 0.4);
    transform: scale(1.1);
  }
</style>
