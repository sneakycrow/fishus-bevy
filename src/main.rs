use avian3d::prelude::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            // PhysicsDebugPlugin::default(),
        ))
        .add_systems(Startup, setup_graphics)
        .add_systems(Startup, setup_physics)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-3.0, 3.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}

fn setup_physics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(100.0, 0.1, 100.0))
        .insert(RigidBody::Static)
        .insert(Transform::from_xyz(0.0, -2.0, 0.0))
        .insert(Restitution::new(1.0))
        .insert(Mesh3d(
            meshes.add(Cuboid::from_size(Vec3::new(100.0, 0.1, 100.0))),
        ))
        .insert(MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))))
        .insert(Friction::new(0.0));

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::sphere(0.25))
        .insert(Restitution::new(1.0))
        .insert(Friction::new(0.0))
        .insert(SpeculativeMargin(2.0))
        .insert(LinearDamping(1.0))
        .insert(AngularDamping(1.0))
        .insert(Transform::from_xyz(0.0, 1.0, 0.0))
        .insert(LinearVelocity(Vec3::ZERO))
        .insert(Mesh3d(meshes.add(Sphere::new(0.25))))
        .insert(MeshMaterial3d(materials.add(Color::srgb_u8(255, 124, 144))));
}
