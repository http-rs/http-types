/// HTTP response status codes.
///
/// HTTP response status codes indicate whether a specific HTTP request has been successfully
/// completed. Responses are grouped in five classes:
#[derive(Debug)]
pub enum StatusCode {
    // 100-199
    /// This interim response indicates that everything so far is OK and that the client should
    /// continue the request, or ignore the response if the request is already finished.
    Continue,

    ///This code is sent in response to an Upgrade request header from the client, and indicates
    ///the protocol the server is switching to.
    SwitchingProtocol,

    /// This code indicates that the server has received and is processing the request, but no response is available yet.
    Processing,

    /// This status code is primarily intended to be used with the Link header, letting the user
    /// agent start preloading resources while the server prepares a response.
    EarlyHints,

    // 200-299
    /// 200 The request has succeeded
    Ok,

    /// 201 The request has succeeded and a new resource has been created as a result. This is typically the response sent after POST requests, or some PUT requests.
    Created,

    /// 202 The request has been received but not yet acted upon. It is noncommittal, since there is no way in HTTP to later send an asynchronous response indicating the outcome of the request. It is intended for cases where another process or server handles the request, or for batch processing.
    Accepted,

    /// 203 This response code means the returned meta-information is not exactly the same as is available from the origin server, but is collected from a local or a third-party copy. This is mostly used for mirrors or backups of another resource. Except for that specific case, the "200 OK" response is preferred to this status.
    NonAuthoritativeInformation,

    /// 204 There is no content to send for this request, but the headers may be useful. The user-agent may update its cached headers for this resource with the new ones.
    NoContent,

    /// 205 Tells the user-agent to reset the document which sent this request.
    ResetContent,

    /// 206 This response code is used when the Range header is sent from the client to request only part of a resource.
    PartialContent,

    /// 207 Conveys information about multiple resources, for situations where multiple status codes might be appropriate.
    MultiStatus,

    /// 208 Used inside a <dav:propstat> response element to avoid repeatedly enumerating the internal members of multiple bindings to the same collection.
    AlreadySupported,

    /// 226 The server has fulfilled a GET request for the resource, and the response is a representation of the result of one or more instance-manipulations applied to the current instance.
    ImUsed,

    // 300-399
    /// The request has more than one possible response. The user-agent or user should choose one of them. (There is no standardized way of choosing one of the responses, but HTML links to the possibilities are recommended so the user can pick.)
    MultipleChoice,
    /// The URL of the requested resource has been changed permanently. The new URL is given in the response.
    MovedPermanently,
    // 400-499
    // 500-599
}
