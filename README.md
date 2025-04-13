# YouTube Channel Analyzer
[OLD VERSION](https://github.com/krasimir-zhelezov/python-youtube-channel-analyzer/)

A Rust tool that fetches and analyzes video data from YouTube channels using the [YouTube Data API v3](https://developers.google.com/youtube/). Currently supports analysis of up to 50 videos per channel.

## Features

- Fetch video statistics by channel username
- Analyze engagement metrics
- Export data for further processing
- (Future) Data visualization capabilities

## Prerequisites

- Rust
- YouTube Data API v3 key

## Installation

1. Clone this repository:
   ```bash
   git clone https://github.com/krasimir-zhelezov/youtube-channel-analyzer.git
   cd youtube-channel-analyzer
   ```

2. Create a .env file in the project root and add your YouTube API key:
    ```env
    API_KEY=your_youtube_api_key_here
    ```

## Usage

```bash
cargo run
```

## TODO
* Rewrite implementation in Rust for performance
* Develop GUI interface
* Add data visualization capabilities
* Increase video limit beyond 50
* Add CSV/Excel export functionality

## Limitations
* Currently limited to analyzing 50 most recent videos per channel

## License
This project is licensed under the **GNU General Public License**.  
See [LICENSE](https://github.com/krasimir-zhelezov/youtube-channel-analyzer?tab=License-1-ov-file) for the full text.