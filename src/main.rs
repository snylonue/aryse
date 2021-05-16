use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

mod cli;

#[derive(Debug, Clone, Default)]
struct Playlist {
    pub sources: Vec<PathBuf>,
    pub pos: usize,
}
struct OpenImage(PathBuf);

struct CurrentImage;

struct Cached;

#[derive(Debug, Default)]
struct Cache(HashMap<PathBuf, Entity>);

impl Playlist {
    pub fn new(sources: Vec<PathBuf>) -> Self {
        Self { sources, pos: 0 }
    }
    pub fn next(&mut self) -> &Path {
        self.pos = (self.pos + 1) % self.sources.len();
        &self.sources[self.pos]
    }
    pub fn current(&self) -> &Path {
        &self.sources[self.pos]
    }
}

fn setup(mut commands: Commands, mut ev_open_img: EventWriter<OpenImage>) {
    let app = cli::app().get_matches();
    let images = app.values_of("image").unwrap().map(Into::into).collect();
    let playlist = Playlist::new(images);
    let first = playlist.current().to_owned();
    commands.insert_resource(playlist);
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    ev_open_img.send(OpenImage(first));
}

fn open_image(
    mut ev_open_img: EventReader<OpenImage>,
    asset_sever: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut queries: QuerySet<(
        Query<(&mut Visible, Entity), With<CurrentImage>>,
        Query<(&mut Visible, Entity), With<Cached>>,
    )>,
    mut cache: ResMut<Cache>,
    mut commands: Commands,
) {
    for OpenImage(path) in ev_open_img.iter() {
        if let Some((mut vis, id)) = queries.q0_mut().iter_mut().next() {
            vis.is_visible = false;
            commands.entity(id).remove::<CurrentImage>();
        }
        match cache.0.get(path) {
            Some(id) => {
                let (mut vis, id) = queries
                    .q1_mut()
                    .iter_mut()
                    .find(|(_, imgid)| imgid == id)
                    .unwrap();
                commands.entity(id).insert(CurrentImage);
                vis.is_visible = true;
            }
            None => {
                let handle = asset_sever.load(path.as_path());
                let id = commands
                    .spawn_bundle(SpriteBundle {
                        material: materials.add(handle.into()),
                        ..Default::default()
                    })
                    .insert(Cached)
                    .insert(CurrentImage)
                    .id();
                cache.0.insert(path.to_owned(), id);
            }
        }
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
        .init_resource::<Cache>()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(open_image.system())
        .add_system(keyboard.system())
        .run();
}
