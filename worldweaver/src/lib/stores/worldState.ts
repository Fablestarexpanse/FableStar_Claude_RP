import { writable } from 'svelte/store';
import type { RoomDetails, NpcInfo } from '../utils/tauri';

export const currentRoom = writable<RoomDetails | null>(null);
export const currentNpcs = writable<NpcInfo[]>([]);
export const narrativeLog = writable<string[]>([]);
export const isLoading = writable<boolean>(false);
export const visitedRooms = writable<Set<string>>(new Set());
