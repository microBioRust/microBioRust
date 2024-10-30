This is functionality for a heatmap data visualisation in Rust WebAssembly calling d3.js

D3.js (D3 short for data-driven documents) is a Javascript library for dynamic, interactive data viz in browsers.
At the moment the heatmap data is coded into the Rust lib.rs as an example, so it is currently working with fixed data
and a rusty colour scheme

To install, you can build with wasm-pack 📦✨ 

``wasm-pack build --target web``

And serve it locally, for example with:

``http-server .``

*Installation*
You can install http-server <i>via</i> brew on MacOSX
or with npm
