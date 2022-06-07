use crate::mime::Mime;
use std::borrow::Cow;

macro_rules! utf8_mime_const {
    ($name:ident, $desc:expr, $base:expr, $sub:expr) => {
        mime_const!(
            with_params,
            $name,
            $desc,
            $base,
            $sub,
            true,
            ";charset=utf-8"
        );
    };
}
macro_rules! mime_const {
    ($name:ident, $desc:expr, $base:expr, $sub:expr) => {
        mime_const!(with_params, $name, $desc, $base, $sub, false, "");
    };

    (with_params, $name:ident, $desc:expr, $base:expr, $sub:expr, $is_utf8:expr, $doccomment:expr) => {
        mime_const!(
            doc_expanded,
            $name,
            $desc,
            $base,
            $sub,
            $is_utf8,
            concat!(
                "Content-Type for ",
                $desc,
                ".\n\n# Mime Type\n\n```text\n",
                $base,
                "/",
                $sub,
                $doccomment,
                "\n```"
            )
        );
    };

    (doc_expanded, $name:ident, $desc:expr, $base:expr, $sub:expr, $is_utf8:expr, $doccomment:expr) => {
        #[doc = $doccomment]
        pub const $name: Mime = Mime {
            essence: Cow::Borrowed(concat!($base, "/", $sub)),
            basetype: Cow::Borrowed($base),
            subtype: Cow::Borrowed($sub),
            is_utf8: $is_utf8,
            params: vec![],
        };
    };
}

utf8_mime_const!(JAVASCRIPT, "JavaScript", "text", "javascript");
utf8_mime_const!(CSS, "CSS", "text", "css");
utf8_mime_const!(HTML, "HTML", "text", "html");
utf8_mime_const!(PLAIN, "Plain text", "text", "plain");
utf8_mime_const!(XML, "XML", "application", "xml");
utf8_mime_const!(RSS, "RSS Feed", "application", "rss+xml");
utf8_mime_const!(ATOM, "Atom Feed", "application", "atom+xml");
mime_const!(ANY, "matching anything", "*", "*");
mime_const!(JSON, "JSON", "application", "json");
mime_const!(SSE, "Server Sent Events", "text", "event-stream");
mime_const!(BYTE_STREAM, "byte streams", "application", "octet-stream");
mime_const!(FORM, "forms", "application", "x-www-form-urlencoded");
mime_const!(MULTIPART_FORM, "multipart forms", "multipart", "form-data");
mime_const!(WASM, "webassembly", "application", "wasm");

// Images
// https://www.iana.org/assignments/media-types/media-types.xhtml#image
mime_const!(BMP, "BMP images", "image", "bmp");
mime_const!(JPEG, "JPEG images", "image", "jpeg");
mime_const!(PNG, "PNG images", "image", "png");
mime_const!(SVG, "SVG", "image", "svg+xml");
mime_const!(WEBP, "WebP images", "image", "webp");

// Audio
// https://www.iana.org/assignments/media-types/media-types.xhtml#audio
mime_const!(MIDI, "MIDI audio", "audio", "midi");
mime_const!(MP3, "MPEG audio layer 3", "audio", "mpeg");
mime_const!(OGG, "Ogg vorbis audio", "audio", "ogg");
mime_const!(OPUS, "Opus audio", "audio", "opus");
mime_const!(M4A, "MPEG audio layer 4", "audio", "mp4");

// Video
// https://www.iana.org/assignments/media-types/media-types.xhtml#video
mime_const!(MP4, "MPEG video layer 4", "video", "mp4");
mime_const!(MPEG, "MPEG video", "video", "mpeg");
mime_const!(WEBM, "WebM video", "video", "webm");
mime_const!(AVI, "Microsoft AVI video", "video", "x-msvideo");
// There are multiple `.ico` mime types known, but `image/x-icon`
// is what most browser use. See:
// https://en.wikipedia.org/wiki/ICO_%28file_format%29#MIME_type
mime_const!(ICO, "ICO icons", "image", "x-icon");

// Fonts
// https://www.iana.org/assignments/media-types/media-types.xhtml#font
mime_const!(OTF, "OTF", "font", "otf");
mime_const!(TTF, "TTF", "font", "ttf");
mime_const!(WOFF, "WOFF", "font", "woff");
mime_const!(WOFF2, "WOFF2", "font", "woff2");

// Archives
mime_const!(ZIP, "Zip archive", "application", "zip");
mime_const!(SEVENZIP, "7Zip archive", "application", "x-7z-compressed");
