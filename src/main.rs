use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::PresentMode;

fn main() {
    App::new()
        .add_startup_system(hex_grid)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resize_constraints: WindowResizeConstraints {
                    min_width: 1280.,
                    min_height: 720.,
                    max_width: 1280.,
                    max_height: 720.,
                },
                title: "hex grid demo".into(),
                resolution: (1280., 720.).into(),
                present_mode: PresentMode::AutoVsync,
                // Tells wasm to resize the window according to the available canvas
                fit_canvas_to_parent: true,
                // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .run();
}

fn hex_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    // Hexagon middle

    let grid_height = 8;
    let grid_width = 12;
    let horizontal: f32 = 3.0_f32.sqrt() * 50.0;
    let vert: f32 = 3. / 2. * 50.;
    for x in 0..grid_width {
        for y in 0..grid_height {
            if y % 2 == 0 {
                commands.spawn(MaterialMesh2dBundle {
                    mesh: meshes.add(shape::RegularPolygon::new(50., 6).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::BLACK)),
                    transform: Transform::from_translation(Vec3::new(
                        horizontal * (x as f32) - (1280. / 3.) - horizontal as f32,
                        vert * y as f32 - (720. / 3.),
                        0.,
                    )),
                    ..default()
                });
                commands.spawn(MaterialMesh2dBundle {
                    mesh: meshes.add(shape::RegularPolygon::new(48., 6).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::WHITE)),
                    transform: Transform::from_translation(Vec3::new(
                        horizontal * (x as f32) - (1280. / 3.) - horizontal as f32,
                        vert * y as f32 - (720. / 3.),
                        1.,
                    )),
                    ..default()
                });
            } else {
                commands.spawn(MaterialMesh2dBundle {
                    mesh: meshes.add(shape::RegularPolygon::new(50., 6).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::BLACK)),
                    transform: Transform::from_translation(Vec3::new(
                        horizontal * (0.5 + x as f32) - (1280. / 3.) - horizontal as f32,
                        vert * y as f32 - (720. / 3.),
                        0.,
                    )),
                    ..default()
                });
                commands.spawn(MaterialMesh2dBundle {
                    mesh: meshes.add(shape::RegularPolygon::new(48., 6).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::WHITE)),
                    transform: Transform::from_translation(Vec3::new(
                        horizontal * (0.5 + x as f32) - (1280. / 3.) - horizontal as f32,
                        vert * y as f32 - (720. / 3.),
                        1.,
                    )),
                    ..default()
                });
            }
        }
    }
}
