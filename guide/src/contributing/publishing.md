# Publishing New `wasm-bindgen` Releases

1. <input type="checkbox"/> Compile the `publish.rs` script:

   ```
   rustc publish.rs
   ```

2. <input type="checkbox"/> Bump every crate's minor version:

   ```
   # Make sure you are in the root of the wasm-bindgen repo!
   ./publish bump
   ```

3. <input type="checkbox"/> Update CHANGELOG.md to add the to-be-released version number, compare URL and release date. [See this example](https://github.com/wasm-bindgen/wasm-bindgen/commit/0b5f0eec2f3d5e75a923fd67ef14b3b5cc855c80#diff-06572a96a58dc510037d5efa622f9bec8519bc1beab13c9f251e97e657a9d4ed)

4. <input type="checkbox"/> Send a pull request for the version bump.

5. <input type="checkbox"/> After the pull request's CI is green and it has been
   merged, publish to cargo:

   ```
   # Make sure you are in the root of the wasm-bindgen repo!
   ./publish publish
   ```
