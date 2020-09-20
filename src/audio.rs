use amethyst::{
    assets::Loader,
    audio::{AudioSink, Mp3Format, SourceHandle},
    ecs::{World, WorldExt},
};
use std::{iter::Cycle, vec::IntoIter};

const OVERWORLD_THEME: &str = "audio/wanderball-overworld-theme.mp3";

const MUSIC_TRACKS: &[&str] = &[OVERWORLD_THEME];

pub struct Music {
    pub music: Cycle<IntoIter<SourceHandle>>,
}

fn load_audio_track(loader: &Loader, world: &World, file: &str) -> SourceHandle {
    loader.load(file, Mp3Format, (), &world.read_resource())
}

pub fn initialize_audio(world: &mut World) {
    world.insert(Music {
        music: vec![].into_iter().cycle(),
    });
}

pub fn start_audio(world: &mut World) {
    let music = {
        let loader = world.read_resource::<Loader>();
        let mut sink = world.write_resource::<AudioSink>();
        sink.set_volume(0.20);

        let music = MUSIC_TRACKS
            .iter()
            .map(|file| load_audio_track(&loader, &world, file))
            .collect::<Vec<_>>()
            .into_iter()
            .cycle();

        Music { music }
    };

    world.insert(music);
}
