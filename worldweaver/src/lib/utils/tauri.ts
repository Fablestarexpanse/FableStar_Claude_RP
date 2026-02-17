import { invoke } from '@tauri-apps/api/core';

export interface Exit {
  direction: string;
  target_room_id: string;
  description?: string;
}

export interface RoomDetails {
  id: string;
  name: string;
  description: string;
  exits: Exit[];
}

export interface NpcInfo {
  name: string;
  description: string;
  personality: string;
  greeting: string;
}

export async function getCurrentRoom(): Promise<RoomDetails> {
  return await invoke<RoomDetails>('get_current_room');
}

export async function getNpcsInCurrentRoom(): Promise<NpcInfo[]> {
  return await invoke<NpcInfo[]>('get_npcs_in_current_room');
}

export async function movePlayer(direction: string): Promise<RoomDetails> {
  return await invoke<RoomDetails>('move_player', { direction });
}

export async function sendPlayerAction(action: string): Promise<string> {
  return await invoke<string>('send_player_action', { action });
}

export async function getWorldTick(): Promise<number> {
  return await invoke<number>('get_world_tick');
}
