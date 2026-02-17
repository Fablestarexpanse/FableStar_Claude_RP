<script lang="ts">
  import { onMount } from 'svelte';
  
  interface ConsoleEntry {
    id: string;
    type: 'command' | 'output' | 'error';
    content: string;
    timestamp: Date;
  }
  
  let consoleHistory = $state<ConsoleEntry[]>([]);
  let commandInput = $state('');
  let commandHistory = $state<string[]>([]);
  let historyIndex = $state(-1);
  
  // Available commands
  const commands = [
    { name: 'help', description: 'Show available commands' },
    { name: 'spawn_npc <name>', description: 'Spawn an NPC in current room' },
    { name: 'create_room <name>', description: 'Create a new room' },
    { name: 'teleport <room_name>', description: 'Teleport player to room' },
    { name: 'give_item <item_name>', description: 'Give item to player' },
    { name: 'set_stat <stat> <value>', description: 'Set player stat' },
    { name: 'advance_time <hours>', description: 'Advance world time' },
    { name: 'trigger_event <event_type>', description: 'Trigger a world event' },
    { name: 'clear', description: 'Clear console' },
    { name: 'save', description: 'Force save world state' },
    { name: 'load', description: 'Reload world from database' }
  ];
  
  onMount(() => {
    addOutput('WorldWeaver GM Console v0.1.0', 'output');
    addOutput('Type "help" for available commands', 'output');
  });
  
  function addEntry(content: string, type: ConsoleEntry['type']) {
    consoleHistory.push({
      id: crypto.randomUUID(),
      type,
      content,
      timestamp: new Date()
    });
    
    // Auto-scroll to bottom
    setTimeout(() => {
      const console = document.querySelector('.console-output');
      if (console) {
        console.scrollTop = console.scrollHeight;
      }
    }, 0);
  }
  
  function addCommand(content: string) {
    addEntry(`> ${content}`, 'command');
  }
  
  function addOutput(content: string, type: 'output' | 'error' = 'output') {
    addEntry(content, type);
  }
  
  async function executeCommand(cmd: string) {
    if (!cmd.trim()) return;
    
    addCommand(cmd);
    commandHistory.unshift(cmd);
    historyIndex = -1;
    
    const parts = cmd.trim().split(' ');
    const command = parts[0].toLowerCase();
    const args = parts.slice(1);
    
    try {
      switch (command) {
        case 'help':
          addOutput('Available commands:', 'output');
          commands.forEach(c => {
            addOutput(`  ${c.name.padEnd(30)} - ${c.description}`, 'output');
          });
          break;
          
        case 'spawn_npc':
          if (args.length === 0) {
            addOutput('Error: Missing NPC name', 'error');
          } else {
            const npcName = args.join(' ');
            // TODO: Add backend command
            addOutput(`Spawned NPC: ${npcName}`, 'output');
          }
          break;
          
        case 'create_room':
          if (args.length === 0) {
            addOutput('Error: Missing room name', 'error');
          } else {
            const roomName = args.join(' ');
            // TODO: Add backend command
            addOutput(`Created room: ${roomName}`, 'output');
          }
          break;
          
        case 'teleport':
          if (args.length === 0) {
            addOutput('Error: Missing room name', 'error');
          } else {
            const roomName = args.join(' ');
            // TODO: Add backend command
            addOutput(`Teleported player to: ${roomName}`, 'output');
          }
          break;
          
        case 'give_item':
          if (args.length === 0) {
            addOutput('Error: Missing item name', 'error');
          } else {
            const itemName = args.join(' ');
            // TODO: Add backend command
            addOutput(`Gave item: ${itemName}`, 'output');
          }
          break;
          
        case 'set_stat':
          if (args.length < 2) {
            addOutput('Error: Usage: set_stat <stat> <value>', 'error');
          } else {
            const [stat, value] = args;
            // TODO: Add backend command
            addOutput(`Set ${stat} to ${value}`, 'output');
          }
          break;
          
        case 'advance_time':
          if (args.length === 0) {
            addOutput('Error: Missing hours', 'error');
          } else {
            const hours = parseInt(args[0]);
            if (isNaN(hours)) {
              addOutput('Error: Hours must be a number', 'error');
            } else {
              // TODO: Add backend command
              addOutput(`Advanced time by ${hours} hours`, 'output');
            }
          }
          break;
          
        case 'trigger_event':
          if (args.length === 0) {
            addOutput('Error: Missing event type', 'error');
          } else {
            const eventType = args.join(' ');
            // TODO: Add backend command
            addOutput(`Triggered event: ${eventType}`, 'output');
          }
          break;
          
        case 'clear':
          consoleHistory = [];
          addOutput('Console cleared', 'output');
          break;
          
        case 'save':
          // TODO: Add backend command
          addOutput('World state saved', 'output');
          break;
          
        case 'load':
          // TODO: Add backend command
          addOutput('World state loaded from database', 'output');
          break;
          
        default:
          addOutput(`Unknown command: ${command}. Type "help" for available commands.`, 'error');
      }
    } catch (error) {
      addOutput(`Error executing command: ${error}`, 'error');
    }
    
    commandInput = '';
  }
  
  function handleKeyDown(event: KeyboardEvent) {
    if (event.key === 'Enter') {
      executeCommand(commandInput);
    } else if (event.key === 'ArrowUp') {
      event.preventDefault();
      if (historyIndex < commandHistory.length - 1) {
        historyIndex++;
        commandInput = commandHistory[historyIndex];
      }
    } else if (event.key === 'ArrowDown') {
      event.preventDefault();
      if (historyIndex > 0) {
        historyIndex--;
        commandInput = commandHistory[historyIndex];
      } else if (historyIndex === 0) {
        historyIndex = -1;
        commandInput = '';
      }
    }
  }
</script>

<div class="testing-console">
  <div class="console-header">
    <h3>⚙️ Testing Console</h3>
    <div class="console-actions">
      <button class="btn-small" onclick={() => consoleHistory = []}>
        🗑️ Clear
      </button>
      <button class="btn-small" onclick={() => addOutput('Commands: ' + commands.map(c => c.name).join(', '), 'output')}>
        📋 Commands
      </button>
    </div>
  </div>

  <div class="console-main">
    <div class="console-output">
      {#each consoleHistory as entry}
        <div class="console-entry {entry.type}">
          <span class="entry-timestamp">
            [{entry.timestamp.toLocaleTimeString()}]
          </span>
          <span class="entry-content">{entry.content}</span>
        </div>
      {/each}
    </div>

    <div class="console-input-area">
      <span class="input-prompt">></span>
      <input
        type="text"
        bind:value={commandInput}
        onkeydown={handleKeyDown}
        placeholder="Enter command..."
        class="console-input"
        autofocus
      />
    </div>
  </div>

  <div class="console-sidebar">
    <h4>Quick Actions</h4>
    
    <div class="quick-actions">
      <button class="action-btn" onclick={() => commandInput = 'spawn_npc '}>
        👤 Spawn NPC
      </button>
      <button class="action-btn" onclick={() => commandInput = 'create_room '}>
        🏠 Create Room
      </button>
      <button class="action-btn" onclick={() => commandInput = 'teleport '}>
        🌀 Teleport
      </button>
      <button class="action-btn" onclick={() => commandInput = 'give_item '}>
        🎁 Give Item
      </button>
      <button class="action-btn" onclick={() => commandInput = 'advance_time '}>
        ⏰ Advance Time
      </button>
      <button class="action-btn" onclick={() => executeCommand('save')}>
        💾 Save World
      </button>
    </div>

    <h4>Command Reference</h4>
    <div class="command-reference">
      {#each commands.slice(0, 6) as cmd}
        <div class="ref-item">
          <div class="ref-command">{cmd.name}</div>
          <div class="ref-description">{cmd.description}</div>
        </div>
      {/each}
    </div>
  </div>
</div>

<style>
  .testing-console {
    display: grid;
    grid-template-columns: 1fr 300px;
    grid-template-rows: auto 1fr;
    gap: 1rem;
    height: 100%;
  }

  .console-header {
    grid-column: 1 / -1;
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem 1.5rem;
    background: rgba(20, 25, 35, 0.8);
    border-radius: 8px;
    border: 1px solid rgba(139, 149, 165, 0.2);
  }

  .console-header h3 {
    margin: 0;
    color: #ff6b35;
    font-size: 1.1rem;
  }

  .console-actions {
    display: flex;
    gap: 0.75rem;
  }

  .console-main {
    display: flex;
    flex-direction: column;
    background: rgba(15, 20, 30, 0.9);
    border-radius: 8px;
    border: 1px solid rgba(139, 149, 165, 0.2);
    overflow: hidden;
  }

  .console-output {
    flex: 1;
    overflow-y: auto;
    padding: 1rem;
    font-family: 'Courier New', monospace;
    font-size: 0.9rem;
    line-height: 1.6;
  }

  .console-entry {
    margin-bottom: 0.5rem;
    display: flex;
    gap: 0.75rem;
  }

  .console-entry.command {
    color: #4caf50;
  }

  .console-entry.output {
    color: #e0e6ed;
  }

  .console-entry.error {
    color: #ff6b6b;
  }

  .entry-timestamp {
    color: #8b95a5;
    font-size: 0.8rem;
    flex-shrink: 0;
  }

  .entry-content {
    flex: 1;
    word-break: break-word;
  }

  .console-input-area {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 1rem;
    background: rgba(20, 25, 35, 0.8);
    border-top: 1px solid rgba(139, 149, 165, 0.2);
  }

  .input-prompt {
    color: #4caf50;
    font-family: 'Courier New', monospace;
    font-size: 1.1rem;
    font-weight: bold;
  }

  .console-input {
    flex: 1;
    padding: 0.6rem;
    background: transparent;
    border: none;
    color: #e0e6ed;
    font-family: 'Courier New', monospace;
    font-size: 0.95rem;
    outline: none;
  }

  .console-input::placeholder {
    color: #8b95a5;
  }

  .console-sidebar {
    background: rgba(20, 25, 35, 0.8);
    border-radius: 8px;
    border: 1px solid rgba(139, 149, 165, 0.2);
    padding: 1.5rem;
    overflow-y: auto;
  }

  .console-sidebar h4 {
    margin: 0 0 1rem 0;
    color: #ff6b35;
    font-size: 0.95rem;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .quick-actions {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    margin-bottom: 2rem;
  }

  .action-btn {
    padding: 0.7rem;
    background: rgba(139, 149, 165, 0.2);
    border: 1px solid rgba(139, 149, 165, 0.3);
    border-radius: 6px;
    color: #e0e6ed;
    font-size: 0.85rem;
    text-align: left;
    cursor: pointer;
    transition: all 0.2s;
  }

  .action-btn:hover {
    background: rgba(139, 149, 165, 0.3);
    border-color: #ff6b35;
  }

  .command-reference {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .ref-item {
    padding: 0.75rem;
    background: rgba(15, 20, 30, 0.4);
    border-radius: 4px;
  }

  .ref-command {
    font-family: 'Courier New', monospace;
    font-size: 0.85rem;
    color: #4caf50;
    margin-bottom: 0.25rem;
  }

  .ref-description {
    font-size: 0.75rem;
    color: #8b95a5;
    line-height: 1.4;
  }

  .btn-small {
    padding: 0.5rem 1rem;
    background: rgba(139, 149, 165, 0.2);
    border: 1px solid rgba(139, 149, 165, 0.3);
    border-radius: 6px;
    color: #e0e6ed;
    font-size: 0.85rem;
    cursor: pointer;
    transition: all 0.2s;
  }

  .btn-small:hover {
    background: rgba(139, 149, 165, 0.3);
  }
</style>
