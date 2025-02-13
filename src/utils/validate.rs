use regex::Regex;
use warp::http::StatusCode;
use warp::reject::{custom, Rejection};
use warp::reply::Reply;

#[derive(Debug)]
pub struct InvalidLink;

impl warp::reject::Reject for InvalidLink {}

/// Validate the link to prevent exploitation
pub fn validate_link(link: String) -> Result<String, Rejection> {
    let url_regex = Regex::new(r"^(https?|ftp)://[^\s/$.?#].[^\s]*$").unwrap();

    // let banned_chars = vec![
    //     " ",
    //     "?",
    //     "\"",
    //     "&",
    //     "%",
    //     "javascript:",
    //     "data:",
    //     "<",
    //     ">",
    //     "'",
    //     "#",
    //     "(",
    //     ")",
    //     "{",
    //     "}",
    //     "[",
    //     "]",
    //     ";",
    //     ":",
    //     "/",
    //     "\\",
    //     ".",
    //     "..",
    //     "eval",
    //     "alert",
    //     "confirm",
    //     "prompt",
    //     "cookie",
    //     "file:",
    //     "ftp:",
    //     "shell:",
    //     "php:",
    //     "telnet:",
    //     "cmd:",
    //     "input",
    //     "onload",
    //     "onerror",
    //     "onmouseover",
    //     "onfocus",
    //     "onkeydown",
    //     "onkeyup",
    //     "onchange",
    //     "iframe",
    //     "form",
    //     "button",
    //     "input",
    //     "select",
    //     "textarea",
    //     "option",
    //     "script",
    //     "object",
    //     "embed",
    //     "applet",
    //     "base64",
    //     "url",
    //     "src",
    //     "action",
    //     "method",
    //     "upload",
    //     "download",
    //     "srcset",
    //     "object",
    //     "frame",
    //     "frameset",
    //     "frameborder",
    //     "title",
    //     "sandbox",
    //     "allow",
    //     "scriptlet",
    //     "eval(",
    //     "document",
    //     "XMLHttpRequest",
    //     "ws://",
    //     "wss://",
    //     "file://",
    //     "tel:",
    //     "ftp://",
    //     "chrome://",
    //     "about://",
    //     "data://",
    //     "vbs://",
    //     "mshta:",
    //     "msdn:",
    //     "moz-extension://",
    //     "javascript:alert",
    //     "javascript:eval",
    //     "javascript:void(0)",
    //     "expression",
    //     "vbscript",
    //     "cmd.exe",
    //     "powershell",
    //     "cmd",
    //     "powershell.exe",
    //     "bash",
    //     "sh",
    //     "exec",
    //     "process",
    //     "System",
    //     "os.system",
    //     "Runtime",
    //     "spawn",
    //     "child_process",
    //     "require",
    //     "require('child_process')",
    //     "Function",
    //     "setTimeout",
    //     "setInterval",
    //     "clearTimeout",
    //     "clearInterval",
    //     "Date",
    //     "Object",
    //     "Function",
    //     "RegExp",
    //     "eval()",
    //     "console",
    //     "console.log",
    //     "console.dir",
    //     "console.error",
    //     "console.trace",
    //     "console.info",
    //     "window",
    //     "document.location",
    //     "document.write",
    //     "document.writeln",
    //     "document.body.innerHTML",
    //     "document.cookie",
    //     "setImmediate",
    //     "clearImmediate",
    //     "window.alert",
    //     "window.confirm",
    //     "window.prompt",
    //     "window.document",
    //     "window.location",
    //     "window.location.replace",
    //     "window.location.href",
    //     "window.opener",
    //     "window.open",
    //     "window.showModalDialog",
    //     "setInterval(",
    //     "setTimeout(",
    //     "eval",
    //     "encodeURIComponent",
    //     "decodeURIComponent",
    //     "encodeURI",
    //     "decodeURI",
    //     "escape",
    //     "unescape",
    //     "btoa",
    //     "atob",
    //     "unescape",
    //     "unescape(",
    //     "escape(",
    //     "new Function",
    //     "function()",
    //     "class",
    //     "type",
    //     "undefined",
    //     "constructor",
    //     "prototype",
    //     "instanceof",
    //     "Symbol",
    //     "BigInt",
    //     "NaN",
    //     "Infinity",
    //     "isNaN",
    //     "isFinite",
    //     "Array",
    //     "Map",
    //     "Set",
    //     "WeakMap",
    //     "WeakSet",
    //     "BigInt",
    //     "async",
    //     "await",
    //     "finally",
    //     "async function",
    //     "Promise",
    //     "async await",
    //     "process.env",
    //     "setInterval",
    //     "setTimeout",
    //     "clearInterval",
    //     "clearTimeout",
    //     "location.hash",
    //     "window.history.pushState",
    //     "history.pushState",
    //     "window.history.replaceState",
    //     "history.replaceState",
    //     "window.location.href",
    //     "window.location.assign",
    //     "window.location.replace",
    //     "window.location.reload",
    //     "window.location.origin",
    //     "history.forward",
    //     "history.back",
    //     "onhashchange",
    //     "popstate",
    //     "onpopstate",
    //     "HTML5",
    //     "WebSocket",
    //     "WebRTC",
    //     "WebGL",
    //     "LocalStorage",
    //     "SessionStorage",
    //     "CacheStorage",
    //     "IndexedDB",
    //     "WebSQL",
    //     "FileSystem",
    //     "Notification",
    //     "FileReader",
    //     "fetch",
    //     "ServiceWorker",
    //     "caches",
    //     "XMLHttpRequest",
    //     "formData",
    //     "FileReader",
    //     "Blob",
    //     "ArrayBuffer",
    //     "SharedWorker",
    //     "Worker",
    //     "postMessage",
    //     "importScripts",
    //     "WebWorker",
    //     "setTimeout",
    //     "clearTimeout",
    //     "setInterval",
    //     "clearInterval",
    //     "onmessage",
    //     "postMessage",
    //     "importScripts",
    //     "console.warn",
    //     "console.debug",
    //     "console.error",
    //     "console.info",
    //     "console.trace",
    //     "console.group",
    //     "console.groupEnd",
    //     "console.assert",
    //     "console.time",
    //     "console.timeEnd",
    //     "console.count",
    //     "console.countReset",
    //     "console.table",
    //     "console.dirxml",
    //     "console.profile",
    //     "console.timeStamp",
    //     "console.memory",
    //     "console.clear",
    //     "console.dir",
    //     "console.log.bind",
    //     "console.debug.bind",
    //     "console.warn.bind",
    //     "console.error.bind",
    //     "console.info.bind",
    //     "console.trace.bind",
    //     "console.group.bind",
    //     "console.groupEnd.bind",
    //     "console.assert.bind",
    //     "console.time.bind",
    //     "console.timeEnd.bind",
    //     "console.count.bind",
    //     "console.countReset.bind",
    //     "console.table.bind",
    //     "console.dirxml.bind",
    //     "console.profile.bind",
    //     "console.timeStamp.bind",
    //     "console.memory.bind",
    //     "console.clear.bind",
    //     "console.dir.bind",
    //     "console.log.call",
    //     "console.debug.call",
    //     "console.warn.call",
    //     "console.error.call",
    //     "console.info.call",
    //     "console.trace.call",
    //     "console.group.call",
    //     "console.groupEnd.call",
    //     "console.assert.call",
    //     "console.time.call",
    //     "console.timeEnd.call",
    //     "console.count.call",
    //     "console.countReset.call",
    //     "console.table.call",
    //     "console.dirxml.call",
    //     "console.profile.call",
    //     "console.timeStamp.call",
    //     "console.memory.call",
    //     "console.clear.call",
    //     "console.dir.call",
    //     "console.log.apply",
    //     "console.debug.apply",
    //     "console.warn.apply",
    //     "console.error.apply",
    //     "console.info.apply",
    //     "console.trace.apply",
    //     "console.group.apply",
    //     "console.groupEnd.apply",
    //     "console.assert.apply",
    //     "console.time.apply",
    //     "console.timeEnd.apply",
    //     "console.count.apply",
    //     "console.countReset.apply",
    //     "console.table.apply",
    //     "console.dirxml.apply",
    //     "console.profile.apply",
    //     "console.timeStamp.apply",
    //     "console.memory.apply",
    //     "console.clear.apply",
    //     "console.dir.apply",
    //     "new Function",
    //     "eval()",
    // ];

    let banned_chars = vec![" ", "?", "\"", "&", "%", "javascript:", "data:"];

    if banned_chars.iter().any(|c| link.contains(c)) || !url_regex.is_match(&link) {
        return Err(custom(InvalidLink));
    }

    Ok(link)
}

/// Error handler
pub async fn error_handler(err: Rejection) -> Result<impl Reply, std::convert::Infallible> {
    if err.find::<InvalidLink>().is_some() {
        Ok(warp::reply::with_status(
            "Invalid link",
            StatusCode::BAD_REQUEST,
        ))
    } else if err
        .find::<crate::utils::rate_limit::TooManyRequests>()
        .is_some()
    {
        Ok(warp::reply::with_status(
            "Too many requests, slow down!",
            StatusCode::TOO_MANY_REQUESTS,
        ))
    } else if err.find::<crate::db::db::DbError>().is_some() {
        Ok(warp::reply::with_status(
            "Database error",
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Internal Server Error",
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}
