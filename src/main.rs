use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::PresentMode;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

pub struct HelloPlugin;

#[derive(Resource)]
struct GreetTimer(Timer);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_startup_system(add_people)
            .add_system(greet_people);
    }
}

fn main() {
    App::new()
        .add_startup_system(hex_grid)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "I am a window!".into(),
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
