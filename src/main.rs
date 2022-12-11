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

extern crate clap;
extern crate console;

mod application;
mod file;

use console::Style;
use clap::{arg, Command};
use application::Application;

fn usage(argv0: &str) {
    let default_style = Style::new().bright().bold().white();
    let green_style = Style::new().bold().green();
    println!("{}{} [Options]... [--help | -h]", default_style.apply_to("Usage: "), argv0);
    println!();
    println!("{}", green_style.apply_to("Available commands:"));
    println!("\t{} Shows This Information.", default_style.apply_to("--help (-h)"));
    println!("\t{} Sets the version of the plugin.", default_style.apply_to("--version (-v)"));
    println!("\t{} Sets the output directory to place the generated files to.", default_style.apply_to("--directory (-d)"));
    println!("\t{} Language to be used for the plugin (C/C++/Rust). Currently does nothing.", default_style.apply_to("--language (-l)"));
    println!();
    println!("{}", green_style.apply_to("Runtime Information"));
    println!("\t{} -> v0.1.0", default_style.apply_to("Version"));
    println!("\t{} -> v1.0.0", default_style.apply_to("Specification Compatibility"));
    println!("\t{} -> v4.0.18", default_style.apply_to("Command Line Parser Version (clap)"), );
}

fn main() -> Result<(), String> {
    let appname : String;
    let version : String;
    let language;
    let directory;

    /* I don't know why clap won't recognize --help. I gotta do it manually. */
    std::env::args().for_each(|f|
        if f == "--help" || f == "-h" {
            usage("vplugin-init");
            std::process::exit(0);
        }
    );

    let matches = Command::new("vplugin-init")
        .version("0.1.0")
        .author("Aggelos Tselios <aggelostselios777@gmail.com>")
        .about("A simple command line utility to create a base VPlugin module.")
        .arg(arg!(--name      <VALUE>).required(true).short('n').help("Sets the plugin's name."))
        //.arg(arg!(--version   <VALUE>).required(true).short('v').help("Sets the plugin's version."))
        .arg(arg!(--language  <VALUE>).short('l').help("Sets the programming language that this plugin will use (Currently does nothing)"))
        .arg(arg!(--directory <DIR>).required(true).short('d').help("Where to write the generated plugin."))
        .get_matches();

    appname   = matches.get_one::<String>("name")    .unwrap().to_string();
    version   = matches.get_one::<String>("version") .unwrap().to_string();
    language  = matches.get_one::<String>("language");
    directory = matches.get_one::<String>("directory");

    Application::run(appname, version, directory, language).expect("Error.");
    Application::print_debug(
        "vplugin-init",
        &format!(
            "Created plugin: {} (v{})",
            matches.get_one::<String>("name").unwrap(),
            matches.get_one::<String>("version").unwrap(),
        )
    );

    Ok(())
}