//! Lolicon API wrapper.
//!
//! Example usage:
//!
//! ```rust
//! use lolicon_api::Request;
//! use lolicon_api::Category;
//! use lolicon_api::ImageSize;
//!
//! let req = Request::default()
//!     .category(Category::R18) // R-18
//!     .num(1).unwrap() // 一张
//!     .uid(&[16731]).unwrap() // 玉之けだま老师
//!     .size(&[ImageSize::Original]).unwrap(); // 原图（默认行为）
//!
//! let req_url = String::from(req);
//!
//! assert_eq!(&req_url, "https://api.lolicon.app/setu/v2?&r18=1&uid=16731");
//! ```
//!
//! Note: `req_url`'s params are sorted by name, and only nessacary ones(i.e. not defaults) will be passed.

mod convert;
mod datatype;

pub use datatype::Request;
pub use datatype::{Category, Error, ImageSize};

#[test]
fn test_num_out_of_range() {
    assert_eq!(
        out_of_range(),
        Err(Error::OutOfRange {
            range: 0..=100,
            actual: 200,
            filed: ""
        })
    );
}

#[test]
fn test_zero_as_num() {
    // there is no `r18=0` in params, as it's by default.
    assert_eq!(Request::default().num(0).unwrap().category(Category::NonR18).to_string(), "https://api.lolicon.app/setu/v2?&num=0")
}

#[cfg(test)]
fn out_of_range() -> Result<(), Error> {
    let _ = Request::default()
        .size(&[])?
        .uid(&[])?
        .category(Category::R18)
        .num(200)?;
    Ok(())
}
