# Server Impl

First Verify In The Root Of The Project There's A File Called **Proj.toml**

```toml
[config]
PORT = 8080
VIDEOS_FOLDER = "videos"
```

This controls where videos are stored, the port and the such

## Running

To run the server run

```sh
cargo run --release
```

## Requests

```
POST /upload
Content-Type: multipart/mixed; boundary=whatever-boundary-youre-using
```

Expects a multipart request, the first part should have the content-type set to application/json

```json
{
  "upload_at": "2026-12-01T00:00:00Z",
  "video_platform": "YoutubeShorts",
  "video_title": "My Video Title",
  "description": "My Video Description",
  "hashtags": ["#example", "#other-hash-tag"],
  "additional": {
    "custom-key": "custom data that needs to be used by scripts in the future"
  }
}
```

The second request should be the video as binary data and the content-type should be the content-type of the video for example:

```
Content-Type: video/mp4

whatever-bytes-of-the-video-your-sending
```
