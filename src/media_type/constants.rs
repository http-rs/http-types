use super::ParamKind;
use crate::MediaType;

macro_rules! utf8_media_type_const {
    ($name:ident, $desc:expr, $base:expr, $sub:expr) => {
        media_type_const!(
            with_params,
            $name,
            $desc,
            $base,
            $sub,
            Some(ParamKind::Utf8),
            ";charset=utf-8"
        );
    };
}
macro_rules! media_type_const {
    ($name:ident, $desc:expr, $base:expr, $sub:expr) => {
        media_type_const!(with_params, $name, $desc, $base, $sub, None, "");
    };

    (with_params, $name:ident, $desc:expr, $base:expr, $sub:expr, $params:expr, $doccomment:expr) => {
        media_type_const!(doc_expanded, $name, $desc, $base, $sub, $params,
             concat!(
                "Content-Type for ",
                $desc,
                ".\n\n# media type\n\n```text\n",
                $base, "/", $sub, $doccomment, "\n```")
        );
    };

    (doc_expanded, $name:ident, $desc:expr, $base:expr, $sub:expr, $params:expr, $doccomment:expr) => {
        #[doc = $doccomment]
        pub const $name: MediaType = MediaType {
            essence: String::new(),
            basetype: String::new(),
            subtype: String::new(),
            params: $params,
            static_essence: Some(concat!($base, "/", $sub)),
            static_basetype: Some($base),
            static_subtype: Some($sub),
        };
    };
}

utf8_media_type_const!(JAVASCRIPT, "JavaScript", "application", "javascript");
utf8_media_type_const!(CSS, "CSS", "text", "css");
utf8_media_type_const!(HTML, "HTML", "text", "html");
utf8_media_type_const!(PLAIN, "Plain text", "text", "plain");
utf8_media_type_const!(XML, "XML", "application", "xml");
media_type_const!(ANY, "matching anything", "*", "*");
media_type_const!(JSON, "JSON", "application", "json");
media_type_const!(SVG, "SVG", "image", "svg+xml");
media_type_const!(PNG, "PNG images", "image", "png");
media_type_const!(JPEG, "JPEG images", "image", "jpeg");
media_type_const!(SSE, "Server Sent Events", "text", "event-stream");
media_type_const!(BYTE_STREAM, "byte streams", "application", "octet-stream");
media_type_const!(FORM, "forms", "application", "x-www-form-urlencoded");
media_type_const!(MULTIPART_FORM, "multipart forms", "multipart", "form-data");
media_type_const!(WASM, "webassembly", "application", "wasm");
// There are multiple `.ico` media types known, but `image/x-icon`
// is what most browser use. See:
// https://en.wikipedia.org/wiki/ICO_%28file_format%29#MIME_type
media_type_const!(ICO, "ICO icons", "image", "x-icon");
