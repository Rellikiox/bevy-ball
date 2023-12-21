use bevy::prelude::*;

use super::SimulationState;

pub fn toggle_simulation_state(
    keyboard_input: Res<Input<KeyCode>>,
    next_state: ResMut<NextState<SimulationState>>,
    simulation_state: Res<State<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        match simulation_state.get() {
            SimulationState::Paused => resume_simulation(next_state),
            SimulationState::Running => pause_simulation(next_state),
        };
    }
}

pub fn pause_simulation(mut next_state: ResMut<NextState<SimulationState>>) {
    next_state.set(SimulationState::Paused);
}

pub fn resume_simulation(mut next_state: ResMut<NextState<SimulationState>>) {
    next_state.set(SimulationState::Running);
}
