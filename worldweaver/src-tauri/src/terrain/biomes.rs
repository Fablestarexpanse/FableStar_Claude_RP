use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use super::config::WorldTheme;

/// Biome types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Biome {
    Ocean,
    Coast,
    TropicalRainforest,
    TemperateForest,
    BorealForest,
    Tundra,
    Grassland,
    Savanna,
    Desert,
    Alpine,
    Glacier,
}

/// Biome definition with display properties
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BiomeDefinition {
    pub name: String,
    pub color: [u8; 3],
    pub theme_names: HashMap<WorldTheme, String>,
}

/// Registry of all biome definitions
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BiomeRegistry {
    pub definitions: HashMap<Biome, BiomeDefinition>,
}

impl Default for BiomeRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl BiomeRegistry {
    pub fn new() -> Self {
        let mut definitions = HashMap::new();

        definitions.insert(
            Biome::Ocean,
            BiomeDefinition {
                name: "Ocean".to_string(),
                color: [30, 60, 120],
                theme_names: HashMap::from([
                    (WorldTheme::Fantasy, "The Endless Sea".to_string()),
                    (WorldTheme::Modern, "Ocean".to_string()),
                    (WorldTheme::SciFi, "Liquid Expanse".to_string()),
                ]),
            },
        );

        definitions.insert(
            Biome::Coast,
            BiomeDefinition {
                name: "Coast".to_string(),
                color: [130, 195, 210],
                theme_names: HashMap::from([
                    (WorldTheme::Fantasy, "Coastal Shores".to_string()),
                    (WorldTheme::Modern, "Coastline".to_string()),
                    (WorldTheme::SciFi, "Shore Zone".to_string()),
                ]),
            },
        );

        definitions.insert(
            Biome::TropicalRainforest,
            BiomeDefinition {
                name: "Tropical Rainforest".to_string(),
                color: [34, 139, 34],
                theme_names: HashMap::from([
                    (WorldTheme::Fantasy, "Verdant Jungle".to_string()),
                    (WorldTheme::Modern, "Rainforest".to_string()),
                    (WorldTheme::SciFi, "Bio-Dense Zone".to_string()),
                ]),
            },
        );

        definitions.insert(
            Biome::TemperateForest,
            BiomeDefinition {
                name: "Temperate Forest".to_string(),
                color: [110, 180, 80],
                theme_names: HashMap::from([
                    (WorldTheme::Fantasy, "Ancient Woods".to_string()),
                    (WorldTheme::Modern, "Forest".to_string()),
                    (WorldTheme::SciFi, "Temperate Biomass".to_string()),
                ]),
            },
        );

        definitions.insert(
            Biome::BorealForest,
            BiomeDefinition {
                name: "Boreal Forest".to_string(),
                color: [90, 120, 70],
                theme_names: HashMap::from([
                    (WorldTheme::Fantasy, "Northern Pines".to_string()),
                    (WorldTheme::Modern, "Taiga".to_string()),
                    (WorldTheme::SciFi, "Cold Forest Zone".to_string()),
                ]),
            },
        );

        definitions.insert(
            Biome::Tundra,
            BiomeDefinition {
                name: "Tundra".to_string(),
                color: [180, 190, 200],
                theme_names: HashMap::from([
                    (WorldTheme::Fantasy, "Frozen Wastes".to_string()),
                    (WorldTheme::Modern, "Tundra".to_string()),
                    (WorldTheme::SciFi, "Cryo-Plains".to_string()),
                ]),
            },
        );

        definitions.insert(
            Biome::Grassland,
            BiomeDefinition {
                name: "Grassland".to_string(),
                color: [180, 200, 110],
                theme_names: HashMap::from([
                    (WorldTheme::Fantasy, "Rolling Plains".to_string()),
                    (WorldTheme::Modern, "Grassland".to_string()),
                    (WorldTheme::SciFi, "Grass Expanse".to_string()),
                ]),
            },
        );

        definitions.insert(
            Biome::Savanna,
            BiomeDefinition {
                name: "Savanna".to_string(),
                color: [210, 185, 110],
                theme_names: HashMap::from([
                    (WorldTheme::Fantasy, "Golden Savanna".to_string()),
                    (WorldTheme::Modern, "Savanna".to_string()),
                    (WorldTheme::SciFi, "Dry Grassland".to_string()),
                ]),
            },
        );

        definitions.insert(
            Biome::Desert,
            BiomeDefinition {
                name: "Desert".to_string(),
                color: [220, 190, 140],
                theme_names: HashMap::from([
                    (WorldTheme::Fantasy, "Scorching Sands".to_string()),
                    (WorldTheme::Modern, "Desert".to_string()),
                    (WorldTheme::SciFi, "Arid Zone".to_string()),
                ]),
            },
        );

        definitions.insert(
            Biome::Alpine,
            BiomeDefinition {
                name: "Alpine".to_string(),
                color: [170, 120, 80],
                theme_names: HashMap::from([
                    (WorldTheme::Fantasy, "Mountain Peaks".to_string()),
                    (WorldTheme::Modern, "Alpine".to_string()),
                    (WorldTheme::SciFi, "High Altitude Zone".to_string()),
                ]),
            },
        );

        definitions.insert(
            Biome::Glacier,
            BiomeDefinition {
                name: "Glacier".to_string(),
                color: [245, 245, 250],
                theme_names: HashMap::from([
                    (WorldTheme::Fantasy, "Eternal Ice".to_string()),
                    (WorldTheme::Modern, "Glacier".to_string()),
                    (WorldTheme::SciFi, "Ice Sheet".to_string()),
                ]),
            },
        );

        Self { definitions }
    }

    pub fn get_name(&self, biome: Biome, theme: WorldTheme) -> String {
        self.definitions
            .get(&biome)
            .and_then(|def| def.theme_names.get(&theme))
            .cloned()
            .unwrap_or_else(|| format!("{:?}", biome))
    }
}

/// Classify biome based on temperature and moisture (Whittaker diagram)
pub fn classify_biome(elevation: f32, temperature: f32, moisture: f32, sea_level: f32) -> Biome {
    // Below sea level = ocean
    if elevation < sea_level {
        return Biome::Ocean;
    }

    // Just above sea level = coast
    if elevation < sea_level + 0.02 {
        return Biome::Coast;
    }

    // Very high elevation = alpine or glacier
    if elevation > 0.85 {
        return if temperature < 0.0 {
            Biome::Glacier
        } else {
            Biome::Alpine
        };
    }

    // Whittaker diagram classification
    if temperature < -10.0 {
        Biome::Tundra
    } else if temperature < 0.0 {
        if moisture > 0.6 {
            Biome::BorealForest
        } else {
            Biome::Tundra
        }
    } else if temperature < 15.0 {
        if moisture > 0.7 {
            Biome::TemperateForest
        } else if moisture > 0.3 {
            Biome::Grassland
        } else {
            Biome::Desert
        }
    } else {
        // Hot climates
        if moisture > 0.7 {
            Biome::TropicalRainforest
        } else if moisture > 0.4 {
            Biome::Savanna
        } else {
            Biome::Desert
        }
    }
}

/// Generate temperature map based on elevation and latitude
pub fn generate_temperature(
    elevation: f32,
    latitude: f32,
    max_elevation: f32,
) -> f32 {
    // Base temperature from latitude (0 = equator, 1 = pole)
    let base_temp = 30.0 - latitude * 40.0; // 30°C at equator, -10°C at pole

    // Atmospheric lapse rate: -6.5°C per 1000m
    let elevation_meters = elevation * max_elevation;
    let temp_reduction = elevation_meters * 0.0065;

    base_temp - temp_reduction
}

/// Generate moisture map with orographic effects
pub fn generate_moisture(
    _x: usize,
    _z: usize,
    elevation: f32,
    prev_elevation: f32,
    moisture: f32,
    sea_level: f32,
) -> f32 {
    let mut new_moisture = moisture;

    // Pick up moisture over water
    if elevation < sea_level {
        new_moisture = (new_moisture + 0.1).min(1.0);
    }

    // Orographic rainfall: deposit moisture when going uphill
    if elevation > prev_elevation && prev_elevation >= sea_level {
        let slope = elevation - prev_elevation;
        let rainfall = (slope * 2.0).min(new_moisture * 0.5);
        new_moisture -= rainfall;
    }

    // Evaporation over land
    if elevation >= sea_level {
        new_moisture *= 0.98;
    }

    new_moisture.clamp(0.0, 1.0)
}
