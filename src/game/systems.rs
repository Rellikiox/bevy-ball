use bevy::prelude::*;

use super::SimulationState;

pub fn toggle_simulation_state(
    keyboard_input: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<SimulationState>>,
    simulation_state: Res<State<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        match simulation_state.get() {
            SimulationState::Paused => {
                next_state.set(SimulationState::Running);
                println!("Simulation running");
            }
            SimulationState::Running => {
                next_state.set(SimulationState::Paused);
                println!("SImulation paused");
            }
        };
    }
}
