A Rust crate to transmogrify English strings.

## Examples

The unenglish function has three modes:

## License

Licensed under either of:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

### Dev Notes

If you just want to run a single test (and see its console output)

```
#[test]
fn demo() {
    let basic_string = "Hello, World.";
    print!("basic_string {}", basic_string);
    let result = transmogrify("Hello, World!", &UnenglishMode::Japanglish);
    assert_eq!(result, "井巳レレ回, 山回尺レ占!");
}
```

Then just enter the command

```
cargo test -- --nocapture demo
```

😁

