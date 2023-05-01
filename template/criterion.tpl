extern crate criterion;

use criterion::{criterion_group, criterion_main, Criterion};
use {name}::run;

fn {name}_benchmark(c: &mut Criterion) {
    c.bench_function("{name}", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                match run() {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Error running {name}: {:?}", e);
                    }
                }
            }
        })
    });
}

criterion_group!(benches, {name}_benchmark);
criterion_main!(benches);
HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"b5adc93c54eec77d9668a8dd6dd02b7bbd65a0bf4bdc5aed794aaf5feb382ccf"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 210E:1AD0:1BCCB81:1CE0F4C:644F7674
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */257
date: Mon, 01 May 2023 08:21:09 GMT
via: 1.1 varnish
x-served-by: cache-lhr7337-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929269.402397,VS0,VE186
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 1b25103bcb4b325c80b28824c8aa2497dd1db1ac
expires: Mon, 01 May 2023 08:26:09 GMT
source-age: 0
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"b5adc93c54eec77d9668a8dd6dd02b7bbd65a0bf4bdc5aed794aaf5feb382ccf"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 210E:1AD0:1BCCB81:1CE0F4C:644F7674
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */257
date: Mon, 01 May 2023 08:21:09 GMT
via: 1.1 varnish
x-served-by: cache-lhr7389-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929269.402465,VS0,VE186
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: fde4a2af1f49aa17089bde2d6bc763e64a163c2c
expires: Mon, 01 May 2023 08:26:09 GMT
source-age: 0
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"b5adc93c54eec77d9668a8dd6dd02b7bbd65a0bf4bdc5aed794aaf5feb382ccf"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 210E:1AD0:1BCCB81:1CE0F4C:644F7674
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */257
date: Mon, 01 May 2023 08:21:09 GMT
via: 1.1 varnish
x-served-by: cache-lhr7324-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929269.402398,VS0,VE186
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 6b9733a3692df79fe7ecdece21de62f444f6f7b2
expires: Mon, 01 May 2023 08:26:09 GMT
source-age: 0
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"b5adc93c54eec77d9668a8dd6dd02b7bbd65a0bf4bdc5aed794aaf5feb382ccf"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 210E:1AD0:1BCCB81:1CE0F4C:644F7674
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */257
date: Mon, 01 May 2023 08:21:09 GMT
via: 1.1 varnish
x-served-by: cache-lhr7382-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929269.402711,VS0,VE186
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: b704ac91f4b7410b78cdc7bc2965dbc74560d6ad
expires: Mon, 01 May 2023 08:26:09 GMT
source-age: 0
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"b5adc93c54eec77d9668a8dd6dd02b7bbd65a0bf4bdc5aed794aaf5feb382ccf"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 210E:1AD0:1BCCB81:1CE0F4C:644F7674
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */257
date: Mon, 01 May 2023 08:21:09 GMT
via: 1.1 varnish
x-served-by: cache-lhr7380-LHR
x-cache: MISS
x-cache-hits: 0
x-timer: S1682929269.402444,VS0,VE186
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 0519d8d4f4fc49f2f1b5c4c639b906dbe7ad1eb0
expires: Mon, 01 May 2023 08:26:09 GMT
source-age: 0
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"b5adc93c54eec77d9668a8dd6dd02b7bbd65a0bf4bdc5aed794aaf5feb382ccf"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 210E:1AD0:1BCCB81:1CE0F4C:644F7674
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */257
date: Mon, 01 May 2023 08:21:09 GMT
via: 1.1 varnish
x-served-by: cache-lhr7373-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929269.402450,VS0,VE187
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 0832ecc36b73a08eec1982466f200a9152407eaf
expires: Mon, 01 May 2023 08:26:09 GMT
source-age: 0
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"b5adc93c54eec77d9668a8dd6dd02b7bbd65a0bf4bdc5aed794aaf5feb382ccf"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 210E:1AD0:1BCCB81:1CE0F4C:644F7674
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */257
date: Mon, 01 May 2023 08:21:32 GMT
via: 1.1 varnish
x-served-by: cache-lhr7387-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929293.660513,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: f17737072603a16779c4788ca2fc42f8509ea95c
expires: Mon, 01 May 2023 08:26:32 GMT
source-age: 23
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"b5adc93c54eec77d9668a8dd6dd02b7bbd65a0bf4bdc5aed794aaf5feb382ccf"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 210E:1AD0:1BCCB81:1CE0F4C:644F7674
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */257
date: Mon, 01 May 2023 08:21:32 GMT
via: 1.1 varnish
x-served-by: cache-lhr7336-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929293.667528,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 0085be8640437048c64f99a33555884e3da3129d
expires: Mon, 01 May 2023 08:26:32 GMT
source-age: 23
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"b5adc93c54eec77d9668a8dd6dd02b7bbd65a0bf4bdc5aed794aaf5feb382ccf"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 210E:1AD0:1BCCB81:1CE0F4C:644F7674
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */257
date: Mon, 01 May 2023 08:21:32 GMT
via: 1.1 varnish
x-served-by: cache-lhr7329-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929293.666001,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 0c39f27964cc84937ad523c0ec62f61bfcd493b2
expires: Mon, 01 May 2023 08:26:32 GMT
source-age: 23
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"b5adc93c54eec77d9668a8dd6dd02b7bbd65a0bf4bdc5aed794aaf5feb382ccf"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 210E:1AD0:1BCCB81:1CE0F4C:644F7674
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */257
date: Mon, 01 May 2023 08:21:32 GMT
via: 1.1 varnish
x-served-by: cache-lhr7374-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929293.672809,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 52d8bd50f7745f6d43c843b32433df580398fd8d
expires: Mon, 01 May 2023 08:26:32 GMT
source-age: 23
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"b5adc93c54eec77d9668a8dd6dd02b7bbd65a0bf4bdc5aed794aaf5feb382ccf"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 210E:1AD0:1BCCB81:1CE0F4C:644F7674
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */257
date: Mon, 01 May 2023 08:21:32 GMT
via: 1.1 varnish
x-served-by: cache-lhr7323-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929293.680589,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: abf3cc6875cf9cdbc237f7d956d346024d0417b6
expires: Mon, 01 May 2023 08:26:32 GMT
source-age: 23
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"b5adc93c54eec77d9668a8dd6dd02b7bbd65a0bf4bdc5aed794aaf5feb382ccf"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 210E:1AD0:1BCCB81:1CE0F4C:644F7674
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */257
date: Mon, 01 May 2023 08:21:32 GMT
via: 1.1 varnish
x-served-by: cache-lhr7330-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929293.678931,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 74cfe9fcdd5ad0337e58561d9d6b106c3ee17faf
expires: Mon, 01 May 2023 08:26:32 GMT
source-age: 23
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"b5adc93c54eec77d9668a8dd6dd02b7bbd65a0bf4bdc5aed794aaf5feb382ccf"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 210E:1AD0:1BCCB81:1CE0F4C:644F7674
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */257
date: Mon, 01 May 2023 08:24:17 GMT
via: 1.1 varnish
x-served-by: cache-lhr7349-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929458.996405,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: f8fac5ecc5c73815fe9ab54367fe223047938468
expires: Mon, 01 May 2023 08:29:17 GMT
source-age: 188
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"b5adc93c54eec77d9668a8dd6dd02b7bbd65a0bf4bdc5aed794aaf5feb382ccf"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 210E:1AD0:1BCCB81:1CE0F4C:644F7674
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */257
date: Mon, 01 May 2023 08:24:18 GMT
via: 1.1 varnish
x-served-by: cache-lhr7327-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929458.009955,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: efca292eba3301102e00dcc9df49ecb9e0141558
expires: Mon, 01 May 2023 08:29:18 GMT
source-age: 188
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"b5adc93c54eec77d9668a8dd6dd02b7bbd65a0bf4bdc5aed794aaf5feb382ccf"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 210E:1AD0:1BCCB81:1CE0F4C:644F7674
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */257
date: Mon, 01 May 2023 08:24:18 GMT
via: 1.1 varnish
x-served-by: cache-lhr7374-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929458.010041,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: bf33a0d2c49a2b56d48ab9487f20b77146da5b26
expires: Mon, 01 May 2023 08:29:18 GMT
source-age: 188
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"b5adc93c54eec77d9668a8dd6dd02b7bbd65a0bf4bdc5aed794aaf5feb382ccf"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 210E:1AD0:1BCCB81:1CE0F4C:644F7674
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */257
date: Mon, 01 May 2023 08:24:18 GMT
via: 1.1 varnish
x-served-by: cache-lhr7356-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929458.009980,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 22c7d79441d86fe20ed00a64ada33d69bd5a8be2
expires: Mon, 01 May 2023 08:29:18 GMT
source-age: 188
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"b5adc93c54eec77d9668a8dd6dd02b7bbd65a0bf4bdc5aed794aaf5feb382ccf"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 210E:1AD0:1BCCB81:1CE0F4C:644F7674
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */257
date: Mon, 01 May 2023 08:24:18 GMT
via: 1.1 varnish
x-served-by: cache-lhr7355-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929458.009920,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 4545fe3907f1b45caea1e1bbc63aa71a08477a48
expires: Mon, 01 May 2023 08:29:18 GMT
source-age: 188
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"b5adc93c54eec77d9668a8dd6dd02b7bbd65a0bf4bdc5aed794aaf5feb382ccf"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 210E:1AD0:1BCCB81:1CE0F4C:644F7674
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */257
date: Mon, 01 May 2023 08:24:18 GMT
via: 1.1 varnish
x-served-by: cache-lhr7389-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929458.009958,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: bc99064222b55a12c78998382b6d01a9e2a1ffd2
expires: Mon, 01 May 2023 08:29:18 GMT
source-age: 188
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"b5adc93c54eec77d9668a8dd6dd02b7bbd65a0bf4bdc5aed794aaf5feb382ccf"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 210E:1AD0:1BCCB81:1CE0F4C:644F7674
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */257
date: Mon, 01 May 2023 08:24:23 GMT
via: 1.1 varnish
x-served-by: cache-lhr7385-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929463.259122,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: d2f9f00c52ddc24c0011eecc278801b3f1d0fc41
expires: Mon, 01 May 2023 08:29:23 GMT
source-age: 194
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"b5adc93c54eec77d9668a8dd6dd02b7bbd65a0bf4bdc5aed794aaf5feb382ccf"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 210E:1AD0:1BCCB81:1CE0F4C:644F7674
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */257
date: Mon, 01 May 2023 08:24:23 GMT
via: 1.1 varnish
x-served-by: cache-lhr7348-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929463.288529,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 3358058b0d9612c2689c3f217eb292575aa4dedf
expires: Mon, 01 May 2023 08:29:23 GMT
source-age: 194
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"b5adc93c54eec77d9668a8dd6dd02b7bbd65a0bf4bdc5aed794aaf5feb382ccf"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 210E:1AD0:1BCCB81:1CE0F4C:644F7674
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */257
date: Mon, 01 May 2023 08:24:23 GMT
via: 1.1 varnish
x-served-by: cache-lhr7389-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929463.288566,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: adb892865f2e5f8f5c190d73f5a27c2b1e40b182
expires: Mon, 01 May 2023 08:29:23 GMT
source-age: 194
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"b5adc93c54eec77d9668a8dd6dd02b7bbd65a0bf4bdc5aed794aaf5feb382ccf"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 210E:1AD0:1BCCB81:1CE0F4C:644F7674
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */257
date: Mon, 01 May 2023 08:24:23 GMT
via: 1.1 varnish
x-served-by: cache-lhr7325-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929463.288483,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 8333f398a1e0f170965252656383a6cc35d851e5
expires: Mon, 01 May 2023 08:29:23 GMT
source-age: 194
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"b5adc93c54eec77d9668a8dd6dd02b7bbd65a0bf4bdc5aed794aaf5feb382ccf"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 210E:1AD0:1BCCB81:1CE0F4C:644F7674
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */257
date: Mon, 01 May 2023 08:24:23 GMT
via: 1.1 varnish
x-served-by: cache-lhr7354-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929463.288515,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 44820f6eda21cc21566e032eda00ef8a0e1046e0
expires: Mon, 01 May 2023 08:29:23 GMT
source-age: 194
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"b5adc93c54eec77d9668a8dd6dd02b7bbd65a0bf4bdc5aed794aaf5feb382ccf"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 210E:1AD0:1BCCB81:1CE0F4C:644F7674
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */257
date: Mon, 01 May 2023 08:24:23 GMT
via: 1.1 varnish
x-served-by: cache-lhr7382-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929463.293960,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 6655ac819645e71db29d1ed9bd64d5fa1f211dea
expires: Mon, 01 May 2023 08:29:23 GMT
source-age: 194
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"b5adc93c54eec77d9668a8dd6dd02b7bbd65a0bf4bdc5aed794aaf5feb382ccf"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 210E:1AD0:1BCCB81:1CE0F4C:644F7674
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */257
date: Mon, 01 May 2023 08:24:23 GMT
via: 1.1 varnish
x-served-by: cache-lhr7337-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929463.289129,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 99619ed564baa93925ad53210fec0c324b8c135d
expires: Mon, 01 May 2023 08:29:23 GMT
source-age: 194
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"b5adc93c54eec77d9668a8dd6dd02b7bbd65a0bf4bdc5aed794aaf5feb382ccf"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 210E:1AD0:1BCCB81:1CE0F4C:644F7674
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */257
date: Mon, 01 May 2023 08:24:23 GMT
via: 1.1 varnish
x-served-by: cache-lhr7337-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929463.323471,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 22ca039a88b75e10fa2a0c1c96dab9bb438e1a6b
expires: Mon, 01 May 2023 08:29:23 GMT
source-age: 194
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"b5adc93c54eec77d9668a8dd6dd02b7bbd65a0bf4bdc5aed794aaf5feb382ccf"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 210E:1AD0:1BCCB81:1CE0F4C:644F7674
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */257
date: Mon, 01 May 2023 08:24:29 GMT
via: 1.1 varnish
x-served-by: cache-lhr7370-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929469.073149,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: fb66d2262b5187c376e794874f32f381b6f53ce5
expires: Mon, 01 May 2023 08:29:29 GMT
source-age: 199
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"b5adc93c54eec77d9668a8dd6dd02b7bbd65a0bf4bdc5aed794aaf5feb382ccf"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 210E:1AD0:1BCCB81:1CE0F4C:644F7674
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */257
date: Mon, 01 May 2023 08:24:29 GMT
via: 1.1 varnish
x-served-by: cache-lhr7382-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929469.080382,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 292c383bdb213d890322bef4269841c1c325f2b1
expires: Mon, 01 May 2023 08:29:29 GMT
source-age: 199
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"b5adc93c54eec77d9668a8dd6dd02b7bbd65a0bf4bdc5aed794aaf5feb382ccf"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 210E:1AD0:1BCCB81:1CE0F4C:644F7674
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */257
date: Mon, 01 May 2023 08:24:29 GMT
via: 1.1 varnish
x-served-by: cache-lhr7389-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929469.080173,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 14be7da34e8cd96e03149c3ab2db6270b2a79ccc
expires: Mon, 01 May 2023 08:29:29 GMT
source-age: 199
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"b5adc93c54eec77d9668a8dd6dd02b7bbd65a0bf4bdc5aed794aaf5feb382ccf"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 210E:1AD0:1BCCB81:1CE0F4C:644F7674
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */257
date: Mon, 01 May 2023 08:24:29 GMT
via: 1.1 varnish
x-served-by: cache-lhr7338-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929469.080451,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 5eec7075095890528bc710e081f78e050b3fd2be
expires: Mon, 01 May 2023 08:29:29 GMT
source-age: 199
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"b5adc93c54eec77d9668a8dd6dd02b7bbd65a0bf4bdc5aed794aaf5feb382ccf"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 210E:1AD0:1BCCB81:1CE0F4C:644F7674
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */257
date: Mon, 01 May 2023 08:24:29 GMT
via: 1.1 varnish
x-served-by: cache-lhr7327-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929469.080485,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 108060f07f14fdf91b4747da09520cd1068d5ab2
expires: Mon, 01 May 2023 08:29:29 GMT
source-age: 199
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"b5adc93c54eec77d9668a8dd6dd02b7bbd65a0bf4bdc5aed794aaf5feb382ccf"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 210E:1AD0:1BCCB81:1CE0F4C:644F7674
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */257
date: Mon, 01 May 2023 08:24:29 GMT
via: 1.1 varnish
x-served-by: cache-lhr7373-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929469.080448,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 83c435ef97b5bd229533c5e959e44d4065960b97
expires: Mon, 01 May 2023 08:29:29 GMT
source-age: 199
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"b5adc93c54eec77d9668a8dd6dd02b7bbd65a0bf4bdc5aed794aaf5feb382ccf"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 210E:1AD0:1BCCB81:1CE0F4C:644F7674
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */257
date: Mon, 01 May 2023 08:24:29 GMT
via: 1.1 varnish
x-served-by: cache-lhr7384-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929469.086114,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: 9b97e16f47c9a99578439d0229341a6966c2c20d
expires: Mon, 01 May 2023 08:29:29 GMT
source-age: 199
content-length: 0

HTTP/2 416 
cache-control: max-age=300
content-security-policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
content-type: text/plain; charset=utf-8
etag: W/"b5adc93c54eec77d9668a8dd6dd02b7bbd65a0bf4bdc5aed794aaf5feb382ccf"
strict-transport-security: max-age=31536000
x-content-type-options: nosniff
x-frame-options: deny
x-xss-protection: 1; mode=block
x-github-request-id: 210E:1AD0:1BCCB81:1CE0F4C:644F7674
content-encoding: gzip
accept-ranges: bytes
content-range: bytes */257
date: Mon, 01 May 2023 08:24:29 GMT
via: 1.1 varnish
x-served-by: cache-lhr7331-LHR
x-cache: HIT
x-cache-hits: 0
x-timer: S1682929469.086267,VS0,VE1
vary: Authorization,Accept-Encoding,Origin
access-control-allow-origin: *
x-fastly-request-id: b47e507aa3b9f4bbe5afb0bd2d8372f0830b2b2f
expires: Mon, 01 May 2023 08:29:29 GMT
source-age: 199
content-length: 0

