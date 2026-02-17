<script lang="ts">
  import { onMount } from 'svelte';
  import { getCurrentRoom, type RoomDetails, type Exit } from '$lib/utils/tauri';
  
  interface RoomNode {
    id: string;
    name: string;
    description: string;
    x: number;
    y: number;
    exits: Exit[];
  }
  
  let rooms = $state<RoomNode[]>([]);
  let selectedRoom = $state<RoomNode | null>(null);
  let isCreatingRoom = $state(false);
  let newRoomName = $state('');
  let newRoomDescription = $state('');
  
  // Canvas for visual map
  let canvas: HTMLCanvasElement;
  let ctx: CanvasRenderingContext2D | null;
  
  onMount(async () => {
    // Load initial room data
    await loadRooms();
    
    // Initialize canvas
    if (canvas) {
      ctx = canvas.getContext('2d');
      drawMap();
    }
  });
  
  async function loadRooms() {
    try {
      // TODO: Add backend command to get all rooms
      // For now, load current room as starting point
      const currentRoom = await getCurrentRoom();
      
      rooms = [{
        id: currentRoom.id,
        name: currentRoom.name,
        description: currentRoom.description,
        x: 400,
        y: 300,
        exits: currentRoom.exits
      }];
      
      drawMap();
    } catch (error) {
      console.error('Failed to load rooms:', error);
    }
  }
  
  function drawMap() {
    if (!ctx || !canvas) return;
    
    // Clear canvas
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    
    // Draw grid
    ctx.strokeStyle = 'rgba(139, 149, 165, 0.1)';
    ctx.lineWidth = 1;
    for (let x = 0; x < canvas.width; x += 50) {
      ctx.beginPath();
      ctx.moveTo(x, 0);
      ctx.lineTo(x, canvas.height);
      ctx.stroke();
    }
    for (let y = 0; y < canvas.height; y += 50) {
      ctx.beginPath();
      ctx.moveTo(0, y);
      ctx.lineTo(canvas.width, y);
      ctx.stroke();
    }
    
    // Draw connections (exits)
    ctx.strokeStyle = '#ff6b35';
    ctx.lineWidth = 2;
    rooms.forEach(room => {
      room.exits.forEach(exit => {
        const targetRoom = rooms.find(r => r.id === exit.target_room_id);
        if (targetRoom) {
          ctx!.beginPath();
          ctx!.moveTo(room.x, room.y);
          ctx!.lineTo(targetRoom.x, targetRoom.y);
          ctx!.stroke();
          
          // Draw arrow
          const angle = Math.atan2(targetRoom.y - room.y, targetRoom.x - room.x);
          const arrowSize = 10;
          ctx!.beginPath();
          ctx!.moveTo(targetRoom.x, targetRoom.y);
          ctx!.lineTo(
            targetRoom.x - arrowSize * Math.cos(angle - Math.PI / 6),
            targetRoom.y - arrowSize * Math.sin(angle - Math.PI / 6)
          );
          ctx!.moveTo(targetRoom.x, targetRoom.y);
          ctx!.lineTo(
            targetRoom.x - arrowSize * Math.cos(angle + Math.PI / 6),
            targetRoom.y - arrowSize * Math.sin(angle + Math.PI / 6)
          );
          ctx!.stroke();
        }
      });
    });
    
    // Draw rooms
    rooms.forEach(room => {
      const isSelected = selectedRoom?.id === room.id;
      
      // Room circle
      ctx!.fillStyle = isSelected ? '#ff6b35' : 'rgba(139, 149, 165, 0.3)';
      ctx!.beginPath();
      ctx!.arc(room.x, room.y, 30, 0, Math.PI * 2);
      ctx!.fill();
      
      // Room border
      ctx!.strokeStyle = isSelected ? '#ffa07a' : '#8b95a5';
      ctx!.lineWidth = 2;
      ctx!.stroke();
      
      // Room name
      ctx!.fillStyle = '#e0e6ed';
      ctx!.font = '12px "Segoe UI"';
      ctx!.textAlign = 'center';
      ctx!.fillText(room.name, room.x, room.y + 50);
    });
  }
  
  function handleCanvasClick(event: MouseEvent) {
    const rect = canvas.getBoundingClientRect();
    const x = event.clientX - rect.left;
    const y = event.clientY - rect.top;
    
    // Check if clicked on a room
    const clickedRoom = rooms.find(room => {
      const distance = Math.sqrt((room.x - x) ** 2 + (room.y - y) ** 2);
      return distance <= 30;
    });
    
    if (clickedRoom) {
      selectedRoom = clickedRoom;
      drawMap();
    } else {
      selectedRoom = null;
      drawMap();
    }
  }
  
  function createRoom() {
    if (!newRoomName.trim()) return;
    
    const newRoom: RoomNode = {
      id: crypto.randomUUID(),
      name: newRoomName,
      description: newRoomDescription,
      x: 400 + Math.random() * 200 - 100,
      y: 300 + Math.random() * 200 - 100,
      exits: []
    };
    
    rooms.push(newRoom);
    selectedRoom = newRoom;
    
    newRoomName = '';
    newRoomDescription = '';
    isCreatingRoom = false;
    
    drawMap();
  }
  
  function deleteRoom() {
    if (!selectedRoom) return;
    
    rooms = rooms.filter(r => r.id !== selectedRoom!.id);
    selectedRoom = null;
    drawMap();
  }
</script>

<div class="world-map-editor">
  <div class="editor-toolbar">
    <button class="btn-primary" onclick={() => isCreatingRoom = true}>
      ➕ Create Room
    </button>
    <button class="btn-secondary" onclick={loadRooms}>
      🔄 Refresh
    </button>
    {#if selectedRoom}
      <button class="btn-danger" onclick={deleteRoom}>
        🗑️ Delete Room
      </button>
    {/if}
  </div>

  <div class="editor-main">
    <div class="map-canvas-container">
      <canvas
        bind:this={canvas}
        width={800}
        height={600}
        onclick={handleCanvasClick}
      ></canvas>
    </div>

    <aside class="room-inspector">
      {#if selectedRoom}
        <h3>📍 {selectedRoom.name}</h3>
        <div class="inspector-section">
          <label>Name:</label>
          <input type="text" bind:value={selectedRoom.name} />
        </div>
        <div class="inspector-section">
          <label>Description:</label>
          <textarea bind:value={selectedRoom.description} rows="4"></textarea>
        </div>
        <div class="inspector-section">
          <label>Exits ({selectedRoom.exits.length}):</label>
          <ul class="exit-list">
            {#each selectedRoom.exits as exit}
              <li>{exit.direction} → {exit.target_room_id}</li>
            {/each}
          </ul>
          <button class="btn-small">➕ Add Exit</button>
        </div>
        <button class="btn-primary full-width">💾 Save Changes</button>
      {:else}
        <div class="inspector-empty">
          <p>Select a room to edit</p>
          <p class="hint">Click on a room node in the map</p>
        </div>
      {/if}
    </aside>
  </div>
</div>

{#if isCreatingRoom}
  <div class="modal-overlay" onclick={() => isCreatingRoom = false}>
    <div class="modal" onclick={(e) => e.stopPropagation()}>
      <h2>Create New Room</h2>
      <div class="form-group">
        <label>Room Name:</label>
        <input type="text" bind:value={newRoomName} placeholder="The Mystic Library" />
      </div>
      <div class="form-group">
        <label>Description:</label>
        <textarea bind:value={newRoomDescription} rows="4" placeholder="A vast library filled with ancient tomes..."></textarea>
      </div>
      <div class="modal-actions">
        <button class="btn-secondary" onclick={() => isCreatingRoom = false}>Cancel</button>
        <button class="btn-primary" onclick={createRoom}>Create</button>
      </div>
    </div>
  </div>
{/if}

<style>
  .world-map-editor {
    display: flex;
    flex-direction: column;
    height: 100%;
    gap: 1rem;
  }

  .editor-toolbar {
    display: flex;
    gap: 0.75rem;
    padding: 1rem;
    background: rgba(20, 25, 35, 0.5);
    border-radius: 8px;
    border: 1px solid rgba(139, 149, 165, 0.2);
  }

  .editor-main {
    display: grid;
    grid-template-columns: 1fr 320px;
    gap: 1rem;
    flex: 1;
    overflow: hidden;
  }

  .map-canvas-container {
    background: rgba(15, 20, 30, 0.8);
    border-radius: 8px;
    border: 1px solid rgba(139, 149, 165, 0.2);
    padding: 1rem;
    overflow: auto;
  }

  canvas {
    display: block;
    border-radius: 4px;
    cursor: pointer;
  }

  .room-inspector {
    background: rgba(20, 25, 35, 0.8);
    border-radius: 8px;
    border: 1px solid rgba(139, 149, 165, 0.2);
    padding: 1.5rem;
    overflow-y: auto;
  }

  .room-inspector h3 {
    margin: 0 0 1.5rem 0;
    color: #ff6b35;
    font-size: 1.2rem;
  }

  .inspector-section {
    margin-bottom: 1.5rem;
  }

  .inspector-section label {
    display: block;
    margin-bottom: 0.5rem;
    color: #8b95a5;
    font-size: 0.9rem;
    font-weight: 500;
  }

  .inspector-section input,
  .inspector-section textarea {
    width: 100%;
    padding: 0.6rem;
    background: rgba(15, 20, 30, 0.6);
    border: 1px solid rgba(139, 149, 165, 0.3);
    border-radius: 4px;
    color: #e0e6ed;
    font-family: inherit;
    font-size: 0.9rem;
  }

  .inspector-section textarea {
    resize: vertical;
  }

  .exit-list {
    list-style: none;
    padding: 0;
    margin: 0 0 0.75rem 0;
  }

  .exit-list li {
    padding: 0.5rem;
    background: rgba(15, 20, 30, 0.4);
    border-radius: 4px;
    margin-bottom: 0.5rem;
    font-size: 0.85rem;
  }

  .inspector-empty {
    text-align: center;
    padding: 3rem 1rem;
    color: #8b95a5;
  }

  .inspector-empty .hint {
    font-size: 0.85rem;
    margin-top: 0.5rem;
  }

  .btn-primary, .btn-secondary, .btn-danger, .btn-small {
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
    transform: translateY(-1px);
  }

  .btn-secondary {
    background: rgba(139, 149, 165, 0.2);
    color: #e0e6ed;
    border: 1px solid rgba(139, 149, 165, 0.3);
  }

  .btn-secondary:hover {
    background: rgba(139, 149, 165, 0.3);
  }

  .btn-danger {
    background: rgba(220, 53, 69, 0.2);
    color: #ff6b6b;
    border: 1px solid rgba(220, 53, 69, 0.3);
  }

  .btn-danger:hover {
    background: rgba(220, 53, 69, 0.3);
  }

  .btn-small {
    padding: 0.4rem 0.8rem;
    font-size: 0.85rem;
  }

  .full-width {
    width: 100%;
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
    max-width: 500px;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.5);
  }

  .modal h2 {
    margin: 0 0 1.5rem 0;
    color: #ff6b35;
  }

  .form-group {
    margin-bottom: 1.5rem;
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
    padding: 0.75rem;
    background: rgba(15, 20, 30, 0.6);
    border: 1px solid rgba(139, 149, 165, 0.3);
    border-radius: 6px;
    color: #e0e6ed;
    font-family: inherit;
    font-size: 1rem;
  }

  .modal-actions {
    display: flex;
    gap: 1rem;
    justify-content: flex-end;
    margin-top: 2rem;
  }
</style>
