use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

pub const PREVIEW_TEXT_LIMIT: usize = 512;
pub const FULL_PREVIEW_TEXT_LIMIT: usize = 8 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ItemId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TimestampMillis(pub u128);

impl TimestampMillis {
    #[must_use]
    pub fn now() -> Self {
        Self::from_system_time(SystemTime::now())
    }

    #[must_use]
    pub fn from_system_time(value: SystemTime) -> Self {
        let elapsed = value.duration_since(UNIX_EPOCH).unwrap_or_default();
        Self(elapsed.as_millis())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClipboardSource {
    pub application_name: Option<String>,
    pub bundle_or_process_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClipboardContent {
    Text {
        text: String,
    },
    RichText {
        plain_text: String,
        rtf: Option<Vec<u8>>,
        html: Option<String>,
    },
    Html {
        plain_text: String,
        html: String,
    },
    Image {
        bytes: Vec<u8>,
        mime_type: String,
        width: Option<u32>,
        height: Option<u32>,
    },
    Files {
        paths: Vec<String>,
    },
    Url {
        url: String,
        title: Option<String>,
    },
    Unsupported {
        format_names: Vec<String>,
    },
}

impl ClipboardContent {
    #[must_use]
    pub fn kind(&self) -> ContentKind {
        match self {
            Self::Text { .. } => ContentKind::Text,
            Self::RichText { .. } => ContentKind::RichText,
            Self::Html { .. } => ContentKind::Html,
            Self::Image { .. } => ContentKind::Image,
            Self::Files { .. } => ContentKind::Files,
            Self::Url { .. } => ContentKind::Url,
            Self::Unsupported { .. } => ContentKind::Unsupported,
        }
    }

    #[must_use]
    pub fn preview_text(&self) -> String {
        match self {
            Self::Text { text } => text.clone(),
            Self::RichText { plain_text, .. } | Self::Html { plain_text, .. } => plain_text.clone(),
            Self::Image {
                mime_type,
                width,
                height,
                ..
            } => match (width, height) {
                (Some(width), Some(height)) => format!("{mime_type}, {width}x{height}"),
                _ => mime_type.clone(),
            },
            Self::Files { paths } => paths.join("\n"),
            Self::Url { url, title } => title
                .as_ref()
                .map_or_else(|| url.clone(), |title| format!("{title}\n{url}")),
            Self::Unsupported { format_names } => {
                if format_names.is_empty() {
                    String::new()
                } else {
                    format_names.join(", ")
                }
            }
        }
    }

    #[must_use]
    pub fn bounded_preview_text(&self) -> BoundedText {
        BoundedText::new(self.preview_text(), PREVIEW_TEXT_LIMIT)
    }

    #[must_use]
    pub fn full_preview_text(&self) -> String {
        match self {
            Self::RichText {
                plain_text, html, ..
            } => html.clone().unwrap_or_else(|| plain_text.clone()),
            Self::Html { html, .. } => html.clone(),
            _ => self.preview_text(),
        }
    }

    #[must_use]
    pub fn bounded_full_preview_text(&self) -> BoundedText {
        BoundedText::new(self.full_preview_text(), FULL_PREVIEW_TEXT_LIMIT)
    }

    #[must_use]
    pub fn plain_text_for_paste(&self) -> String {
        match self {
            Self::Text { text } => text.clone(),
            Self::RichText { plain_text, .. }
            | Self::Html { plain_text, .. }
            | Self::Url {
                url: plain_text, ..
            } => plain_text.clone(),
            Self::Files { paths } => paths.join("\n"),
            Self::Image { .. } => self.preview_text(),
            Self::Unsupported { .. } => String::new(),
        }
    }

    #[must_use]
    pub fn approximate_size_bytes(&self) -> usize {
        match self {
            Self::Text { text } => text.len(),
            Self::RichText {
                plain_text,
                rtf,
                html,
            } => {
                plain_text.len()
                    + rtf.as_ref().map_or(0, Vec::len)
                    + html.as_ref().map_or(0, String::len)
            }
            Self::Html { plain_text, html } => plain_text.len() + html.len(),
            Self::Image { bytes, .. } => bytes.len(),
            Self::Files { paths }
            | Self::Unsupported {
                format_names: paths,
            } => paths.iter().map(String::len).sum(),
            Self::Url { url, title } => url.len() + title.as_ref().map_or(0, String::len),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundedText {
    pub text: String,
    pub truncated: bool,
}

impl BoundedText {
    #[must_use]
    pub fn new(value: String, max_chars: usize) -> Self {
        if value.chars().count() <= max_chars {
            return Self {
                text: value,
                truncated: false,
            };
        }

        let text = if max_chars <= 3 {
            value.chars().take(max_chars).collect::<String>()
        } else {
            let mut text = value.chars().take(max_chars - 3).collect::<String>();
            text.push_str("...");
            text
        };

        Self {
            text,
            truncated: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContentKind {
    Text,
    RichText,
    Html,
    Image,
    Files,
    Url,
    Unsupported,
}

impl ContentKind {
    #[must_use]
    pub fn localization_key(self) -> &'static str {
        match self {
            Self::Text => "content.kind.text",
            Self::RichText => "content.kind.rich_text",
            Self::Html => "content.kind.html",
            Self::Image => "content.kind.image",
            Self::Files => "content.kind.files",
            Self::Url => "content.kind.url",
            Self::Unsupported => "content.kind.unsupported",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClipboardItem {
    pub id: ItemId,
    pub created_at: TimestampMillis,
    pub last_used_at: TimestampMillis,
    pub content: ClipboardContent,
    pub content_kind: ContentKind,
    pub content_kind_key: String,
    pub preview_text: String,
    #[serde(default)]
    pub preview_truncated: bool,
    pub pinned: bool,
    pub pinned_shortcut: Option<char>,
    pub source: Option<ClipboardSource>,
}

impl ClipboardItem {
    #[must_use]
    pub fn new(
        id: ItemId,
        content: ClipboardContent,
        created_at: TimestampMillis,
        source: Option<ClipboardSource>,
    ) -> Self {
        let content_kind = content.kind();
        let bounded_preview = content.bounded_preview_text();

        Self {
            id,
            created_at,
            last_used_at: created_at,
            content,
            content_kind,
            content_kind_key: content_kind.localization_key().to_string(),
            preview_text: bounded_preview.text,
            preview_truncated: bounded_preview.truncated,
            pinned: false,
            pinned_shortcut: None,
            source,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_previews_for_supported_clipboard_content_types() {
        let cases = [
            (
                ClipboardContent::Text {
                    text: "plain".to_string(),
                },
                "plain",
            ),
            (
                ClipboardContent::RichText {
                    plain_text: "rich".to_string(),
                    rtf: Some(vec![1, 2, 3]),
                    html: None,
                },
                "rich",
            ),
            (
                ClipboardContent::Html {
                    plain_text: "html".to_string(),
                    html: "<strong>html</strong>".to_string(),
                },
                "html",
            ),
            (
                ClipboardContent::Files {
                    paths: vec!["/tmp/a.txt".to_string(), "/tmp/b.txt".to_string()],
                },
                "/tmp/a.txt\n/tmp/b.txt",
            ),
            (
                ClipboardContent::Url {
                    url: "https://example.com".to_string(),
                    title: Some("Example".to_string()),
                },
                "Example\nhttps://example.com",
            ),
            (
                ClipboardContent::Unsupported {
                    format_names: vec!["custom/type".to_string()],
                },
                "custom/type",
            ),
        ];

        for (content, expected_preview) in cases {
            assert_eq!(content.preview_text(), expected_preview);
        }
    }

    #[test]
    fn generates_image_preview_with_dimensions() {
        let content = ClipboardContent::Image {
            bytes: vec![0, 1, 2],
            mime_type: "image/png".to_string(),
            width: Some(10),
            height: Some(20),
        };

        assert_eq!(content.preview_text(), "image/png, 10x20");
    }

    #[test]
    fn rich_content_full_preview_prefers_html() {
        let content = ClipboardContent::RichText {
            plain_text: "plain".to_string(),
            rtf: None,
            html: Some("<p>plain</p>".to_string()),
        };

        assert_eq!(content.full_preview_text(), "<p>plain</p>");
    }

    #[test]
    fn plain_text_for_paste_preserves_whitespace() {
        let content = ClipboardContent::Text {
            text: " one\r\n  two\n".to_string(),
        };

        assert_eq!(content.plain_text_for_paste(), " one\r\n  two\n");
    }

    #[test]
    fn bounds_clipboard_item_preview_text_without_changing_content() {
        let long_text = "a".repeat(PREVIEW_TEXT_LIMIT + 100);
        let content = ClipboardContent::Text {
            text: long_text.clone(),
        };
        let item = ClipboardItem::new(ItemId(1), content, TimestampMillis(1), None);

        assert!(item.preview_text.len() < long_text.len());
        assert!(item.preview_text.ends_with("..."));
        assert!(item.preview_truncated);
        assert_eq!(item.content, ClipboardContent::Text { text: long_text });
    }

    #[test]
    fn reports_bounded_full_preview_truncation() {
        let content = ClipboardContent::Html {
            plain_text: "plain".to_string(),
            html: "<p>".to_string() + &"a".repeat(FULL_PREVIEW_TEXT_LIMIT + 100),
        };

        let preview = content.bounded_full_preview_text();

        assert!(preview.truncated);
        assert!(preview.text.ends_with("..."));
    }
}
