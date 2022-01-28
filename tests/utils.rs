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

#[cfg(test)]
mod test_utils {
    use std::path::PathBuf;

    use ebox::utils;

    #[test]
    fn test_str_to_absolute_path() {
        let path_name = "./";
        let abs = utils::str_to_absolute_path(path_name);
        assert_eq!(abs.ok().unwrap().is_absolute(), true);
    }

    #[test]
    fn test_path_to_absolute_path() {
        let path_name = PathBuf::from("./");
        let abs = utils::path_to_absolute_path(&path_name);
        assert_eq!(abs.ok().unwrap().is_absolute(), true);
    }

    #[test]
    fn test_dir_exist() {
        assert_eq!(utils::dir_exist("./").unwrap(), true);
    }

    #[test]
    fn test_dir_contents() {
        let contents = utils::dir_contents("./");

        if contents.is_err() {
            panic!("{:?}", contents.err());
        }

        assert!(contents.ok().unwrap().len() >= 1);
    }

    #[test]
    fn test_file_exist() {
        let target = PathBuf::from("./Cargo.toml");
        assert_eq!(utils::file_exist(&target).unwrap(), true);
    }
}
