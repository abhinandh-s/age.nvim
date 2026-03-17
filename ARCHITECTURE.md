## On the use of AI in this project

The use of AI in this project is limited to unit tests only.

## References

1. [Parse, don't Validate and Type-Driven Design in Rust](https://www.harudagondi.space/blog/parse-dont-validate-and-type-driven-design-in-rust) by Gio Genre De Asis

```rust 
// types.rs
struct ExistingAgeFile<'a>(Cow<'a, Path>);
struct ExistingNonAgeFile<'a>(Cow<'a, Path>);
```

Both are inspired by this article.