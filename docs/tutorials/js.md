# Using ICU4X from JavaScript and TypeScript

ICU4X's core functionality is completely available from JS and TypeScript via WebAssembly, with bindings generated by [Diplomat].

A sample NPM package can be found under [`ffi/diplomat/js/examples/node`], with Sphinx docs at [`ffi/diplomat/js/docs`]. You can also run `tsdoc` on the package to get TypeScript docs.

_We are still working on improving the user experience of using ICU4X from other languages. As such, this tutorial may be a bit sparse, but we are happy to answer questions on our [discussions forum] and help you out_

# Building the package

The example package contains test data, which is why it's not released on NPM. You will need to build the package yourself to select the desired Rust features.

See [`ffi/diplomat/js/examples/node/package.json`] for inspiration.

# Using the built package

Similar to C++, the JS APIs mirror the Rust code in the `icu_capi` crate, which can be explored on [docs.rs][rust-docs], though the precise types used may be different.

See [`ffi/diplomat/js/examples/wasm-demo`] for an NPM package that uses the example ICU4X package. You can run it using `npm run start`.

We hope to fill in these docs over time with more examples.

 [discussions forum]: https://github.com/unicode-org/icu4x/discussions
 [Diplomat]: https://github.com/rust-diplomat/diplomat
 [`ffi/diplomat/js/examples/node`]: https://github.com/unicode-org/icu4x/tree/main/ffi/diplomat/js/examples/node
 [`ffi/diplomat/js/docs`]: https://github.com/unicode-org/icu4x/tree/main/ffi/diplomat/js/docs
 [`ffi/diplomat/js/examples/wasm-demo`]: https://github.com/unicode-org/icu4x/tree/main/ffi/diplomat/js/examples/wasm-demo