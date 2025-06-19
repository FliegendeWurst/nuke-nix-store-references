# nuke-nix-store-references

Quick script to remove store hashes from text files.

## Installation

`cargo install --git ` followed by this repository's URL

## Usage

- With no argements: stdin → stdout.

Example: `nix log /nix/store/q8bik9dyjhkrz0k73147xyy2xgwdx6k1-lilypond-2.24.4 | ansi2txt | nuke-nix-store-references > /tmp/x`

- With argument(s): process all arguments as files (overwrites original!)

It also removes lines starting with `@nix { "action": "setPhase", "phase": "` to align with Hydra defaults

## License

MIT License

Copyright 🄯 2025 FliegendeWurst
