/*
FaF is a high performance json benchmarking tool
Copyright (C) 2023  James Bates

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use clap::Parser;

/// FaF HTTP Bench - A HTTP Server Benchmarker
#[derive(Parser, Debug, Default)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
   /// enable debug output
   #[clap(long)]
   #[clap(default_value_t = false)]
   pub debug: bool,

   /// the number of threads to use, defaults to the number of logical cores
   #[clap(short, long)]
   pub threads: Option<usize>,

   /// the duration of the test in seconds
   #[clap(short, long)]
   #[clap(default_value_t = 3)]
   pub duration: u64,

   /// the name of the benchmark group. Used when saving the results to file
   #[clap(short, long)]
   #[clap(default_value = "untitled")]
   pub bench: String,

   /// clear the saved benchmark stats
   #[clap(long)]
   #[clap(default_value_t = false)]
   pub clear: bool,

   /// clear the saved benchmark stats
   #[clap(short, long)]
   #[clap(default_value_t = false)]
   pub about: bool,
}
