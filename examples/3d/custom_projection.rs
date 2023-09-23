//! Demonstrates creation of a custom camera projection.

use bevy::{
    prelude::*,
    render::camera::{CameraProjection, CameraProjectionPlugin, ScalingMode},
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            CameraProjectionPlugin::<InterpolatedProjection>::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, update_interpolation)
        .run();
}

/// A simple custom projection that interpolates between perspective and orthographic
#[derive(Component, Reflect, Default)]
pub struct InterpolatedProjection {
    pub orthographic: OrthographicProjection,
    pub perspective: PerspectiveProjection,
    /// 0.0: Fully orthographic, 1.0: Fully perspective
    pub lerp: f32,
}

impl CameraProjection for InterpolatedProjection {
    fn get_projection_matrix(&self) -> Mat4 {
        let lerp = self.lerp.clamp(0.0, 1.0);
        self.perspective.get_projection_matrix() * lerp
            + (1.0 - lerp) * self.orthographic.get_projection_matrix()
    }

    fn update(&mut self, width: f32, height: f32) {
        self.perspective.update(width, height);
        self.orthographic.update(width, height);
    }

    fn far(&self) -> f32 {
        let lerp = self.lerp.clamp(0.0, 1.0);
        self.perspective.far() * lerp + (1.0 - lerp) * self.orthographic.far()
    }
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // camera
    commands
        .spawn((
            Camera3dBundle {
                transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
            },
            InterpolatedProjection {
                orthographic: OrthographicProjection {
                    scale: 3.0,
                    scaling_mode: ScalingMode::FixedVertical(2.0),
                    ..default()
                },
                perspective: PerspectiveProjection { ..default() },
                ..default()
            },
        ))
        .remove::<Projection>();

    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // cubes
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(1.5, 0.5, 1.5),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(1.5, 0.5, -1.5),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(-1.5, 0.5, 1.5),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(-1.5, 0.5, -1.5),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(3.0, 8.0, 5.0),
        ..default()
    });
}

/// Allow the user to adjust the interpolation
fn update_interpolation(
    mut query: Query<&mut InterpolatedProjection>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    if keyboard.pressed(KeyCode::Right) {
        query.for_each_mut(|mut projection| {
            projection.lerp += (0.1 * time.delta_seconds()).clamp(0., 1.)
        });
    }
    if keyboard.pressed(KeyCode::Left) {
        query.for_each_mut(|mut projection| {
            projection.lerp -= (0.1 * time.delta_seconds()).clamp(0., 1.)
        });
    }
}
