pub mod agent;
pub mod plant_grid;

struct World {
    agents: Vec<agent::Agent>,
    plant_grid: plant_grid::PlantGrid
}
