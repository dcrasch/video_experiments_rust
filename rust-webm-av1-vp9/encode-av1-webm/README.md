# Create webm file with av1 encoded frames

Use the rav1e library to encode the frames.

## Run

Run with the --release flag, otherwise is sloooooow.

```
cargo run --release
```

## check file

```
webm_info -i output.webm
```

## Requirements

```
apt install libvpx-dev libwebm-tools
```

## TODO

[ ] fix codecprivate data, this is required for an av1 stream inside webm, but I have no idea what to put there. 0x1 is valid?
