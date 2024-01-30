mod asset_loader;
mod asteroids;
mod camera;
mod collision_detection;
mod debug;
mod despawn;
mod movement;
mod spaceship;
mod schedule;
mod state;

use crate::asset_loader::AssetLoaderPlugin;
use crate::asteroids::AsteroidPlugin;
use crate::collision_detection::CollisionDetectionPlugin;
use crate::despawn::DespawnPlugin;
use bevy::prelude::*;
use camera::CameraPlugin;
use movement::MovementPlugin;
use spaceship::SpaceshipPlugin;
use schedule::SchedulePlugin;
use state::StatePlugin;

fn main() {
	App::new()
		// Bevy built-ins
		.insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
		.insert_resource(AmbientLight {
			color: Color::default(),
			brightness: 0.75,
		})
		.add_plugins(DefaultPlugins)
		// User configured plugins
		.add_plugins(AssetLoaderPlugin)
		.add_plugins(SpaceshipPlugin)
		.add_plugins(CameraPlugin)
		.add_plugins(MovementPlugin)
		.add_plugins(AsteroidPlugin)
		.add_plugins(CollisionDetectionPlugin)
		.add_plugins(DespawnPlugin)
		.add_plugins(SchedulePlugin)
		.add_plugins(StatePlugin)
		// .add_plugins(DebugPlugin)
		.run();
}
