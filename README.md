# w2e - Where to eat

Too many food options nearby and always have no idea where to go? `w2e` randomly picks a spot from an input list and displays it on your [MagicMirror](https://github.com/MichMich/MagicMirror). Anything else that needs to be randomized works too.

![Screenshot](media/media.png)

Set up a configuration `yaml` file and pass it in with the `-f` flag. An example is provided at [example.yaml](example.yaml).

This program works with a `MagicMirror`, that has the [MMM-IFTTT](https://github.com/jc21/MMM-IFTTT) module installed.

The `MMM-IFTTT` module requires an additional step of setting up DNS and port forwarding. I tried but couldn't get it all working, so I wrote this instead.

The `mirror_client` is run on the magic mirror, while a host computer can run the `randomizer`. The randomized choice is published over the `zenoh` middleware under the key expression `magic_mirror/w2e`, received by the client, and a `POST` request to `MMM-IFTTT` is performed.

## Pre-requisites

Install [`rust`](https://www.rust-lang.org/tools/install), on both the magic mirror and host.

Or just cross-compile everything on host and send the binaries over to the magic mirror.

## Run on magic mirror

```bash
git clone https://github.com/aaronchong/w2e
cd w2e
cargo build --release
# or just cross-compile and send the binaries over

cargo run --bin mirror_client
```

## Run on host

```bash
git clone https://github.com/aaronchongth/w2e
cd w2e
cargo build --release
cargo run --bin randomizer -- -f example.yaml
```
