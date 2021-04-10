use bevy::prelude::*;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Default)]
struct Playlist {
    pub sources: Vec<PathBuf>,
    pub pos: usize,
}

struct OpenImage(PathBuf);

struct ImageTimer(Timer);

impl Playlist {
    pub fn next(&mut self) -> &Path {
        self.pos = (self.pos + 1) % self.sources.len();
        self.sources[self.pos].as_path()
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_event::<OpenImage>()
        .add_resource(ImageTimer(Timer::from_seconds(1.0, true)))
        // .add_resource(Playlist {
            // sources: vec![
                // PathBuf::from(r"D:\Files\Image\collection\pixiv\87165096_p0.png"),
                // PathBuf::from(r"D:\Files\Image\collection\pixiv\73739752_p0.png")
            // ],
            // pos: 0
        // })
        .init_resource::<Playlist>()
        .add_startup_system(add_image.system())
        .add_system(show_image.system())
        .add_system(open_image.system())
        .run();
}

fn add_image(commands: &mut Commands,mut playlist: ResMut<Playlist>) {
    commands.spawn(Camera2dBundle::default());
    playlist.sources.extend(vec![
        PathBuf::from(r"D:\Files\Image\collection\pixiv\87165096_p0.png"),
        PathBuf::from(r"D:\Files\Image\collection\pixiv\73739752_p0.png")
    ]);
}
fn show_image(
    mut ev_open_img: ResMut<Events<OpenImage>>,
    mut playlist: ResMut<Playlist>,
    time: Res<Time>,
    mut timer: ResMut<ImageTimer>,
) {
    if !timer.0.tick(time.delta_seconds()).just_finished() || playlist.sources.len() == 0 {
        return;
    }
    ev_open_img.send(OpenImage(playlist.next().into()));
}
fn open_image(
    events: Res<Events<OpenImage>>,
    mut ev_reader: Local<EventReader<OpenImage>>,
    asset_sever: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    commands: &mut Commands,
) {
    for OpenImage(ref path) in ev_reader.iter(&events) {
        let handle = asset_sever.load(dbg!(path).as_path());
        commands.spawn(SpriteBundle {
            material: materials.add(handle.into()),
            ..Default::default()
        });
    }
}
