#![allow(dead_code)]

use curl_ffi as ffi;

pub use curl_ffi::CURLINFO_EFFECTIVE_URL as EFFECTIVE_URL;
pub use curl_ffi::CURLINFO_RESPONSE_CODE as RESPONSE_CODE;
pub use curl_ffi::CURLINFO_TOTAL_TIME as TOTAL_TIME;
pub use curl_ffi::CURLINFO_REDIRECT_COUNT as REDIRECT_COUNT;
pub use curl_ffi::CURLINFO_CONNECT_TIME as CONNECT_TIME;

pub type Key = ffi::CURLINFO;
