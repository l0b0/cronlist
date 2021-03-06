# cronlist [![Build Status](https://travis-ci.org/l0b0/cronlist.svg?branch=master)](https://travis-ci.org/l0b0/cronlist) [![codecov](https://codecov.io/gh/l0b0/cronlist/branch/master/graph/badge.svg)](https://codecov.io/gh/l0b0/cronlist)

`cronlist` lists upcoming cron actions from `/etc/crontab` and your personal `crontab`. By default, the next ten actions are printed.

## Warning

This implementation is not timezone aware, so don't expect it to work in the presence of:

- `CRON_TZ`
- Daylight savings time

## Installation

    git submodule update --init
    make
    sudo make install

## Rust alpha notes

Build, test, lint & format:

    rustup default nightly
    rustup component add rustfmt-preview
    cargo install --force clippy
    make --file=rust.mk test lint

Build optimized binary:

    make --file=rust.mk release

Run:

    crontab -l | ./target/release/cronlist
