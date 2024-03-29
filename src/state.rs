use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
	#[default]
	InGame,
	Pause,
	GameOver,
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
	fn build(&self, app: &mut App) {
		app.add_state::<GameState>()
			.add_systems(Update, game_state_input_events);
	}
}

pub fn game_state_input_events(
	mut next_state: ResMut<NextState<GameState>>,
	state: Res<State<GameState>>,
	keyboard_input: Res<Input<KeyCode>>,
) {
	if keyboard_input.just_pressed(KeyCode::Escape) {
		match state.get() {
			GameState::InGame => next_state.set(GameState::Pause),
			GameState::Pause => next_state.set(GameState::InGame),
			_ => (),
		}
	}
}
