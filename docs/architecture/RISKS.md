# Technical risks and validation gates

| Risk | Mitigation / owner issue | Current evidence |
| --- | --- | --- |
| Transparent WebView2 artifacts / hit testing | Real overlay tests; whole-window passive mode with tray recovery; LUK-18 | See VALIDATION.md |
| Focus stealing and keyboard accessibility | Startup without focus; explicit interactive policy; LUK-18/64 | Full transition matrix pending |
| Mixed DPI, negative coordinates, monitor hot-plug | Separate logical dimensions from physical monitor bounds; LUK-22/65 | Primary-monitor initial placement only |
| Background WebView cost | No background polling in foundation; profile host + child processes; LUK-61 | No benchmark claim |
| Windows hooks / OLE / DWM complexity | Dedicated disposable native adapters; LUK-50–59 | Not implemented |
| OS backdrop differs from CSS blur | Native composition spike with opaque fallback; LUK-23 | No final glass promised |
| Unsupported download/AI metrics | Supported sources, explicit unavailable state; LUK-36/37 | No scraping or integration implemented |
| Module coupling / privilege escalation | Import checks, injected services, native authorization; LUK-17/62 | Import/lifecycle tests; no untrusted plugins |
| Settings corruption / failed writes | Validation, atomic replacement, no overwrite of invalid/future schemas; LUK-20 | Rust tests; recovery UI remains future work |
| Duplicate processes writing settings | Singleton before production use; LUK-19 | M0 prototype must run one instance at a time |
| Installer/update trust | Signed packages and signed update metadata; LUK-66/67 | Build only; no release pipeline |
| Review gate | Public repository; branch workflow and CI retained | Main protection disabled by owner for solo development; review before merging |

Open M0 gates must be recorded on LUK-18 and LUK-5. Do not close the parent milestone while the native spike remains unverified.
