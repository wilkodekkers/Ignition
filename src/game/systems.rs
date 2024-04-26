use bevy::prelude::*;

use crate::game::SimulationState;

pub fn toggle_simulation(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    state: Res<State<SimulationState>>,
    mut next_state: ResMut<NextState<SimulationState>>
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        match state.get() {
            SimulationState::Running => {
                next_state.set(SimulationState::Paused);
                println!("Simulation Paused");
            },
            SimulationState::Paused => {
                next_state.set(SimulationState::Running);
                println!("Simulation Running");
            },
        }
    }
}
