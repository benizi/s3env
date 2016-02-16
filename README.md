# s3env

(Mainly to learn Rust.)

## What is this?

`s3env` is a command to parse S3 credentials from various tools' normal config
files, then spawn a command with those creds inserted via env vars.

## Current status

- very early on
    - only handles `s3cmd` config (`~/.s3cfg`)
    - can specify a section, though (not a feature of the standard `s3cmd`,
      something I added in [my fork](https://github.com/benizi/s3cmd))
