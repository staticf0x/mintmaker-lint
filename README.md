# MintMaker config linter

This project was created to help [MintMaker](https://github.com/konflux-ci/mintmaker) users test and iterate
on their config faster and more reliably. This is achieved through
validating the config against [Renovate's](https://github.com/renovatebot/renovate) schema and then
performing some MintMaker specific tests.

## Run from terminal

`cargo run --release -- config/file/to/test.json`

## Disable colors

Set the environment variable `NO_COLOR=1`.
