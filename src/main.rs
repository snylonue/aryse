use bevy::prelude::*;
use bevy::input::keyboard::KeyboardInput;
use std::path::Path;
use std::path::PathBuf;

mod cli;

#[derive(Debug, Clone, Default)]
struct Playlist {
    pub sources: Vec<PathBuf>,
    pub pos: usize,
}
struct OpenImage(PathBuf);

struct ImageTimer(Timer);

impl Playlist {
    pub fn new(sources: Vec<PathBuf>) -> Self {
        Self { sources, pos: 0 }
    }
    pub fn next(&mut self) -> &Path {
        let tmp = self.sources[self.pos].as_path();
        self.pos = (self.pos + 1) % self.sources.len();
        tmp
    }
}

fn setup(mut commands: Commands) {
    let app = cli::app().get_matches();
    let images = app.values_of("image").unwrap().map(Into::into).collect();
    let playlist = Playlist::new(images);
    commands.insert_resource(playlist);
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn open_image(
    mut ev_open_img: EventReader<OpenImage>,
    asset_sever: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    for OpenImage(ref path) in ev_open_img.iter() {
        let handle = asset_sever.load(dbg!(path).as_path());
        commands.spawn_bundle(SpriteBundle {
            material: materials.add(handle.into()),
            ..Default::default()
        });
    }
}

fn keyboard(
    mut ev_key: EventReader<KeyboardInput>,
    mut ev_open_img: EventWriter<OpenImage>,
    mut playlist: ResMut<Playlist>,
) {
    for ev in ev_key.iter() {
        match ev.state {
            bevy::input::ElementState::Pressed if ev.key_code == Some(KeyCode::Right) => {
                let img = playlist.next();
                ev_open_img.send(OpenImage(img.to_owned()));
            }
            _ => {}
        }
    }
}

fn main() {
    App::build()
        .add_event::<OpenImage>()
        .insert_resource(ImageTimer(Timer::from_seconds(1.0, true)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(open_image.system())
        .add_system(keyboard.system())
        .run();
}
