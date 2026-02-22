use actix_web::{http::header, middleware::DefaultHeaders};

pub fn default_headers() -> DefaultHeaders {
    DefaultHeaders::new()
        // Prevent MIME type sniffing
        .add((header::X_CONTENT_TYPE_OPTIONS, "nosniff"))
        // Prevent clickjacking by disallowing iframe embedding
        .add((header::X_FRAME_OPTIONS, "DENY"))
        // Enable XSS filter and block rendering on attack detection (legacy browsers)
        .add((header::X_XSS_PROTECTION, "1; mode=block"))
        // Enforce HTTPS for 1 year, including subdomains
        .add((
            header::STRICT_TRANSPORT_SECURITY,
            "max-age=31536000; includeSubDomains",
        ))
        // Restrict resource loading to none; disallow iframe embedding via CSP
        .add((
            header::CONTENT_SECURITY_POLICY,
            "default-src 'none'; frame-ancestors 'none'",
        ))
        // Do not send referrer information to other origins
        .add((header::REFERRER_POLICY, "no-referrer"))
        // Disable all caching (HTTP/1.1)
        .add((header::CACHE_CONTROL, "no-store, no-cache, must-revalidate"))
        // Disable caching for HTTP/1.0 proxies
        .add((header::PRAGMA, "no-cache"))
        // Expire immediately for HTTP/1.0 compatibility
        .add((header::EXPIRES, "0"))
}
