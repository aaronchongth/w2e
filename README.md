# where_to_eat

Too many food options nearby and always have no idea where to go? `where_to_eat` randomly picks a spot from an input list and displays it on your [MagicMirror](https://github.com/MichMich/MagicMirror). Anything else that needs to be randomized works too.

Set up a configuration `yaml` file and pass it in with the `-f` flag. An example is provided at [example.yaml](example.yaml).

This program works with a `MagicMirror`, that has the [MMM-IFTTT](https://github.com/jc21/MMM-IFTTT) module installed.

The `MMM-IFTTT` module requires an additional step of setting up DNS and port forwarding. I tried but couldn't get it all working, so I wrote this instead. It just `ssh` into the unit running the `MagicMirror`, and calls a `curl` command.

## Pre-requisites

Make sure that the unit running the `MagicMirror` has `curl` installed,

```bash
sudo apt install curl -y
```

## Run

Install [`rust`](https://www.rust-lang.org/tools/install), build and run,

```bash
git clone https://github.com/aaronchongth/what_to_eat
cd what_to_eat
cargo run -- -f example.yaml
```
