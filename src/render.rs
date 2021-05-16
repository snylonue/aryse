use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use bevy::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct Playlist {
    pub sources: Vec<PathBuf>,
    pub pos: usize,
}
pub struct OpenImage(pub PathBuf);

pub struct CurrentImage;

pub struct Cached;

#[derive(Debug, Default)]
pub struct Cache(HashMap<PathBuf, Entity>);

pub struct RenderPlugin;

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

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<OpenImage>()
        .init_resource::<Cache>()
        .add_system(open_image.system());
    }
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