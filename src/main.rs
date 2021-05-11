use bevy::prelude::*;
use bevy::{input::keyboard::KeyboardInput, render::render_graph::base::MainPass};
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

#[derive(Debug, Default)]
struct LastId(Option<Entity>);

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
    mut last_id: ResMut<LastId>,
    mut commands: Commands,
) {
    for OpenImage(ref path) in ev_open_img.iter() {
        let handle = asset_sever.load(dbg!(path).as_path());
        let id = commands
            .spawn_bundle(SpriteBundle {
                material: materials.add(handle.into()),
                ..Default::default()
            })
            .id();
        if let Some(pre_id) = last_id.0 {
            commands
                .entity(pre_id)
                .remove::<Sprite>()
                .remove::<Handle<Mesh>>()
                .remove::<MainPass>()
                .remove::<Draw>()
                .remove::<Visible>()
                .remove::<RenderPipelines>()
                .remove::<Transform>()
                .remove::<GlobalTransform>();
            // ev_hide_img.send(HideImage(pre_id));
        }
        last_id.0.replace(id);
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
        .init_resource::<LastId>()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(open_image.system())
        .add_system(keyboard.system())
        .run();
}
