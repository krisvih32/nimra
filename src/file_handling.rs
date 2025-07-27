/*
 * Copyright (C) 2025 Vihaan Krishnan
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/agpl-3.0.html>.
 */

use std::{env, fs};

pub fn get_code() -> Result<String, String> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => fs::read_to_string(args[1].clone())
            .map_err(|e| format!("Error: Unable to read file or directory: {e}")),
        _ => Err("Error: Please provide exactly one argument (the file path)".to_string()),
    }
}
