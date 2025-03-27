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
