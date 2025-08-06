# web-sys: Weather report

[View full source code][code]

[online]: https://wasm-bindgen.github.io/wasm-bindgen/exbuild/weather_report/
[code]: https://github.com/wasm-bindgen/wasm-bindgen/tree/master/examples/weather_report

This example makes an HTTP request to [OpenWeather API](https://openweathermap.org/),
parses response in JSON and render UI from that JSON. It also shows the usage of
`spawn_local` function for handling asynchronous tasks.

Please add your api key in *get_response()* before running this application.

## `src/lib.rs`

```rust
{{#include ../../../examples/weather_report/src/lib.rs}}
```
