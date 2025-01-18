use windows::Media::Playback::MediaPlayer;
use windows::Media::MediaPlaybackType;

pub(crate) async fn update_system_media_info(title: &str, artist: &str, cover_url: &str) {
    let player = MediaPlayer::new().unwrap();
    let controls = player.SystemMediaTransportControls().unwrap();
    let updater = controls.DisplayUpdater().unwrap();

    updater.SetType(MediaPlaybackType::Music).unwrap();
    let music_properties = updater.MusicProperties().unwrap();
    music_properties.SetTitle(HSTRING::from(title)).unwrap();
    music_properties.SetArtist(HSTRING::from(artist)).unwrap();
    updater.Update().unwrap();
}

