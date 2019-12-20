
## Unreleased

* Add `raise` macro that makes building `Fail` more easy.

## 0.2.0 (2019-12-05)

* `ErrorMessageExt` is replaced by `FailExt`.
* `Fail` is now struct.
    - Internal variants will be hidden.

## 0.1.0 (2019-10-14)

* Added Types
    - [`Fail`](https://docs.rs/tiny_fail/0.1.0/tiny_fail/enum.Fail.html)
    - [`Error`](https://docs.rs/tiny_fail/0.1.0/tiny_fail/struct.Error.html)
* Added Traits
    - [`ErrorMessageExt`](https://docs.rs/tiny_fail/0.1.0/tiny_fail/trait.ErrorMessageExt.html)