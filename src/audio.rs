use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        apply_global_volume.run_if(resource_changed::<GlobalVolume>),
    );
}

#[derive(Component, Reflect, Default, Clone)]
#[reflect(Component)]
pub struct Music;

pub fn music(handle: Handle<AudioSource>) -> impl Scene {
    bsn! {
        AudioPlayer(handle)
        PlaybackSettings::LOOP
        Music
    }
}

fn apply_global_volume(
    global_volume: Res<GlobalVolume>,
    mut audio_query: Query<(&PlaybackSettings, &mut AudioSink)>,
) {
    for (playback, mut sink) in &mut audio_query {
        sink.set_volume(global_volume.volume * playback.volume);
    }
}
