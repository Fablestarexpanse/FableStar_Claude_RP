use std::collections::HashMap;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

/// A quality (tracked stat/attribute) that gates storylets
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Quality {
    pub id: String,
    pub name: String,
    pub value: i32,
    pub min: i32,
    pub max: i32,
    pub description: String,
}

impl Quality {
    pub fn new(id: String, name: String, min: i32, max: i32) -> Self {
        Self {
            id,
            name,
            value: min,
            min,
            max,
            description: String::new(),
        }
    }
    
    pub fn set_value(&mut self, value: i32) {
        self.value = value.clamp(self.min, self.max);
    }
    
    pub fn modify(&mut self, change: i32) {
        self.value = (self.value + change).clamp(self.min, self.max);
    }
}

/// A storylet (narrative node) gated by quality requirements
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Storylet {
    pub id: String,
    pub title: String,
    pub description_template: String,  // LLM fills this in with context
    pub requirements: Vec<QualityRequirement>,
    pub branches: Vec<StoryletBranch>,
    pub category: String,  // "quest", "dialogue", "discovery", etc.
}

impl Storylet {
    pub fn new(id: String, title: String, description_template: String) -> Self {
        Self {
            id,
            title,
            description_template,
            requirements: Vec::new(),
            branches: Vec::new(),
            category: "general".to_string(),
        }
    }
    
    pub fn add_requirement(&mut self, requirement: QualityRequirement) {
        self.requirements.push(requirement);
    }
    
    pub fn add_branch(&mut self, branch: StoryletBranch) {
        self.branches.push(branch);
    }
}

/// Requirement for a quality to access a storylet or branch
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QualityRequirement {
    pub quality_id: String,
    pub min_value: Option<i32>,
    pub max_value: Option<i32>,
}

impl QualityRequirement {
    pub fn min(quality_id: String, min_value: i32) -> Self {
        Self {
            quality_id,
            min_value: Some(min_value),
            max_value: None,
        }
    }
    
    pub fn max(quality_id: String, max_value: i32) -> Self {
        Self {
            quality_id,
            min_value: None,
            max_value: Some(max_value),
        }
    }
    
    pub fn range(quality_id: String, min_value: i32, max_value: i32) -> Self {
        Self {
            quality_id,
            min_value: Some(min_value),
            max_value: Some(max_value),
        }
    }
    
    pub fn check(&self, value: i32) -> bool {
        if let Some(min) = self.min_value {
            if value < min {
                return false;
            }
        }
        if let Some(max) = self.max_value {
            if value > max {
                return false;
            }
        }
        true
    }
}

/// A branch within a storylet (player choice)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StoryletBranch {
    pub id: String,
    pub text_template: String,  // LLM generates narrative from this
    pub requirements: Vec<QualityRequirement>,
    pub effects: Vec<QualityEffect>,
    pub success_chance: Option<f32>,  // For skill checks (0.0-1.0)
}

impl StoryletBranch {
    pub fn new(id: String, text_template: String) -> Self {
        Self {
            id,
            text_template,
            requirements: Vec::new(),
            effects: Vec::new(),
            success_chance: None,
        }
    }
    
    pub fn with_success_chance(mut self, chance: f32) -> Self {
        self.success_chance = Some(chance.clamp(0.0, 1.0));
        self
    }
    
    pub fn add_requirement(&mut self, requirement: QualityRequirement) {
        self.requirements.push(requirement);
    }
    
    pub fn add_effect(&mut self, effect: QualityEffect) {
        self.effects.push(effect);
    }
}

/// Effect on a quality when a branch is chosen
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QualityEffect {
    pub quality_id: String,
    pub change: i32,
}

impl QualityEffect {
    pub fn new(quality_id: String, change: i32) -> Self {
        Self { quality_id, change }
    }
}

/// Manages storylets and qualities for entities
pub struct StoryletManager {
    qualities: HashMap<Uuid, HashMap<String, i32>>,  // entity_id -> quality_name -> value
    storylets: Vec<Storylet>,
}

impl StoryletManager {
    pub fn new() -> Self {
        Self {
            qualities: HashMap::new(),
            storylets: Vec::new(),
        }
    }
    
    /// Register a storylet
    pub fn add_storylet(&mut self, storylet: Storylet) {
        self.storylets.push(storylet);
    }
    
    /// Get or create qualities map for an entity
    fn get_qualities_mut(&mut self, entity_id: Uuid) -> &mut HashMap<String, i32> {
        self.qualities.entry(entity_id).or_insert_with(HashMap::new)
    }
    
    /// Get qualities for an entity (read-only)
    pub fn get_qualities(&self, entity_id: Uuid) -> Option<&HashMap<String, i32>> {
        self.qualities.get(&entity_id)
    }
    
    /// Set a quality value for an entity
    pub fn set_quality(&mut self, entity_id: Uuid, quality_id: String, value: i32) {
        let qualities = self.get_qualities_mut(entity_id);
        qualities.insert(quality_id, value);
    }
    
    /// Modify a quality value for an entity
    pub fn modify_quality(&mut self, entity_id: Uuid, quality_id: String, change: i32) {
        let qualities = self.get_qualities_mut(entity_id);
        let current = qualities.get(&quality_id).copied().unwrap_or(0);
        qualities.insert(quality_id, current + change);
    }
    
    /// Get a quality value for an entity
    pub fn get_quality(&self, entity_id: Uuid, quality_id: &str) -> i32 {
        self.qualities.get(&entity_id)
            .and_then(|q| q.get(quality_id))
            .copied()
            .unwrap_or(0)
    }
    
    /// Get all storylets available to an entity based on their qualities
    pub fn available_storylets(&self, entity_id: Uuid) -> Vec<&Storylet> {
        let empty_map = HashMap::new();
        let qualities = self.qualities.get(&entity_id).unwrap_or(&empty_map);
        
        self.storylets.iter()
            .filter(|s| self.check_requirements(&s.requirements, qualities))
            .collect()
    }
    
    /// Get available branches for a storylet
    pub fn available_branches<'a>(&self, entity_id: Uuid, storylet: &'a Storylet) -> Vec<&'a StoryletBranch> {
        let empty_map = HashMap::new();
        let qualities = self.qualities.get(&entity_id).unwrap_or(&empty_map);
        
        storylet.branches.iter()
            .filter(|b| self.check_requirements(&b.requirements, qualities))
            .collect()
    }
    
    /// Check if requirements are met
    fn check_requirements(&self, reqs: &[QualityRequirement], qualities: &HashMap<String, i32>) -> bool {
        reqs.iter().all(|req| {
            let value = qualities.get(&req.quality_id).copied().unwrap_or(0);
            req.check(value)
        })
    }
    
    /// Execute a branch (apply its effects)
    pub fn execute_branch(&mut self, entity_id: Uuid, branch: &StoryletBranch) {
        let qualities = self.get_qualities_mut(entity_id);
        
        for effect in &branch.effects {
            let current = qualities.get(&effect.quality_id).copied().unwrap_or(0);
            qualities.insert(effect.quality_id.clone(), current + effect.change);
        }
    }
    
    /// Check if a branch succeeds (for skill checks)
    pub fn check_success(&self, branch: &StoryletBranch, roll: f32) -> bool {
        if let Some(chance) = branch.success_chance {
            roll <= chance
        } else {
            true  // No chance means automatic success
        }
    }
}

impl Default for StoryletManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quality_clamping() {
        let mut quality = Quality::new("test".to_string(), "Test".to_string(), 0, 100);
        
        quality.set_value(150);
        assert_eq!(quality.value, 100);
        
        quality.set_value(-50);
        assert_eq!(quality.value, 0);
    }
    
    #[test]
    fn test_quality_requirement() {
        let req = QualityRequirement::range("test".to_string(), 10, 50);
        
        assert!(!req.check(5));
        assert!(req.check(10));
        assert!(req.check(30));
        assert!(req.check(50));
        assert!(!req.check(60));
    }
    
    #[test]
    fn test_storylet_availability() {
        let mut manager = StoryletManager::new();
        let entity_id = Uuid::new_v4();
        
        // Set up qualities
        manager.set_quality(entity_id, "courage".to_string(), 50);
        manager.set_quality(entity_id, "wisdom".to_string(), 30);
        
        // Create storylet requiring courage >= 40
        let mut storylet = Storylet::new(
            "test".to_string(),
            "Test Storylet".to_string(),
            "A test storylet".to_string(),
        );
        storylet.add_requirement(QualityRequirement::min("courage".to_string(), 40));
        
        manager.add_storylet(storylet);
        
        // Should be available
        let available = manager.available_storylets(entity_id);
        assert_eq!(available.len(), 1);
        
        // Lower courage below threshold
        manager.set_quality(entity_id, "courage".to_string(), 30);
        
        // Should not be available
        let available = manager.available_storylets(entity_id);
        assert_eq!(available.len(), 0);
    }
    
    #[test]
    fn test_branch_execution() {
        let mut manager = StoryletManager::new();
        let entity_id = Uuid::new_v4();
        
        manager.set_quality(entity_id, "gold".to_string(), 100);
        
        let mut branch = StoryletBranch::new(
            "buy".to_string(),
            "You purchase the item".to_string(),
        );
        branch.add_effect(QualityEffect::new("gold".to_string(), -50));
        branch.add_effect(QualityEffect::new("items".to_string(), 1));
        
        manager.execute_branch(entity_id, &branch);
        
        assert_eq!(manager.get_quality(entity_id, "gold"), 50);
        assert_eq!(manager.get_quality(entity_id, "items"), 1);
    }
}
