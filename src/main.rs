// Copyright (c) 2026, Yago Mouriño Mendaña <ylabs82@gmail.com>
// SPDX-License-Identifier: MIT

use clap::Parser;

use crate::args::Args;

mod args;

fn main() {
    let _args: Args = Args::parse();
}
