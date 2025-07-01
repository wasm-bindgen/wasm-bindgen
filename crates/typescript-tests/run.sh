#!/bin/sh

set -ex

cd `dirname "$0"`

cargo build --target wasm32-unknown-unknown

rm -rf pkg
mkdir pkg
cargo run -p wasm-bindgen-cli --bin wasm-bindgen -- \
  ../../target/wasm32-unknown-unknown/debug/typescript_tests.wasm \
  --out-dir pkg \
  --typescript

mkdir pkg/web
cargo run -p wasm-bindgen-cli --bin wasm-bindgen -- \
  ../../target/wasm32-unknown-unknown/debug/typescript_tests.wasm \
  --out-dir pkg/web \
  --target web \
  --typescript

if [ ! -d node_modules ]; then
  npm install
fi

npm run tsc

# Build using no-modules.
mkdir pkg/no_modules
cargo run -p wasm-bindgen-cli --bin wasm-bindgen -- \
  ../../target/wasm32-unknown-unknown/debug/typescript_tests.wasm \
  --out-dir pkg/no_modules \
  --target no-modules \
  --typescript

# Then create copies of the TypeScript tests used on "--target web" (i.e. those in ./src) but adjust them to work with "--target no-modules".
# Store the new generated test files under "src_no_modules".
rm -rf src_no_modules
mkdir src_no_modules
cd src
for filename in *.ts; do
  if grep -q "_bg.wasm" "$filename"
  then
    # I am unsure how to import or test the "*_bg.wasm" file.
    # So any test file that includes those is currently being skipped.
    echo "Ignoring $filename as it uses the *.wasm.d.ts file."
  else
    path="../src_no_modules/$filename"
    # Copy over every line EXCEPT for the import statements (since "no-modules" has no modules to import).
    grep -v -w "import" "$filename" > "$path"
    # Then replace all of the instances of "wbg" (the namespace all the tests use to import the *.d.ts files from "--target web") with "wasm_bindgen" (the namespace the `no-modules` *.d.ts files use).
    sed -i'.orig' "s/wbg\./wasm_bindgen./g" "$path"
    sed -i'.orig' "s/wbg /wasm_bindgen /g" "$path"
    sed -i'.orig' "s/wbg\[/wasm_bindgen[/g" "$path"
  fi
done
cd ..

# Then try to build the typescript in the src_no_modules folder against the pkg/no_modules build.
npm run tsc -- -p no_modules.tsconfig.json

npm test
