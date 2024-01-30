use crate::{asteroids::Asteroid, schedule::InGameSet};
use bevy::prelude::*;

const DESPAWN_DISTANCE: f32 = 50.0;

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(
			Update,
			despawn_far_away_entities.in_set(InGameSet::DespawnEntities),
		);
	}
}

fn despawn_far_away_entities(
	mut commands: Commands,
	query: Query<(Entity, &GlobalTransform), With<Asteroid>>,
) {
	for (entity, transform) in query.iter() {
		let distance = transform.translation().distance(Vec3::ZERO);

		// Entity is far away from the camera's viewport (i.e. origin)
		if distance > DESPAWN_DISTANCE {
			commands.entity(entity).despawn_recursive();
		}
	}
}
