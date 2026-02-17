export interface TerrainConfig {
  chunk_size: number;
  vertex_count: number;
  world_width: number;
  world_height: number;
  cell_size_meters: number;
  max_elevation: number;
  sea_level: number;
  seed: number;
  theme: 'Fantasy' | 'Modern' | 'SciFi';
}

export interface RiverSegment {
  id: number;
  path: [number, number][];
  strahler_order: number;
  width_meters: number;
}

export interface RoomMarker {
  id: string;
  name: string;
  x: number;
  z: number;
  elevation: number;
  biome?: string;
}

export interface NoiseParameters {
  continent_frequency: number;
  continent_octaves: number;
  mountain_frequency: number;
  mountain_octaves: number;
  hill_frequency: number;
  hill_octaves: number;
  detail_frequency: number;
  detail_octaves: number;
  land_coverage?: number;  // Threshold for land vs ocean (0.0-1.0)
}

export interface GenerateTerrainRequest {
  width: number;
  height: number;
  seed: number;
  theme: 'Fantasy' | 'Modern' | 'SciFi';
  use_erosion: boolean;
  erosion_iterations: number;
  noise_params?: NoiseParameters;
}

export interface GenerateTerrainResponse {
  success: boolean;
  message: string;
  chunk_count: number;
}

export interface GetChunkRequest {
  chunk_x: number;
  chunk_z: number;
  lod: number;
}

export interface ApplyBrushRequest {
  chunk_x: number;
  chunk_z: number;
  center_x: number;
  center_z: number;
  radius: number;
  strength: number;
  brush_type: string;
}

export type BrushType = 'raise' | 'lower' | 'smooth' | 'flatten' | 'erode' | 'noise';

export interface ViewTransform {
  translateX: number;
  translateY: number;
  scale: number;
  rotation: number;
}

export interface VisibleChunks {
  chunks: Set<string>; // "x,z" format
  minX: number;
  maxX: number;
  minZ: number;
  maxZ: number;
}
