/*
FaF is a high performance dns benchmarking tool
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

pub const PROJECT_NAME: &str = env!("CARGO_PKG_NAME");
//pub const PROJECT_DIR: &str = env!("CARGO_MANIFEST_DIR");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub static ARGS: once_cell::sync::Lazy<crate::args::Args> = once_cell::sync::Lazy::new(clap::Parser::parse);
