use std::time::Duration;

pub struct LyricsContext {
    pub current_lyrics: Option<Lyrics>,
}

impl LyricsContext {
    pub fn new() -> LyricsContext {
        LyricsContext {
            current_lyrics: None,
        }
    }
}

pub struct Lyrics {
    pub ti: Option<String>,
    pub ar: Option<String>,
    pub al: Option<String>,
    pub offset: Option<i64>,
    pub by: Option<String>,
    pub ve: Option<String>,
    pub au: Option<String>,
    pub re: Option<String>,
    pub lines: Vec<LyricLine>,
    pub raw: Option<String>,
}

pub struct LyricLine {
    pub timestamp: Option<Duration>,
    pub text: String,
}

impl Lyrics {
    pub fn new() -> Lyrics {
        Lyrics {
            ti: None,
            ar: None,
            al: None,
            offset: None,
            by: None,
            ve: None,
            au: None,
            re: None,
            lines: vec![],
            raw: None,
        }
    }

    pub fn from_str(s: String) -> Lyrics {
        let mut lyrics = Lyrics::new();
        lyrics.raw = Some(s.clone());
        s.lines().for_each(|line| {
            let line = line.trim();
            if line.starts_with("[ti:") && line.ends_with(']') {
                let content = &line[4..line.len() - 1];
                if !content.is_empty() {
                    lyrics.ti = Some(content.to_string());
                }
            } else if line.starts_with("[ar:") && line.ends_with(']') {
                let content = &line[4..line.len() - 1];
                if !content.is_empty() {
                    lyrics.ar = Some(content.to_string());
                }
            } else if line.starts_with("[al:") && line.ends_with(']') {
                let content = &line[4..line.len() - 1];
                if !content.is_empty() {
                    lyrics.al = Some(content.to_string());
                }
            } else if line.starts_with("[offset:") && line.ends_with(']') {
                let content = &line[8..line.len() - 1];
                if let Ok(value) = content.parse::<i64>() {
                    lyrics.offset = Some(value);
                }
            } else if line.starts_with("[by:") && line.ends_with(']') {
                let content = &line[4..line.len() - 1];
                if !content.is_empty() {
                    lyrics.by = Some(content.to_string());
                }
            } else if line.starts_with("[ve:") && line.ends_with(']') {
                let content = &line[4..line.len() - 1];
                if !content.is_empty() {
                    lyrics.ve = Some(content.to_string());
                }
            } else if line.starts_with("[au:") && line.ends_with(']') {
                let content = &line[4..line.len() - 1];
                if !content.is_empty() {
                    lyrics.au = Some(content.to_string());
                }
            } else if line.starts_with("[re:") && line.ends_with(']') {
                let content = &line[4..line.len() - 1];
                if !content.is_empty() {
                    lyrics.re = Some(content.to_string());
                }
            } else if line.starts_with('[') && line.contains(']') {
                lyrics.lines.push(LyricLine::from_str(line.to_string()));
            }
        });
        lyrics
    }
}

impl LyricLine {
    pub fn new(timestamp: Option<Duration>, text: String) -> LyricLine {
        LyricLine { timestamp, text }
    }

    pub fn from_str(line: String) -> LyricLine {
        let parts: Vec<&str> = line.split(']').collect();
        if parts.len() < 2 {
            return LyricLine {
                timestamp: None,
                text: line,
            };
        }
        let timestamp_str = parts[0].trim_start_matches('[');
        let text = parts[1..].join("]").trim().to_string();
        let timestamp = parse_timestamp(timestamp_str);
        LyricLine { timestamp, text }
    }
}

pub fn parse_timestamp(s: &str) -> Option<Duration> {
    let parts = s.split(&[':', '.'][..]).collect::<Vec<&str>>();
    if parts.len() < 2 {
        return None;
    }
    let minutes = parts[0].parse::<u64>().ok()?;
    let seconds = parts[1].parse::<u64>().ok()?;
    let millis = if parts.len() > 2 {
        if parts[2].len() == 2 {
            parts[2].parse::<u64>().ok()? * 10
        } else {
            parts[2].parse::<u64>().ok()?
        }
    } else {
        0
    };
    Some(Duration::from_millis(
        minutes * 60 * 1000 + seconds * 1000 + millis,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_lyric_line() {
        let line = "[00:12.34] Hello world".to_string();
        let lyric_line = LyricLine::from_str(line);
        assert_eq!(lyric_line.timestamp, Some(Duration::from_millis(12340)));
        assert_eq!(lyric_line.text, "Hello world");
    }

    #[test]
    fn test_parse_lyric() {
        let lyric_str = "[ti:Test Song]
[ar:Test Artist]
[al:Test Album]
[00:12.34] Hello world
[00:45.67] This is a test lyric line.";
        let lrc = Lyrics::from_str(String::from(lyric_str));
        assert_eq!(lrc.ti, Some("Test Song".to_string()));
        assert_eq!(lrc.ar, Some("Test Artist".to_string()));
        assert_eq!(lrc.al, Some("Test Album".to_string()));
        assert_eq!(lrc.lines.len(), 2);
        assert_eq!(lrc.lines[0].timestamp, Some(Duration::from_millis(12340)));
        assert_eq!(lrc.lines[0].text, "Hello world");
        assert_eq!(lrc.lines[1].timestamp, Some(Duration::from_millis(45670)));
        assert_eq!(lrc.lines[1].text, "This is a test lyric line.");
    }
}
