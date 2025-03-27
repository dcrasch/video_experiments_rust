Using:

0. load an image using the [image](https://github.com/image-rs/image) crate
1. single frame convert from rgb using [uyvutils](https://crates.io/crates/yuvutils-rs) crate to yuv420
2. and encode this frame using [rave1](https://github.com/xiph/rav1e/) to av1 and 
3. [vvpx-encode](https://github.com/astraw/vpx-encode/) for vp9
4. the webm crate [webm](https://github.com/DiamondLovesYou/rust-webm) to store in the webm container file.
