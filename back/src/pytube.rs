
use inline_python::{python, Context};



pub async fn download_video(url: &String, file_path: String) -> anyhow::Result<String> {
    println!("PROCESS: Downloading video...");
    let c: Context = python! {
        from pytubefix import YouTube
        from pytubefix.cli import on_progress

        try:
            yt = YouTube('url, use_oauth=True,
            allow_oauth_cache=True)

            video_stream = yt.streams.filter(adaptive=True, res="720p", file_extension="mp4").first()
            video_stream.download(output_path='file_path, filename="video_stream.mp4")
            holder = "Some"
            holder

        except:
            holder = "None"
            holder
    };

    let holder: String = c.get("holder");
    Ok(holder)
}

pub async fn download_audio(url: &String, file_path: String) -> anyhow::Result<()> {
    println!("PROCESS: Downloading audio...");
    let c: Context = python! {
        from pytubefix import YouTube
        from pytubefix.cli import on_progress

        try:
            yt = YouTube('url, use_oauth=True,
            allow_oauth_cache=True)

            audio_stream = yt.streams.filter(progressive=True, file_extension="mp4").first()
            audio_stream.download(output_path='file_path, filename="audio_stream.mp4")

        except:
            audio_stream = "None"
            audio_stream
    };
    Ok(())
}

pub async fn get_thumb_url(url: &String, file_path: String) -> anyhow::Result<String> {
    println!("PROCESS: Downloading thumb...");
    let c: Context = python! {
        from pytubefix import YouTube
        from pytubefix.cli import on_progress

        try:
            yt = YouTube('url, use_oauth=True,
            allow_oauth_cache=True)

            video_stream = yt.streams.filter(adaptive=True).first().url
            thumb = yt.thumbnail_url

        except:
            thumb = "None"
            video_stream
    };
    let thumb: String = c.get("thumb");
    Ok(thumb)
}

pub async fn get_title(url: &String) -> anyhow::Result<String> {
    println!("PROCESS: Getting title...");
    let c: Context = python! {
        from pytubefix import YouTube
        from pytubefix.cli import on_progress

        try:
            yt = YouTube('url, use_oauth=True,
            allow_oauth_cache=True)

            title = yt.title

        except:
            title = "None"
            title
    };
    let title: String = c.get("title");
    Ok(title)
}