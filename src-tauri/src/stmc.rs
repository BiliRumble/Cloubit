use windows::Media::Playback::MediaPlayer;
use windows::Media::MediaPlaybackType;
use windows::Foundation::Uri;
use windows::core::HSTRING;
use windows::Storage::Streams::RandomAccessStreamReference;

pub fn update_system_media_info(title: &str, artist: &str, cover_url: &str) {
    let player = match MediaPlayer::new() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to create MediaPlayer: {:?}", e);
            return;
        }
    };

    let controls = match player.SystemMediaTransportControls() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to get SystemMediaTransportControls: {:?}", e);
            return;
        }
    };

    let updater = match controls.DisplayUpdater() {
        Ok(u) => u,
        Err(e) => {
            eprintln!("Failed to get DisplayUpdater: {:?}", e);
            return;
        }
    };

    // 设置媒体类型为音乐
    if let Err(e) = updater.SetType(MediaPlaybackType::Music) {
        eprintln!("Failed to set media type: {:?}", e);
        return;
    }

    // 获取音乐属性
    let music_properties = match updater.MusicProperties() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to get MusicProperties: {:?}", e);
            return;
        }
    };

    // 设置歌曲名和歌手名
    if let Err(e) = music_properties.SetTitle(&HSTRING::from(title)) {
        eprintln!("Failed to set title: {:?}", e);
        return;
    }

    if let Err(e) = music_properties.SetArtist(&HSTRING::from(artist)) {
        eprintln!("Failed to set artist: {:?}", e);
        return;
    }

    // 设置封面URL
    let cover_uri = match Uri::CreateUri(&HSTRING::from(cover_url)) {
        Ok(u) => u,
        Err(e) => {
            eprintln!("Failed to create Uri from cover_url: {:?}", e);
            return;
        }
    };

    let thumbnail = match RandomAccessStreamReference::CreateFromUri(&cover_uri) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Failed to create RandomAccessStreamReference from Uri: {:?}", e);
            return;
        }
    };

    if let Err(e) = updater.SetThumbnail(&thumbnail) {
        eprintln!("Failed to set thumbnail: {:?}", e);
        return;
    }

    // 更新显示信息
    if let Err(e) = updater.Update() {
        eprintln!("Failed to update display information: {:?}", e);
    } else {
        println!("System media info updated successfully.");
    }
}
