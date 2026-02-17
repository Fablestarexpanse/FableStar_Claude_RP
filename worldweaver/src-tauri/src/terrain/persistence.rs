use rusqlite::{Connection, params};
use std::path::Path;
use super::heightmap::HeightmapChunk;
use super::rivers::RiverSegment;
use super::config::TerrainConfig;
use anyhow::{Result, Context};

/// SQL schema for terrain database
const SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS terrain_config (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS terrain_chunks (
    chunk_x INTEGER NOT NULL,
    chunk_z INTEGER NOT NULL,
    lod INTEGER NOT NULL DEFAULT 0,
    data BLOB NOT NULL,
    flow_data BLOB,
    biome_data BLOB,
    modified_at INTEGER NOT NULL,
    PRIMARY KEY (chunk_x, chunk_z, lod)
);

CREATE TABLE IF NOT EXISTS river_segments (
    id INTEGER PRIMARY KEY,
    path BLOB NOT NULL,
    strahler_order INTEGER NOT NULL,
    width_meters REAL NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_chunks_modified ON terrain_chunks(modified_at);
"#;

/// Terrain database manager
pub struct TerrainDatabase {
    conn: Connection,
}

impl TerrainDatabase {
    /// Create or open terrain database
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute_batch(SCHEMA)?;
        Ok(Self { conn })
    }

    /// Save terrain configuration
    pub fn save_config(&self, config: &TerrainConfig) -> Result<()> {
        let config_json = serde_json::to_string(config)?;
        self.conn.execute(
            "INSERT OR REPLACE INTO terrain_config (key, value) VALUES (?1, ?2)",
            params!["config", config_json],
        )?;
        Ok(())
    }

    /// Load terrain configuration
    pub fn load_config(&self) -> Result<TerrainConfig> {
        let config_json: String = self.conn.query_row(
            "SELECT value FROM terrain_config WHERE key = ?1",
            params!["config"],
            |row| row.get(0),
        )?;
        let config: TerrainConfig = serde_json::from_str(&config_json)?;
        Ok(config)
    }

    /// Save a chunk to database with zstd compression
    pub fn save_chunk(&self, chunk: &HeightmapChunk) -> Result<()> {
        // Serialize heights to bytes
        let heights_bytes: Vec<u8> = chunk.heights.iter()
            .flat_map(|h| h.to_le_bytes())
            .collect();

        // Compress with zstd
        let compressed = zstd::encode_all(&heights_bytes[..], 3)
            .context("Failed to compress chunk data")?;

        // Compress flow data if present
        let flow_compressed = if let Some(ref flow) = chunk.flow_accumulation {
            let flow_bytes: Vec<u8> = flow.iter()
                .flat_map(|f| f.to_le_bytes())
                .collect();
            Some(zstd::encode_all(&flow_bytes[..], 3)?)
        } else {
            None
        };

        // Compress biome data if present
        let biome_compressed = chunk.biome_ids.as_ref().map(|b| b.clone());

        let now = chrono::Utc::now().timestamp();

        self.conn.execute(
            "INSERT OR REPLACE INTO terrain_chunks 
             (chunk_x, chunk_z, lod, data, flow_data, biome_data, modified_at) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                chunk.coord.0,
                chunk.coord.1,
                chunk.lod,
                compressed,
                flow_compressed,
                biome_compressed,
                now,
            ],
        )?;

        Ok(())
    }

    /// Load a chunk from database
    pub fn load_chunk(&self, chunk_x: i32, chunk_z: i32, lod: u8) -> Result<HeightmapChunk> {
        let (compressed, flow_compressed, biome_data): (Vec<u8>, Option<Vec<u8>>, Option<Vec<u8>>) = 
            self.conn.query_row(
                "SELECT data, flow_data, biome_data FROM terrain_chunks 
                 WHERE chunk_x = ?1 AND chunk_z = ?2 AND lod = ?3",
                params![chunk_x, chunk_z, lod],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
            )?;

        // Decompress heights
        let heights_bytes = zstd::decode_all(&compressed[..])
            .context("Failed to decompress chunk data")?;
        
        let heights: Vec<f32> = heights_bytes
            .chunks_exact(4)
            .map(|chunk| f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
            .collect();

        // Decompress flow data if present
        let flow_accumulation = if let Some(flow_comp) = flow_compressed {
            let flow_bytes = zstd::decode_all(&flow_comp[..])?;
            let flow: Vec<f32> = flow_bytes
                .chunks_exact(4)
                .map(|chunk| f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
                .collect();
            Some(flow)
        } else {
            None
        };

        Ok(HeightmapChunk {
            coord: (chunk_x, chunk_z),
            heights,
            lod,
            flow_accumulation,
            biome_ids: biome_data,
        })
    }

    /// Check if a chunk exists
    pub fn chunk_exists(&self, chunk_x: i32, chunk_z: i32, lod: u8) -> Result<bool> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM terrain_chunks 
             WHERE chunk_x = ?1 AND chunk_z = ?2 AND lod = ?3",
            params![chunk_x, chunk_z, lod],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }

    /// Save a river segment
    pub fn save_river_segment(&self, segment: &RiverSegment) -> Result<()> {
        let path_bytes = bincode::serialize(&segment.path)?;

        self.conn.execute(
            "INSERT OR REPLACE INTO river_segments (id, path, strahler_order, width_meters) 
             VALUES (?1, ?2, ?3, ?4)",
            params![
                segment.id,
                path_bytes,
                segment.strahler_order,
                segment.width_meters,
            ],
        )?;

        Ok(())
    }

    /// Load all river segments
    pub fn load_river_segments(&self) -> Result<Vec<RiverSegment>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, path, strahler_order, width_meters FROM river_segments"
        )?;

        let segments = stmt.query_map([], |row| {
            let id: u32 = row.get(0)?;
            let path_bytes: Vec<u8> = row.get(1)?;
            let strahler_order: u8 = row.get(2)?;
            let width_meters: f32 = row.get(3)?;

            let path: Vec<(f32, f32)> = bincode::deserialize(&path_bytes)
                .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

            Ok(RiverSegment {
                id,
                path,
                strahler_order,
                width_meters,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

        Ok(segments)
    }

    /// Delete all chunks (for regeneration)
    pub fn clear_chunks(&self) -> Result<()> {
        self.conn.execute("DELETE FROM terrain_chunks", [])?;
        Ok(())
    }

    /// Delete all river segments
    pub fn clear_rivers(&self) -> Result<()> {
        self.conn.execute("DELETE FROM river_segments", [])?;
        Ok(())
    }

    /// Get chunk count
    pub fn get_chunk_count(&self) -> Result<i64> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM terrain_chunks",
            [],
            |row| row.get(0),
        )?;
        Ok(count)
    }
}
