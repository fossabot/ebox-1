//  Copyright (C) 2022  The ebox authors.
//  This file is part of ebox.
//
//  This program is free software: you can redistribute it and/or modify
//  it under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  This program is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//  GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::{fs, io::Error, path::PathBuf};

/// function for getting absolute path from string.
pub fn str_to_absolute_path(target: &str) -> Result<PathBuf, Error> {
    let path = PathBuf::from(target);
    if path.is_absolute() {
        return Ok(path);
    }

    fs::canonicalize(path)
}

/// function for getting absolute path from PathBuf.
pub fn path_to_absolute_path(path: &PathBuf) -> Result<PathBuf, Error> {
    if path.is_absolute() {
        return Ok(path.to_path_buf());
    }

    fs::canonicalize(path)
}

/// function for check if directory exist.
pub fn dir_exist(target: &str) -> Result<bool, Error> {
    let path = str_to_absolute_path(target)?;
    Ok(path.is_dir())
}

/// function for getting directory contents.
pub fn dir_contents(target: &str) -> Result<Vec<PathBuf>, Error> {
    let path = str_to_absolute_path(target)?;
    let mut contents: Vec<PathBuf> = Vec::new();

    for ele in fs::read_dir(path)? {
        let ele_path = path_to_absolute_path(&ele.unwrap().path())?;
        contents.push(ele_path);
    }

    Ok(contents)
}

/// function for check if file exist.
pub fn file_exist(target: &PathBuf) -> Result<bool, Error> {
    let abs = path_to_absolute_path(target)?;
    Ok(abs.exists() && !abs.is_dir())
}

/// function for read a file.
pub fn file_read(target: &PathBuf) -> Result<String, Error> {
    let abs = path_to_absolute_path(target)?;
    fs::read_to_string(abs)
}

/// function for write a new file.
pub fn file_new(target: &PathBuf, contents: &str) -> Result<(), Error> {
    let abs = path_to_absolute_path(target)?;
    fs::write(abs, contents)
}
