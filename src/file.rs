/*
 * vplugin-init - A simple program to create a new plugin for the VPlugin framework.
 * Copyright (C) 2022  Aggelos Tselios
 *
 * vplugin-init is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * vplugin-init is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with vplugin-init.  If not, see <https://www.gnu.org/licenses/>.
*/

use crate::application::Application;
use std::fs;

pub struct FileManager;

impl FileManager {
        pub fn new_directory(path: &str) {
                Application::print_debug("Creating directory", path);
                let result = fs::create_dir(path);
                if result.is_err() {
                        Application::print_error(
                                &format!(
                                        "Couldn't create directory {}: {:?}",
                                        path,
                                        result.as_ref().unwrap_err().kind()
                                )
                        );
                        std::process::exit(result.unwrap_err().raw_os_error().unwrap());
                }
        }

        pub fn new_file(path: &str) {
                Application::print_debug("Creating file", path);
                let result = fs::File::create(path);
                if result.is_err() {
                        Application::print_error(
                                &format!(
                                        "Couldn't create file {}: {:?}",
                                        path,
                                        result.unwrap_err().kind()
                                )
                        );
                }
        }
}