//
// Libraries - native
//
use std::ffi::OsStr;
use std::fmt::{Debug, Display};
use std::fs::DirEntry;
use std::path::{Path, PathBuf};
//
// Libraries - downloaded
//
use shellexpand;
use substring::Substring;
//
// Tests
//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_absolute_path() {
        let string_path = "./test/test_b/test_c/../";
        assert_eq!(
            get_absolute_path(&string_path),
            format!("{}/test/test_b", env!("CARGO_MANIFEST_DIR")),
        );
        let string_result = get_absolute_path(&"~/test");
        if string_result.contains("~") {
            panic!("Result contains tilde.")
        }
        if !string_result.ends_with("/test") {
            panic!("Result doesn't end with '/test'.")
        }
    }

    #[test]
    fn test_get_absolute_path_or_error() {
        let string_path = "./test/test_b/test_c/../";
        let result = match get_absolute_path_or_error(&string_path) {
            Ok(string_result) => string_result,
            Err(err) => panic!("{}", err,),
        };
        let expected = format!("{}/test/test_b", env!("CARGO_MANIFEST_DIR").to_string(),);
        assert_eq!(result, expected)
    }

    #[test]
    fn test_get_base_name_with_extension() {
        let string_path = "test/test_a/text_a_a.txt";
        let result = match get_base_name(&string_path) {
            Some(string_result) => string_result,
            None => panic!(""),
        };
        let expected = "text_a_a.txt".to_string();
        assert_eq!(result, expected)
    }

    #[test]
    fn test_get_base_name_on_dir() {
        let string_path = "test/test_b/test_c/text_b_c_a.txt";
        let result = match get_base_name(&string_path) {
            Some(string_result) => string_result,
            None => panic!(""),
        };
        let expected = "text_b_c_a.txt".to_string();
        assert_eq!(result, expected)
    }

    #[test]
    fn test_get_common_path() {
        let slice_of_strings = [
            "src/helpers_disk/A/B/C",
            "src/helpers_disk/A/B",
            "src/helpers_disk/A",
        ];
        let result = match get_common_path(&slice_of_strings) {
            Ok(string_result) => string_result,
            Err(err) => panic!("{}", err),
        };
        let expected = "src".to_string();
        assert_eq!(result, expected)
    }

    #[test]
    fn test_get_common_prefix() {
        let slice_of_strings = [
            "src/helpers_disk/A/B/C",
            "src/helpers_disk/A/B",
            "src/helpers_disk/A",
        ];
        let result = match get_common_prefix(&slice_of_strings) {
            Ok(string_result) => string_result,
            Err(err) => panic!("{}", err),
        };
        let expected = "src/helpers_disk/A".to_string();
        assert_eq!(result, expected)
    }

    #[test]
    fn test_get_dir_name() {
        let string_path = "test/test_b/test_c/text_b_c_a.txt";
        let result = match get_dir_name(&string_path) {
            Some(string_result) => string_result,
            None => panic!("Failed"),
        };
        let expected = "test/test_b/test_c".to_string();
        assert_eq!(result, expected)
    }

    #[test]
    fn test_get_dir_ancestor_n_levels_up() {
        let string_path = "test/test_b/test_c/text_b_c_a.txt";
        let int_layers_up: usize = 2;
        let result = match get_dir_ancestor_n_levels_up(&string_path, int_layers_up) {
            Some(string_result) => string_result,
            None => panic!("Failed"),
        };
        let expected = "test/test_b".to_string();
        assert_eq!(result, expected)
    }

    #[test]
    fn test_get_dir_ancestor_that_exists() {
        let string_path = "test/test_b/test_c/text_b_c_a.txt/A/B/C";
        let result = match get_dir_ancestor_that_exists(&string_path) {
            Some(string_result) => string_result,
            None => panic!("Failed"),
        };
        let expected = "test/test_b/test_c/text_b_c_a.txt".to_string();
        assert_eq!(result, expected)
    }

    #[test]
    fn test_get_extension() {
        let string_path = "test/test_b/test_c/text_b_c_a.txt";
        let result = match get_extension(&string_path) {
            Some(string_result) => string_result,
            None => panic!("Failed"),
        };
        let expected = "txt".to_string();
        assert_eq!(result, expected)
    }

    #[test]
    fn test_get_only_dirs_from_slice() {
        let slice_of_strings = [
            "test/test_b",
            "test/test_b/test_c",
            "test/test_b/test_c/text_b_c_b.txt",
            "test/test_b/test_c/text_b_c_a.txt",
            "test/test_a",
            "test/test_a/text_a_a.txt",
        ];
        let result = get_only_dirs_from_slice(&slice_of_strings);
        let expected = ["test/test_b", "test/test_b/test_c", "test/test_a"]
            .iter()
            .map(|item| format!("{}", item,))
            .collect::<Vec<String>>();
        assert_eq!(result, expected,)
    }

    #[test]
    fn test_get_only_file_paths_from_slice() {
        let slice_of_strings = [
            "test/test_b",
            "test/test_b/test_c",
            "test/test_b/test_c/text_b_c_b.txt",
            "test/test_b/test_c/text_b_c_a.txt",
            "test/test_a",
            "test/test_a/text_a_a.txt",
        ];
        let result = get_only_file_paths_from_slice(&slice_of_strings);
        let expected = [
            "test/test_b/test_c/text_b_c_b.txt",
            "test/test_b/test_c/text_b_c_a.txt",
            "test/test_a/text_a_a.txt",
        ]
        .iter()
        .map(|item| format!("{}", item,))
        .collect::<Vec<String>>();
        assert_eq!(result, expected,)
    }

    #[test]
    fn test_get_path_joined() {
        let slice_of_strings = ["A", "B", "C"];
        let result = match get_path_joined(&slice_of_strings) {
            Some(string_result) => string_result,
            None => panic!("Failed"),
        };
        let expected = "A/B/C".to_string();
        assert_eq!(result, expected)
    }

    #[test]
    fn test_get_paths_in_dir() {
        let string_path = "test";
        let result = match get_paths_in_dir(&string_path) {
            Ok(vec_result) => vec_result,
            Err(err) => panic!("{}", err,),
        };
        let expected = ["test/test_a", "test/test_b"]
            .iter()
            .map(|item_str| item_str.to_string())
            .collect::<Vec<String>>();
        assert_eq!(result, expected)
    }

    #[test]
    fn test_get_paths_in_dir_and_sub_dirs() {
        let string_path = "test";
        let result = match get_paths_in_dir_and_sub_dirs(&string_path) {
            Ok(vec_result) => vec_result,
            Err(err) => panic!("{}", err),
        };
        let expected = [
            "test/test_b",
            "test/test_b/test_c",
            "test/test_b/test_c/text_b_c_b.txt",
            "test/test_b/test_c/text_b_c_a.txt",
            "test/test_a",
            "test/test_a/text_a_a.txt",
        ]
        .iter()
        .map(|item_str| item_str.to_string())
        .collect::<Vec<String>>();
        assert_eq!(result, expected)
    }

    #[test]
    fn test_get_paths_sorted_by_size_starting_with_shortest() {
        let slice_of_strings = ["/A/B/C", "/A", "/A/B"];
        let result = match get_paths_sorted_by_size_starting_with_shortest(&slice_of_strings) {
            Ok(vec_result) => vec_result,
            Err(err) => panic!("{}", err),
        };
        let expected = ["/A", "/A/B", "/A/B/C"]
            .iter()
            .map(|item_str| item_str.to_string())
            .collect::<Vec<String>>();
        assert_eq!(result, expected)
    }

    #[test]
    fn test_get_paths_to_only_dirs_in_dir_and_sub_dirs() {
        let string_path = "test";
        let result = match get_paths_to_only_dirs_in_dir_and_sub_dirs(&string_path) {
            Ok(vec_result) => vec_result,
            Err(err) => panic!("{}", err,),
        };
        let expected = ["test/test_b", "test/test_b/test_c", "test/test_a"]
            .iter()
            .map(|item_str| item_str.to_string())
            .collect::<Vec<String>>();
        assert_eq!(result, expected)
    }

    #[test]
    fn test_get_paths_to_only_files_in_dir_and_sub_dirs() {
        let string_path = "test";
        let result = match get_paths_to_only_files_in_dir_and_sub_dirs(&string_path) {
            Ok(vec_result) => vec_result,
            Err(err) => panic!("{}", err,),
        };
        let expected = [
            "test/test_b/test_c/text_b_c_b.txt",
            "test/test_b/test_c/text_b_c_a.txt",
            "test/test_a/text_a_a.txt",
        ]
        .iter()
        .map(|item_str| item_str.to_string())
        .collect::<Vec<String>>();
        assert_eq!(result, expected)
    }

    #[test]
    fn test_get_relative_path() {
        let string_path_abs_root = "/A/B/C";
        let string_path_abs = "/A/B/C/D";
        let result = match get_relative_path(&string_path_abs, &string_path_abs_root) {
            Ok(string_result) => string_result,
            Err(err) => panic!("{}", err,),
        };
        let expected = "D".to_string();
        assert_eq!(result, expected)
    }

    #[test]
    fn test_get_vec_by_splitting_path() {
        let string_path = "test/test_b/test_c/text_b_c_a.txt";
        let result = match get_vec_by_splitting_path(&string_path) {
            Some(vec_result) => vec_result,
            None => panic!("Failed"),
        };
        let expected = ["test", "test_b", "test_c", "text_b_c_a.txt"]
            .iter()
            .map(|item_str| item_str.to_string())
            .collect::<Vec<String>>();
        assert_eq!(result, expected)
    }

    #[test]
    fn test_is_absolute() {
        assert_eq!(is_absolute(&"/A/B/C"), true,)
    }

    #[test]
    fn test_is_existing_path() {
        assert_eq!(is_existing_path(&"test/test_b/test_c/text_b_c_a.txt"), true,)
    }

    #[test]
    fn test_is_path_type() {
        assert_eq!(
            is_path_type(&Path::new(&"test/test_b/test_c/text_b_c_a.txt")),
            true,
        )
    }

    #[test]
    fn test_is_path_buf_type() {
        assert_eq!(
            is_path_buf_type(&PathBuf::from("test/test_b/test_c/text_b_c_a.txt")),
            true,
        )
    }

    #[test]
    fn test_is_path_inside_dir_parent() {
        assert_eq!(
            is_path_inside_dir_parent(&"test/test_b/test_c/text_b_c_a.txt", &"test/test_b"),
            true,
        )
    }

    #[test]
    fn test_raise_error_if_path_is_not_in_project_absolute() {
        let mut string_path = "/badpath";
        match raise_error_if_path_is_not_in_project(&string_path) {
            Ok( () ) => {
                panic!(
                    "{}",
                    [
                        "Did not return error on bad absolute path.".to_string(),
                        format!("string_path = {}", string_path,)
                    ]
                        .join("\n")
                )
            }
            Err( _err ) => {}
        }
        string_path = "test";
        match raise_error_if_path_is_not_in_project(&string_path) {
            Ok(()) => {}
            Err(_err) => {
                panic!(
                    "{}",
                    [
                        "Returned error on good relative path.".to_string(),
                        format!("string_path = {}", string_path,),
                    ]
                        .join("\n")
                )
            }
        }
        string_path = "bad/test";
        match raise_error_if_path_is_not_in_project(&string_path) {
            Ok(()) => {
                panic!(
                    "{}",
                    [
                        "Did not return error on bad relative path.".to_string(),
                        format!("string_path = {}", string_path,),
                    ]
                        .join("\n")
                )
            }
            Err(_err) => {}
        }
    }

    #[test]
    fn test_raise_error_if_path_points_to_src() {
        match raise_error_if_path_points_to_src(&"src") {
            Ok(()) => { panic!("Did not return error") }
            Err( _err ) => {},
        }
        match raise_error_if_path_points_to_src(&"src/") {
            Ok(()) => { panic!("Did not return error") }
            Err(_err) => {},
        }
        match raise_error_if_path_points_to_src(&format!("{}/src/", env!("CARGO_MANIFEST_DIR"),)) {
            Ok(()) => {panic!("No error returned")}
            Err(_err) => {},
        }
        match raise_error_if_path_points_to_src(&"src/") {
            Ok(()) => {panic!("No error returned")}
            Err(_err) => {},
        }
    }

    #[test]
    fn test_raise_error_if_path_points_to_cargo_toml() {
        let mut string_path = format!("{}/Cargo.toml", env!("CARGO_MANIFEST_DIR"),);
        match raise_error_if_path_points_to_cargo_toml(&string_path) {
            Ok(()) => {
                panic!(
                    "{}",
                    [
                        "Didn't raise error when passed the absolute path to Cargo.toml"
                            .to_string(),
                        format!("string_path = {}", string_path,),
                    ]
                        .join("\n")
                )
            }
            Err(_err) => {}
        }
        string_path = "Cargo.toml".to_string();
        match raise_error_if_path_points_to_cargo_toml(&string_path) {
            Ok(()) => {
                panic!(
                    "{}",
                    [
                        "Didn't raise error when passed the relative path to Cargo.toml".to_string(),
                        format!("string_path = {}", string_path,),
                    ]
                        .join("\n")
                )
            }
            Err(_err) => {},
        }
        string_path = "src".to_string();
        match raise_error_if_path_points_to_cargo_toml(&string_path) {
            Ok(()) => {}
            Err(_err) => {
                panic!(
                    "{}",
                    [
                        "Raised error when not pointing to Cargo.toml".to_string(),
                        format!("string_path = {}", string_path,),
                    ]
                        .join("\n")
                )
            }
        }
    }

    #[test]
    fn test_raise_error_if_path_points_to_main_rs() {
        let mut string_path = format!("{}/src/main.rs", env!("CARGO_MANIFEST_DIR"),);
        match raise_error_if_path_points_to_main_rs(&string_path) {
            Ok(()) => {
                panic!(
                    "{}",
                    [
                        "Failed to return error when passed absolute path to main.rs".to_string(),
                        format!("string_path = {}", string_path,),
                    ]
                        .join("\n")
                )
            }
            Err(_err) => {}
        }
        string_path = "src/main.rs".to_string();
        match raise_error_if_path_points_to_main_rs(&string_path) {
            Ok(()) => {
                panic!(
                    "{}",
                    [
                        "Failed to return error when passed relative path to main.rs".to_string(),
                        format!("string_path = {}", string_path,),
                    ]
                        .join("\n")
                )
            }
            Err(_err) => {}
        }
        string_path = "src".to_string();
        match raise_error_if_path_points_to_main_rs(&string_path) {
            Ok(()) => {

            }
            Err(_err) => {
                panic!(
                    "{}",
                    [
                        "Raised error when not pointing at main.rs".to_string(),
                        format!("string_path = {}", string_path,),
                    ]
                        .join("\n")
                )
            }
        }
    }
}
//
// Public - get - paths
//
/// This attempts to get the full path
///
/// If the conversion fails, this will return the 'most complete' string it built
/// up to that point.
///
/// Since canonicalize requires the paths to exist, if the argument doesn't exist in this case
/// this function will return the path with the expanded tilde, but will still contain any positional
/// elements (ie '..')
///
/// This supports '~', '.', '..'
/// # Arguments
/// * arg_string_path: string-like
/// # Examples
/// let string_absolute_path = get_absolute_path( &string_path_relative );
pub fn get_absolute_path<T: Debug + Display>(arg_string_path: &T) -> String {
    let string_path = get_path_with_tilde_expanded_if_necessary(&arg_string_path);
    //
    // Reminder: canonicalize() will throw an error if the path doesn't exist
    //
    match std::fs::canonicalize(PathBuf::from(&string_path)) {
        Ok(path_buf_result) => match path_buf_result.to_str() {
            Some(str_result) => str_result.to_string(),
            None => return string_path,
        },
        Err(_err) => string_path,
    }
}

/// Similar to get_absolute_path() except this is 'all-or-nothing.' If any step fails, this returns
/// an error message explaining which step failed.
/// Since canonicalize requires the final path to exist, this will count as a 'failure' condition.
/// This supports '~', '.', '..'
/// # Arguments
/// * arg_string_path: string-like
/// # Examples
/// let string_absolute_path = match get_absolute_path( &string_path_relative ) {
///     Ok( string_result ) => { string_result }
///     Err( err ) => { panic!( "{}", err ) }
/// };
pub fn get_absolute_path_or_error<T: Debug + Display>(
    arg_string_path: &T,
) -> Result<String, String> {
    let string_path = get_path_with_tilde_expanded_if_necessary(&arg_string_path);
    //
    // Reminder: canonicalize() will throw an error if the path doesn't exist
    //
    match std::fs::canonicalize(PathBuf::from(&string_path)) {
        Ok(path_buf_result) => match path_buf_result.to_str() {
            Some(str_result) => Ok(str_result.to_string()),
            None => {
                return Err([
                    "Error: Failed to extract str from PathBuf.".to_string(),
                    format!("arg_string_path = {}", arg_string_path,),
                    format!("path built = {}", string_path,),
                ]
                .join("\n"))
            }
        },
        Err(err) => {
            return Err([
                "Error: Failed to 'canonicalize' string_path.".to_string(),
                format!("arg_string_path = {}", arg_string_path,),
                format!("err = {}", err,),
                format!("path built = {}", string_path,),
            ]
            .join("\n"))
        }
    }
}

/// Returns a string consisting of only the filename
/// Returns None in case of failure
/// # Arguments
/// * arg_string_path: string-like data type representing a relative path
/// # Examples
/// let string_path = "test/test_a/text_a_a.txt";
/// let result = match get_base_name( &string_path ) {
///     Some( string_result ) => { string_result }
///     None => { panic!( "" ) }
/// };
/// let expected = "text_a_a.txt".to_string();
/// assert_eq!( result, expected )
pub fn get_base_name<T: Debug + Display>(arg_string_path: &T) -> Option<String> {
    match Path::new(&format!("{}", arg_string_path,)).file_name() {
        Some(os_str_result) => match os_str_result.to_str() {
            Some(str_result) => Some(str_result.to_string()),
            None => return None,
        },
        None => return None,
    }
}

/// Returns a string path which is shared between all paths in the slice of string-like values
/// If the common path doesn't exist, the function will then iterate upwards through the common path's
/// ancestors until to a path is found, or no further parents exist.
/// Returns None in case of failure (either no match found, nothing about the common string exists)
/// # Arguments
/// * arg_slice_of_strings: slice of string-like paths
/// # Examples
/// let slice_of_strings = [
///     "/A/B",
///     "/A/B/C",
///     "/A/B/C/D",
/// ];
/// let result = match get_common_path( &slice_of_strings ) {
///     Ok( string_result ) => { string_result }
///     Err( err ) => { panic!( "{}", err ) }
/// };
/// // If '/A/B' exists, then that is the common shared path and the function will return this
/// // If '/A' exists, but '/A/B' is actually fictitious, then the function will continue
/// // fetching the 'parent' directory until it finds one that exists. In this scenario, that
/// // directory would be '/A'
pub fn get_common_path<T: Debug + Display>(arg_slice_of_strings: &[T]) -> Result<String, String> {
    let vec_of_path_bufs =
        get_path_bufs_sorted_by_size_starting_with_shortest(&arg_slice_of_strings);
    let mut path_buf_prefix_to_return = match vec_of_path_bufs.get(0) {
        Some(path_buf_result) => path_buf_result.clone(),
        None => {
            return Err([
                "Error: Failed to get value at index 0.".to_string(),
                format!(
                    "arg_slice_of_strings.len() = {}",
                    arg_slice_of_strings.len()
                ),
                format!(
                    "arg_slice_of_strings.len() = {:#?}",
                    arg_slice_of_strings.len()
                ),
            ]
            .join("\n"))
        }
    };
    loop {
        let bool_all_path_bufs_meet_requirement = {
            let mut bool_all_path_bufs_meet_requirement = true;
            for item_path_buf in &vec_of_path_bufs {
                if !item_path_buf.starts_with(&path_buf_prefix_to_return) {
                    bool_all_path_bufs_meet_requirement = false;
                    break;
                }
            }
            bool_all_path_bufs_meet_requirement
        };
        if bool_all_path_bufs_meet_requirement {
            match path_buf_prefix_to_return.to_str() {
                Some(_str_result) => break,
                None => {
                    return Err([
                        "Error: Failed to extract str from PathBuf.".to_string(),
                        format!(
                            "path buf value at failure = {:?}",
                            path_buf_prefix_to_return
                        ),
                        format!("arg_slice_of_strings = {:#?}", arg_slice_of_strings,),
                    ]
                    .join("\n"))
                }
            }
        } else {
            path_buf_prefix_to_return = match path_buf_prefix_to_return.parent() {
                Some(path_result) => PathBuf::from(path_result),
                None => {
                    return Err([
                        "Error: Attempted to access non-existent parent.".to_string(),
                        format!(
                            "path buf value at failure = {:?}",
                            path_buf_prefix_to_return
                        ),
                        format!("arg_slice_of_strings = {:#?}", arg_slice_of_strings,),
                    ]
                    .join("\n"))
                }
            }
        }
    }
    //
    // Keep getting parent dir until it exists
    //
    loop {
        if path_buf_prefix_to_return.exists() {
            break;
        }
        path_buf_prefix_to_return = match path_buf_prefix_to_return.parent() {
            Some(path_result) => PathBuf::from(path_result),
            None => {
                return Err([
                    "Error: Attempted to access non-existent parent.".to_string(),
                    format!(
                        "path buf value at failure = {:?}",
                        path_buf_prefix_to_return
                    ),
                    format!("arg_slice_of_strings = {:#?}", arg_slice_of_strings,),
                ]
                .join("\n"))
            }
        }
    }
    match path_buf_prefix_to_return.to_str() {
        Some(str_result) => Ok(str_result.to_string()),
        None => Err([
            "Error: Failed to extract str from PathBuf.".to_string(),
            format!(
                "path buf value at failure = {:?}",
                path_buf_prefix_to_return
            ),
            format!("arg_slice_of_strings = {:#?}", arg_slice_of_strings,),
        ]
        .join("\n")),
    }
}

/// Returns a string or None which is the same shared path represented within
/// a slice of string-like paths
/// # Arguments
/// * arg_slice_of_strings: slice of string-like paths
/// # Examples
/// let slice_of_strings = [
///     "/A/B",
///     "/A/B/C",
///     "/A/B/C/D",
/// ];
/// let result = match get_common_prefix( &slice_of_strings ) {
///     Ok( string_result ) => { string_result }
///     Err( err ) => { panic!( "{}", err, ) }
/// };
/// // If '/A/B' exists, then that is the common shared path and the function will return this,
/// // regardless of whether or not this path actually exists
pub fn get_common_prefix<T: Debug + Display>(arg_slice_of_strings: &[T]) -> Result<String, String> {
    let vec_of_path_bufs =
        get_path_bufs_sorted_by_size_starting_with_shortest(&arg_slice_of_strings);
    let mut path_buf_to_return = match vec_of_path_bufs.get(0) {
        Some(path_buf_result) => path_buf_result.clone(),
        None => {
            return Err([
                "Error: Failed to get value at index 0.".to_string(),
                format!(
                    "arg_slice_of_strings.len() = {}",
                    arg_slice_of_strings.len()
                ),
                format!(
                    "arg_slice_of_strings.len() = {:#?}",
                    arg_slice_of_strings.len()
                ),
            ]
            .join("\n"))
        }
    };
    loop {
        let bool_all_path_bufs_meet_requirement = {
            let mut bool_all_path_bufs_meet_requirement = true;
            for item_path_buf in &vec_of_path_bufs {
                if !item_path_buf.starts_with(&path_buf_to_return) {
                    bool_all_path_bufs_meet_requirement = false;
                    break;
                }
            }
            bool_all_path_bufs_meet_requirement
        };
        if bool_all_path_bufs_meet_requirement {
            break;
        }
        path_buf_to_return = match path_buf_to_return.parent() {
            Some(path_result) => PathBuf::from(path_result),
            None => {
                return Err([
                    "Error: Attempted to access non-existent parent.".to_string(),
                    format!("path buf value at failure = {:?}", path_buf_to_return),
                    format!("arg_slice_of_strings = {:#?}", arg_slice_of_strings,),
                ]
                .join("\n"))
            }
        }
    }
    match path_buf_to_return.to_str() {
        Some(str_result) => Ok(str_result.to_string()),
        None => {
            return Err([
                "Error: Failed to extract str from PathBuf.".to_string(),
                format!("path buf value at failure = {:?}", path_buf_to_return),
                format!("arg_slice_of_strings = {:#?}", arg_slice_of_strings,),
            ]
            .join("\n"))
        }
    }
}

/// Returns a string or None which is the path n-layers up
/// # Arguments
/// * arg_string_path: string-like path
/// * arg_n: usize-type value representing the number of layers to iterate through
/// # Examples
/// let result = match get_dir_ancestor_n_levels_up( &"/A/B/C", 2 ) {
///     Some( string_result ) => { string_result }
///     None => { panic!( "Failed to get ancestor" ) }
/// };
/// // result = "/A"
pub fn get_dir_ancestor_n_levels_up<T: Debug + Display>(
    arg_string_path: &T,
    arg_n: usize,
) -> Option<String> {
    let mut path_buf = PathBuf::from(format!("{}", arg_string_path,));
    for _ in 0..arg_n {
        path_buf = match path_buf.parent() {
            Some(path_result) => PathBuf::from(path_result),
            None => return None,
        };
    }
    match path_buf.to_str() {
        Some(str_result) => Some(str_result.to_string()),
        None => None,
    }
}

/// Returns a string or None which is the part of the argument path that actually exists
/// # Arguments
/// * arg_string_path: string-like path
/// # Examples
/// let result = match get_dir_ancestor_that_exists( &"/A/B/C" ) {
///     Some( string_result ) => { string_result }
///     None => { panic!( "Failed to get ancestor" ) }
/// };
/// // The returned string will be "/A/B/C" if it exists
/// // The returned string will be "/A/B" if it exists and "/A/B/C" does not
/// // The returned string will be "/A" if all in-between paths do not exist
pub fn get_dir_ancestor_that_exists<T: Debug + Display>(arg_string_path: &T) -> Option<String> {
    let mut path_buf = PathBuf::from(format!("{}", arg_string_path,));
    loop {
        if path_buf.exists() {
            return match path_buf.to_str() {
                Some(str_result) => Some(str_result.to_string()),
                None => None,
            };
        } else {
            path_buf = match path_buf.parent() {
                Some(path_result) => PathBuf::from(path_result),
                None => return None,
            };
        }
    }
}

/// Returns a string or None which is the path to the current working directory
/// # Examples
/// let result = match get_dir_cwd() {
///     Ok( string_result ) => { string_result }
///     Err( err ) => { panic!( "{}", err, ) }
/// };
pub fn get_dir_cwd() -> Result<String, String> {
    match std::env::current_dir() {
        Ok(path_buf_from_env) => match path_buf_from_env.to_str() {
            Some(str_result) => Ok(str_result.to_string()),
            None => {
                return Err([
                    "Error: could not get str from PathBuf.".to_string(),
                    format!("path_buf_from_env = {:?}", path_buf_from_env,),
                ]
                .join("\n"))
            }
        },
        Err(err) => {
            return Err([
                "Error: could not get PathBuf from std::env::current_dir().".to_string(),
                format!("err = {}", err,),
                format!("std::env::current_dir() = {:?}", std::env::current_dir(),),
            ]
            .join("\n"))
        }
    }
}

/// Returns a string or None which is the path to the current working directory
/// # Arguments
/// * arg_string_path: string-like
/// # Examples
/// let result = match get_dir_name( &"/A/B/C" ) {
///     Some( string_result ) => { string_result }
///     None => { panic!( "Failed to get dirname." ) }
/// };
/// // result = "/A/B"
pub fn get_dir_name<T: Debug + Display>(arg_string_path: &T) -> Option<String> {
    match PathBuf::from(format!("{}", arg_string_path,)).parent() {
        Some(path_result) => match path_result.to_str() {
            Some(str_result) => Some(str_result.to_string()),
            None => None,
        },
        None => None,
    }
}

/// Returns a string representing the path to the project root directory
/// # Examples
/// let result = get_dir_proj_root();
pub fn get_dir_proj_root() -> String {
    env!("CARGO_MANIFEST_DIR").to_string()
}

/// Returns a string representing the file extension without the period
/// # Arguments
/// * arg_string_path: string-like
/// # Examples
/// let result = match get_extension( "file.txt" ) {
///     Some( string_result ) => { string_result }
///     None => { panic!( "Failed to get extension." ) }
/// };
/// // result = "txt"
pub fn get_extension<T: Debug + Display>(arg_string_path: &T) -> Option<String> {
    match Path::new(&format!("{}", arg_string_path,)).extension() {
        Some(os_str_result) => match os_str_result.to_str() {
            Some(str_result) => Some(str_result.to_string()),
            None => return None,
        },
        None => return None,
    }
}

/// Returns a string path pointing at the binary file created by the compilation process
/// # Examples
/// let string_path_file_binary = match get_file_path_binary() {
///     Ok( string_result ) => { string_result }
///     Err( err ) => { panic!( "{}", err, ) }
/// };
pub fn get_file_path_binary() -> Result<String, String> {
    match std::env::current_exe() {
        Ok(path_buf_result) => match path_buf_result.to_str() {
            Some(str_result) => Ok(str_result.to_string()),
            None => Err([
                "Error: failed to convert path to binary file from PathBuff to str".to_string(),
                format!("path_buf_from_current_exe = {:?}", path_buf_result,),
            ]
            .join("\n")),
        },
        Err(err) => Err([
            "Error: failed to get path to binary file outputted by compilation process".to_string(),
            format!("err = {}", err,),
        ]
        .join("\n")),
    }
}

/// Returns a vec of only strings referencing directories from a slice argument
/// # Arguments
/// * arg_slice_of_strings: [] of string-likes
/// # Examples
/// let slice_of_strings = [
///     "test/test_b",
///     "test/test_b/test_c",
///     "test/test_b/test_c/text_b_c_b.txt",
///     "test/test_b/test_c/text_b_c_a.txt",
///     "test/test_a",
///     "test/test_a/text_a_a.txt",
/// ];
/// let result = get_only_dirs_from_slice( &slice_of_strings );
/// let expected = [
///     "test/test_b",
///     "test/test_b/test_c",
///     "test/test_a",
/// ].iter().map( | item | { format!( "{}", item, ) } ).collect::<Vec<String>>();
/// assert_eq!( result, expected, )
pub fn get_only_dirs_from_slice<T: Debug + Display>(arg_slice_of_strings: &[T]) -> Vec<String> {
    arg_slice_of_strings
        .iter()
        .map(|item| format!("{}", item))
        .filter(|item_string| PathBuf::from(item_string).is_dir())
        .collect::<Vec<String>>()
}

/// Returns a vec of only strings referencing files from a slice argument
/// # Arguments
/// * arg_slice_of_strings: [] of string-likes
/// # Examples
/// let slice_of_strings = [
///     "test/test_b",
///     "test/test_b/test_c",
///     "test/test_b/test_c/text_b_c_b.txt",
///     "test/test_b/test_c/text_b_c_a.txt",
///     "test/test_a",
///     "test/test_a/text_a_a.txt",
/// ];
/// let result = get_only_file_paths_from_slice( &slice_of_strings );
/// let expected = [
///     "test/test_b/test_c/text_b_c_b.txt",
///     "test/test_b/test_c/text_b_c_a.txt",
///     "test/test_a/text_a_a.txt",
/// ].iter().map( | item | { format!( "{}", item, ) } ).collect::<Vec<String>>();
/// assert_eq!( result, expected, )
pub fn get_only_file_paths_from_slice<T: Debug + Display>(
    arg_slice_of_strings: &[T],
) -> Vec<String> {
    arg_slice_of_strings
        .iter()
        .map(|item| format!("{}", item))
        .filter(|item_string| PathBuf::from(item_string).is_file())
        .collect::<Vec<String>>()
}

/// Returns a string path that is the result of combining a slice of string-like values
/// In case of a failure, this returns None
/// # Arguments
/// * arg_slice_of_strings: slice of string-likes
/// # Examples
/// let string_path = match get_path_joined( &[ "/A", "B", "C" ] ) {
///     Some( string_result ) => { string_result }
///     None => { panic!( "Failed to join strings into a path." ) }
/// };
/// string_path = "/A/B/C"
pub fn get_path_joined<T: Debug + Display>(arg_slice_of_strings: &[T]) -> Option<String> {
    match arg_slice_of_strings
        .iter()
        .map(|item| format!("{}", item,))
        .collect::<PathBuf>()
        .to_str()
    {
        Some(str_result) => Some(str_result.to_string()),
        None => None,
    }
}

/// Returns a string with the '~' expanded within the path
/// # Arguments
/// * arg_string_path: this is a string-like reference
/// # Examples
/// let string_path = get_path_with_tilde_expanded_if_necessary( "~/test" );
/// // string_path = '/Users/<user>/test
pub fn get_path_with_tilde_expanded_if_necessary<T: Debug + Display>(
    arg_string_path: &T,
) -> String {
    let mut string_path = format!("{}", arg_string_path,);
    if string_path.starts_with("~") {
        string_path = [
            shellexpand::tilde("~").to_string(),
            string_path.substring(1, string_path.len()).to_string(),
        ]
        .join("")
    }
    string_path
}

/// Returns a vec of string paths inside directory
/// In case of a failure, this returns an error explaining what happened
/// # Arguments
/// * arg_string_path: string-like path
/// # Examples
/// let string_path = "test";
/// let result = match get_paths_in_dir( &string_path ) {
///     Ok( vec_result ) => vec_result,
///     Err( err ) => panic!( "{}", err, )
/// };
/// let expected = [
///     "test/test_a",
///     "test/test_b",
/// ].iter().map( | item_str | { item_str.to_string() } ).collect::<Vec<String>>();
/// assert_eq!( result, expected )
pub fn get_paths_in_dir<T: Debug + Display>(arg_string_path: &T) -> Result<Vec<String>, String> {
    let slice_of_read_dirs = match std::fs::read_dir(&format!("{}", arg_string_path,)) {
        Ok(data_from_read_dir) => data_from_read_dir,
        Err(err) => {
            return Err([
                "Error: failed to read directory.".to_string(),
                format!("err = {}", err,),
                format!("arg_string_path_dir = {}", &arg_string_path,),
            ]
            .join("\n"))
        }
    };
    //
    // Convert slice_of_read_dirs to a vec of strings
    //
    let mut vec_to_return = vec![];
    for item_dir_entry_result in slice_of_read_dirs {
        vec_to_return.push(match &item_dir_entry_result {
            Ok(item_dir_entry_from_result) => match item_dir_entry_from_result.path().to_str() {
                Some(str_from_path_buf_to_return) => str_from_path_buf_to_return.to_string(),
                None => {
                    return Err([
                        "Error: failed to extract str from arg_path_buf".to_string(),
                        format!(
                            "item_dir_entry_from_result = {:?}",
                            &item_dir_entry_from_result,
                        ),
                    ]
                    .join("\n"))
                }
            },
            Err(err) => {
                return Err([
                    "Error: failed to extract item_dir_entry".to_string(),
                    format!("err = {}", err,),
                    format!("item_dir_entry_result = {:?}", &item_dir_entry_result,),
                ]
                .join("\n"))
            }
        })
    }
    Ok(vec_to_return)
}

/// Returns a vec of string paths inside directory
/// In case of a failure, this returns an error explaining what happened
/// # Arguments
/// * arg_string_path: string-like path
/// # Examples
/// let string_path = "test";
/// let result = match get_paths_in_dir_and_sub_dirs( &string_path ) {
///     Ok( vec_result ) => vec_result,
///     Err( err ) => panic!( "{}", err )
/// };
/// let expected = [
///     "test/test_b",
///     "test/test_b/test_c",
///     "test/test_b/test_c/text_b_c_b.txt",
///     "test/test_b/test_c/text_b_c_a.txt",
///     "test/test_a",
///     "test/test_a/text_a_a.txt",
/// ].iter().map( | item_str | { item_str.to_string() } ).collect::<Vec<String>>();
/// assert_eq!( result, expected )
pub fn get_paths_in_dir_and_sub_dirs<T: Debug + Display>(
    arg_string_path: &T,
) -> Result<Vec<String>, String> {
    let slice_of_read_dirs = match std::fs::read_dir(&format!("{}", arg_string_path,)) {
        Ok(data_from_read_dir) => data_from_read_dir,
        Err(err) => {
            return Err([
                "Error: failed to read directory.".to_string(),
                format!("err = {}", err,),
                format!("arg_string_path = {}", &arg_string_path,),
            ]
            .join("\n"))
        }
    };
    let mut vec_of_string_paths_files_contained_in_dir = vec![];
    for item_dir_entry_result in slice_of_read_dirs {
        vec_of_string_paths_files_contained_in_dir.push(match &item_dir_entry_result {
            Ok(item_dir_entry) => match item_dir_entry.path().to_str() {
                Some(str_from_path_buf) => str_from_path_buf.to_string(),
                None => {
                    return Err([
                        "Error: failed to extract str from arg_path_buf".to_string(),
                        format!("item_dir_entry = {:?}", &item_dir_entry,),
                    ]
                    .join("\n"))
                }
            },
            Err(err) => {
                return Err([
                    "Error: failed to extract item_dir_entry".to_string(),
                    format!("err = {}", err,),
                    format!("item_dir_entry_result = {:?}", &item_dir_entry_result,),
                ]
                .join("\n"))
            }
        })
    }
    //
    // Iterate through sub paths
    //
    let mut stack_of_dir_entries_to_process = Vec::from(vec_of_string_paths_files_contained_in_dir);
    //
    // Iterate over sub-paths and make sure nothing broke
    //
    let mut vec_to_return: Vec<String> = Vec::new();
    loop {
        match stack_of_dir_entries_to_process.pop() {
            Some(item_string_path_dir) => {
                //
                // Create string copy to push to vec
                //
                vec_to_return.push(item_string_path_dir.clone());
                //
                // Prep next iteration
                //
                let metadata_from_path = match std::fs::metadata(&item_string_path_dir) {
                    Ok(metadata_extracted) => metadata_extracted,
                    Err(err) => {
                        return Err([
                            "Error: failed to get meta data from arg_string_path.".to_string(),
                            format!("err = {:?}", err,),
                            format!("arg_string_path = {}", arg_string_path,),
                        ]
                        .join("\n"))
                    }
                };
                if metadata_from_path.is_dir() {
                    stack_of_dir_entries_to_process.extend({
                        let string_path_dir = format!("{}", item_string_path_dir,);
                        let read_dir_results = match std::fs::read_dir(&string_path_dir) {
                            Ok(data_from_read_dir) => data_from_read_dir,
                            Err(err) => {
                                return Err([
                                    "Error: failed to read directory.".to_string(),
                                    format!("err = {}", err,),
                                    format!("string_path_dir = {}", &string_path_dir,),
                                ]
                                .join("\n"))
                            }
                        };
                        let mut vec_of_results = vec![];
                        for item_dir_entry_result in read_dir_results {
                            //
                            // Get string path from dir entry result
                            //
                            vec_of_results.push(match &item_dir_entry_result {
                                Ok(item_dir_entry_from_result) => {
                                    match item_dir_entry_from_result.path().to_str() {
                                        Some(str_from_path_buf) => str_from_path_buf.to_string(),
                                        None => {
                                            return Err([
                                                "Error: failed to extract str from arg_path_buf"
                                                    .to_string(),
                                                format!(
                                                    "item_dir_entry_from_result.path() = {:?}",
                                                    &item_dir_entry_from_result.path(),
                                                ),
                                            ]
                                            .join("\n"))
                                        }
                                    }
                                }
                                Err(err) => {
                                    return Err([
                                        "Error: failed to extract item_dir_entry".to_string(),
                                        format!("err = {}", err,),
                                        format!(
                                            "item_dir_entry_result = {:?}",
                                            &item_dir_entry_result,
                                        ),
                                    ]
                                    .join("\n"))
                                }
                            })
                        }
                        vec_of_results
                    });
                }
            }
            //
            // Exit loop when we're out of sub directories
            //
            None => break,
        }
    }
    //
    // Return vec
    //
    Ok(vec_to_return)
}

/// Returns a vec of string paths, sorted by their depth, with the shortest first
/// # Arguments
/// * arg_slice_of_strings: [] of string-likes
/// # Examples
/// let slice_of_strings = [
///     "/A/B/C",
///     "/A",
///     "/A/B",
/// ];
/// let vec_of_strings = get_paths_sorted_by_size_starting_with_shortest( &slice_of_strings );
/// // vec_of_strings = [
/// //  "/A",
/// //  "/A/B",
/// //  "/A/B/C",
/// // ]
pub fn get_paths_sorted_by_size_starting_with_shortest<T: Debug + Display>(
    arg_slice_of_strings: &[T],
) -> Result<Vec<String>, String> {
    let vec_of_path_bufs =
        get_path_bufs_sorted_by_size_starting_with_shortest(&arg_slice_of_strings);
    let mut vec_to_return = vec![];
    for item_path_buf in vec_of_path_bufs {
        match item_path_buf.to_str() {
            Some(str_result) => vec_to_return.push(str_result.to_string()),
            None => {
                return Err([
                    "Error: failed to extract str from PathBuf".to_string(),
                    format!("item_path_buf = {:?}", item_path_buf,),
                ]
                .join("\n"))
            }
        }
    }
    Ok(vec_to_return)
}

/// Returns a vec of string paths inside directory
/// In case of a failure, this returns an error explaining what happened
/// # Arguments
/// * arg_string_path: string-like path
/// # Examples
/// let string_path = "test";
/// let result = match get_paths_to_only_dirs_in_dir_and_sub_dirs( &string_path ) {
///     Ok( vec_result ) => vec_result,
///     Err( err ) => panic!( "{}", err, )
/// };
/// let expected = [
///     "test/test_b",
///     "test/test_b/test_c",
///     "test/test_a",
/// ].iter().map(|item_str| {item_str.to_string()}).collect::<Vec<String>>();
/// assert_eq!( result, expected )
pub fn get_paths_to_only_dirs_in_dir_and_sub_dirs<T: Display>(
    arg_string_path_dir: &T,
) -> Result<Vec<String>, String> {
    Ok(
        match get_paths_in_dir_and_sub_dirs(&format!("{}", arg_string_path_dir,)) {
            Ok(result) => result,
            Err(err) => return Err(err),
        }
        .iter()
        .filter(|item_string_path| Path::new(&format!("{}", item_string_path,)).is_dir())
        .map(|item_string_path| item_string_path.clone())
        .collect::<Vec<String>>(),
    )
}

/// Returns a vec of string paths inside directory
/// In case of a failure, this returns an error explaining what happened
/// # Arguments
/// * arg_string_path: string-like path
/// # Examples
/// let string_path = "test";
/// let result = match get_paths_to_only_files_in_dir_and_sub_dirs( &string_path ) {
///     Ok( vec_result ) => { vec_result }
///     Err( err ) => { panic!( "{}", err, ) }
/// };
/// let expected = [
///     "test/test_b/test_c/text_b_c_b.txt",
///     "test/test_b/test_c/text_b_c_a.txt",
///     "test/test_a/text_a_a.txt",
/// ].iter().map( | item_str | { item_str.to_string() } ).collect::<Vec<String>>();
/// assert_eq!( result, expected )
pub fn get_paths_to_only_files_in_dir_and_sub_dirs<T: Display>(
    arg_string_path_dir: &T,
) -> Result<Vec<String>, String> {
    Ok(
        match get_paths_in_dir_and_sub_dirs(&format!("{}", arg_string_path_dir,)) {
            Ok(result) => result,
            Err(err) => return Err(err),
        }
        .iter()
        .filter(|item_string_path| Path::new(&format!("{}", item_string_path,)).is_file())
        .map(|item_string_path| item_string_path.clone())
        .collect::<Vec<String>>(),
    )
}

/// Returns a string that's a relative path after the prefix is removed
/// In case of a failure, this returns an error explaining what happened
/// # Arguments
/// * arg_string_path: string-like path
/// # Examples
/// let string_relative_path = match get_relative_path( &"/A/B/C/D", &"/A/B" ) {
///     Ok( string_result ) => { string_result }
///     Err( err ) => { panic!( "{}", err, ) }
/// };
/// // string_relative_path = "C/D"
pub fn get_relative_path<T1: Debug + Display, T2: Debug + Display>(
    arg_string_path: &T1,
    arg_string_path_root_prefix: &T2,
) -> Result<String, String> {
    match PathBuf::from(format!("{}", arg_string_path,))
        .strip_prefix(&PathBuf::from(format!("{}", arg_string_path_root_prefix,)))
    {
        Ok(path_result) => match path_result.to_str() {
            Some(str_result) => Ok(str_result.to_string()),
            None => {
                return Err([
                    "Error: failed getting str from path.".to_string(),
                    format!("path_relative (datatype: Path) = {:?}", path_result,),
                ]
                .join("\n"))
            }
        },
        Err(err) => {
            return Err([
                "Error: strip_prefix() failed.".to_string(),
                format!("err = {}", err,),
                format!("arg_string_path = {}", arg_string_path,),
                format!(
                    "arg_string_path_root_prefix = {}",
                    arg_string_path_root_prefix,
                ),
            ]
            .join("\n"))
        }
    }
}

/// Returns a vec resulting from splitting the path into substrings
/// Returns None in case of failure
/// # Arguments
/// * arg_string_path: a string-like path
/// # Examples
/// let vec_of_substrings = match get_vec_by_splitting_path( &"/A/B/C/D" ) {
///     Some( vec_result ) => { vec_result }
///     None => { panic!( "Failed to split path." ) }
/// };
/// // vec_of_substrings = [ "A", "B", "C", "D" ]
pub fn get_vec_by_splitting_path<T: Debug + Display>(arg_string_path: &T) -> Option<Vec<String>> {
    let mut vec_to_return = vec![];
    for item_os_str in PathBuf::from(format!("{}", arg_string_path,)).iter() {
        match item_os_str.to_str() {
            Some(str_result) => vec_to_return.push(str_result.to_string()),
            None => return None,
        }
    }
    Some(vec_to_return)
}
//
// Public - ( logic ) are / is
//
/// Returns true if both paths are pointing to the same dir / file on the disk
/// If relative paths are used, this fetches the cwd.
/// This assumes if a relative path is passed, the root dir of the working area is the project
/// # Arguments
/// * arg_string_path_left: string-like path to compare
/// * arg_string_right: string-like path to compare
/// * arg_string_path_working_dir: This serves as the 'root working directory' if relative paths are used
/// # Examples
/// let result = are_paths_the_same( &"/<project dir>/test", &"test" );
/// // result = true
pub fn are_paths_the_same<T1: Debug + Display, T2: Debug + Display, T3: Debug + Display>(
    arg_string_path_left: &T1,
    arg_string_path_right: &T2,
    arg_string_path_working_dir: &T3,
) -> bool {
    let path_buf_working_dir = PathBuf::from(format!("{}", arg_string_path_working_dir,));
    let path_buf_left = {
        let path_buf_from_arg = PathBuf::from(format!("{}", arg_string_path_left,));
        if path_buf_from_arg.is_absolute() {
            path_buf_from_arg
        } else {
            [&path_buf_working_dir, &path_buf_from_arg]
                .iter()
                .collect::<PathBuf>()
        }
    };
    let path_buf_right = {
        let path_buf_from_arg = PathBuf::from(format!("{}", arg_string_path_right,));
        if path_buf_from_arg.is_absolute() {
            PathBuf::from(format!("{}", arg_string_path_right,))
        } else {
            [&path_buf_working_dir, &path_buf_from_arg]
                .iter()
                .collect::<PathBuf>()
        }
    };
    path_buf_left == path_buf_right
}

/// Returns true if both paths are pointing to the same dir / file on the disk
/// If relative paths are used, this assumes the working directory is cwd
/// Due to the cwd fetch's potential for errors, this function requires unpacking
/// # Arguments
/// * arg_string_path: string-like path
/// # Examples
/// let result = match are_paths_the_same( &"/<project dir>/test", &"test" ) {
///     Ok( bool_result ) => { bool_result }
///     Err( err ) => { panic!( "{}", err, ) }
/// };
/// // result = true
pub fn are_paths_the_same_assume_cwd<T1: Debug + Display, T2: Debug + Display>(
    arg_string_path_left: &T1,
    arg_string_path_right: &T2,
) -> Result<bool, String> {
    Ok(are_paths_the_same(
        &arg_string_path_left,
        &arg_string_path_right,
        &match get_dir_cwd() {
            Ok(string_result) => string_result,
            Err(err) => return Err(err),
        },
    ))
}

/// Returns true if both paths are pointing to the same dir / file on the disk
/// If relative paths are used, this fetches the cwd.
/// If a relative path is used, this assumes the working directory is the project's root
/// # Arguments
/// * arg_string_path: string-like path
/// # Examples
/// let result = are_paths_the_same( &"/<project dir>/test", &"test" );
/// // result = true
pub fn are_paths_the_same_assume_project_dir<T1: Debug + Display, T2: Debug + Display>(
    arg_string_path_left: &T1,
    arg_string_path_right: &T2,
) -> bool {
    are_paths_the_same(
        &arg_string_path_left,
        &arg_string_path_right,
        &env!("CARGO_MANIFEST_DIR"),
    )
}

/// Returns true if the path argument has a parent, and false if not
/// # Arguments
/// * arg_string_path: string-like
/// # Examples
/// let result = has_parent( "/A/B/C" )
/// // result = true
pub fn has_parent<T: Debug + Display>(arg_string_path: &T) -> bool {
    match PathBuf::from(format!("{}", arg_string_path)).parent() {
        Some(_result) => true,
        None => false,
    }
}

/// Returns true if the string is an absolute path
/// # Arguments
/// * arg_string_path: string-like
/// # Examples
/// let result = is_absolute( &"/A/B/C" );
/// assert_eq!( result, true, );
pub fn is_absolute<T: Debug + Display>(arg_string_path: &T) -> bool {
    PathBuf::from(format!("{}", arg_string_path,)).is_absolute()
}

/// Returns bool is path is a directory
/// # Arguments
/// * arg_string_path: a string-like path
/// # Examples
/// let result = is_dir( &"/A/B/C/D" );
pub fn is_dir<T: Debug + Display>(arg_string_path: &T) -> bool {
    return PathBuf::from(format!("{}", arg_string_path,)).is_dir();
}

/// Returns bool is path is a directory
/// # Arguments
/// * arg_string_path: string-like path
/// # Examples
/// let result = is_existing_path( &"/A/B/C/D" );
pub fn is_existing_path<T: Debug + Display>(arg_string_path: &T) -> bool {
    return PathBuf::from(format!("{}", arg_string_path,)).exists();
}

/// Returns bool is path is a directory
/// # Arguments
/// * arg_string_path: string-like path
/// # Examples
/// let result = is_file( &"/A/B/C/D" );
pub fn is_file<T: Debug + Display>(arg_string_path: &T) -> bool {
    return PathBuf::from(format!("{}", arg_string_path,)).is_file();
}

/// Returns true if arg is type Path
/// # Arguments
/// * arg: any data type
/// # Examples
/// let result = is_path( &"/A/B/C/D" );
pub fn is_path_type<T>(_arg: &T) -> bool {
    std::any::type_name::<T>()
        .to_string()
        .ends_with("std::path::Path")
}

/// Returns true if arg is type PathBuf
/// # Arguments
/// * arg: any data type
/// # Examples
/// let result = is_path_buf( &"/A/B/C/D" );
pub fn is_path_buf_type<T>(_arg: &T) -> bool {
    std::any::type_name::<T>()
        .to_string()
        .ends_with("std::path::PathBuf")
}

/// Returns true if arg_string_path is inside arg_string_dir_parent
/// # Arguments
/// * arg_string_path: string-like
/// * arg_string_dir_parent: string-like
/// # Examples
/// let result = is_path_inside_dir_parent( &"/A/B/C", &"/A/B/C/D" );
pub fn is_path_inside_dir_parent<T1: Display, T2: Display>(
    arg_string_path: &T1,
    arg_string_dir_parent: &T2,
) -> bool {
    Path::new(&format!("{}", arg_string_path,)).starts_with(format!("{}", arg_string_dir_parent,))
}
//
// Public - raise error
//
/// Returns an error if one is detected. Otherwise, this returns None
/// # Argument
/// * arg_string_path: string-like
/// # Examples
/// let err = match raise_error_if_path_does_not_exist( &"/A/B/C" ) {
///     Ok( () ) => {},
///     Err( err ) => { panic!( "{:?}", err, ) }
/// };
pub fn raise_error_if_path_does_not_exist<T: Debug + Display>(
    arg_string_path: &T,
) -> Result<(), String> {
    if !Path::new(format!("{}", arg_string_path,).as_str()).exists() {
        return Err(
            [
                "Error: path does not exist.".to_string(),
                format!("arg_string_path = {}", arg_string_path,),
                match get_dir_ancestor_that_exists(&arg_string_path) {
                    Some(string_result) => {
                        format!("ancestor that actually exists = {}", string_result,)
                    }
                    None => format!("No existing ancestor exists."),
                },
            ]
            .join("\n"),
        );
    }
    Ok(())
}

/// Returns an error if arg_string_path points to a location outside the project.
/// Otherwise, this returns None.
/// # Arguments
/// * arg_string_path: string-like
/// # Examples
/// let err = match raise_error_if_path_is_not_in_project( &"/A/B/C" ) {
///     Ok( () ) => {}
///     Err( err ) => { panic!( "{:?}", err, ) }
/// };
pub fn raise_error_if_path_is_not_in_project<T: Debug + Display>(
    arg_string_path: &T,
) -> Result<(), String> {
    let path_buf_control = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let bool_raise_error = {
        let path_buf_from_arg = PathBuf::from(format!("{}", arg_string_path,));
        if path_buf_from_arg.is_absolute() {
            !path_buf_from_arg.starts_with(&path_buf_control)
        } else {
            ![&path_buf_control, &path_buf_from_arg]
                .iter()
                .collect::<PathBuf>()
                .exists()
        }
    };
    if bool_raise_error {
        return Err(
            [
                "Error: arg_string_path is either the project directory or outside it.".to_string(),
                format!("arg_string_path = {}", arg_string_path,),
                format!("path_buf_control = {:?}", path_buf_control,),
            ]
            .join("\n"),
        );
    }
    Ok(())
}

/// Returns an error if arg_string_path points to project directory.
/// Otherwise, this returns None.
/// # Arguments
/// * arg_string_path: string-like
/// # Examples
/// let err = match raise_error_if_path_points_to_project_root( &"/A/B/C" ) {
///     Ok( () ) => {},
///     Err( err ) => { panic!( "{:?}", err, ) }
/// };
pub fn raise_error_if_path_points_to_project_root<T: Display>(
    arg_string_path: &T,
) -> Result<(), String> {
    if Path::new(&format!("{}", arg_string_path,))
        == Path::new(&env!("CARGO_MANIFEST_DIR").to_string())
    {
        return Err(
            [
                "Error: arg_string_path points at project root directory.".to_string(),
                format!("arg_string_path = {}", arg_string_path,),
            ]
            .join("\n"),
        );
    }
    Ok(())
}

/// Returns an error if arg_string_path points to src within project.
/// Otherwise, this returns None.
/// If the path is relative, then the
/// # Arguments
/// * arg_string_path: string-like
/// # Examples
/// let err = match raise_error_if_path_points_to_src( &"/A/B/C" ) {
///     Ok( () ) => {}
///     Err( err ) => { panic!( "{:?}", err, ) }
/// };
pub fn raise_error_if_path_points_to_src<T: Debug + Display>(
    arg_string_path: &T,
) -> Result<(), String> {
    if are_paths_the_same_assume_project_dir(arg_string_path, &"src") {
        return Err(
            [
                "Error: arg_string_path points at the src directory.".to_string(),
                format!("arg_string_path = {}", arg_string_path,),
            ]
            .join("\n"),
        );
    }
    Ok(())
}

/// Returns an error if arg_string_path points to Cargo.toml within project.
/// Otherwise, this returns None.
/// # Arguments
/// * arg_string_path: string-like
/// # Examples
/// let err = match raise_error_if_path_points_to_cargo_toml( &"/A/B/C" ) {
///     Ok( () ) => {},
///     Err( err ) => { panic!( "{:?}", err, ) }
/// };
pub fn raise_error_if_path_points_to_cargo_toml<T: Debug + Display>(
    arg_string_path: &T,
) -> Result<(), String> {
    if are_paths_the_same_assume_project_dir(arg_string_path, &"Cargo.toml") {
        return Err(
            [
                "Error: arg_string_path points at Cargo.toml.".to_string(),
                format!("arg_string_path = {}", &arg_string_path,),
            ]
            .join("\n"),
        );
    }
    Ok(())
}

/// Returns an error if arg_string_path points at main.rs in the project.
/// Otherwise, this returns None.
/// # Arguments
/// * arg_string_path: string-like
/// # Examples
/// let err = match raise_error_if_path_points_to_main_rs( &"/A/B/C" ) {
///     Ok( ()) ) => {}
///     Err( err ) => { panic!( "{:?}", err, ) }
/// };
pub fn raise_error_if_path_points_to_main_rs<T: Debug + Display>(
    arg_string_path: &T,
) -> Result<(), String> {
    if are_paths_the_same_assume_project_dir(arg_string_path, &"src/main.rs") {
        return Err(
            [
                "Error: arg_string_path points at main.rs.".to_string(),
                format!("arg_string_path = {}", arg_string_path,),
            ]
            .join("\n"),
        );
    }
    Ok(())
}
//
// Public - get - from type
//
// Reminder: Don't need tests for these since they are straight type conversions.
//
/// Returns PathBuf created from a string-like argument
/// # Arguments
/// * arg_string: string-like
/// # Examples
/// let path_buf = get_path_buf_from_type_string( &str_or_string );
pub fn get_path_buf_from_type_string<T: Debug + Display>(arg_string: &T) -> PathBuf {
    PathBuf::from(format!("{}", arg_string,))
}

/// Returns a String extracted from DirEntry
/// In case of failure, this returns None
/// # Arguments
/// * arg_dir_entry: argument of type: &DirEntry
/// # Examples
/// let string_result = match get_string_from_type_dir_entry( &dir_entry ) {
///     Some( string_result ) => { string_result }
///     None => { panic!( "Failed to get string from DirEntry." ) }
/// };
pub fn get_string_from_type_dir_entry(arg_dir_entry: &DirEntry) -> Option<String> {
    match arg_dir_entry.path().to_str() {
        Some(str_result) => Some(str_result.to_string()),
        None => None,
    }
}

/// Returns String extracted from OsStr
/// Returns None in case of failure
/// # Arguments
/// * arg_osstr: argument of type: &OsStr
/// # Examples
/// let string_result = match get_string_from_type_dir_entry( &os_str ) {
///     Some( string_result ) => { string_result }
///     None => { panic!( "Failed to get string from OsStr." ) }
/// };
pub fn get_string_from_type_osstr(arg_osstr: &OsStr) -> Option<String> {
    match arg_osstr.to_str() {
        Some(str_result) => Some(str_result.to_string()),
        None => None,
    }
}

/// Returns String extracted from Path
/// Returns None in case of failure
/// # Arguments
/// * arg_path: argument of type: &Path
/// # Examples
/// let string_result = match get_string_from_type_dir_entry( &path_variable ) {
///     Some( string_result ) => { string_result }
///     None => { panic!( "Failed to get string from Path." ) }
/// };
pub fn get_string_from_type_path(arg_path: &Path) -> Option<String> {
    match arg_path.to_str() {
        Some(str_result) => Some(str_result.to_string()),
        None => None,
    }
}

/// Returns String extracted from PathBuf
/// Returns None in case of failure
/// # Arguments
/// * arg_path_buf: argument of type: &PathBuf
/// # Examples
/// let string_result = match get_string_from_type_dir_entry( &path_buf ) {
///     Some( string_result ) => { string_result }
///     None => { panic!( "Failed to get string from PathBuf." ) }
/// };
pub fn get_string_from_type_path_buf(arg_path_buf: &PathBuf) -> Option<String> {
    match arg_path_buf.to_str() {
        Some(str_result) => Some(str_result.to_string()),
        None => None,
    }
}
//
// Private
//
fn get_path_bufs_sorted_by_size_starting_with_shortest<T: Debug + Display>(
    arg_slice_of_strings: &[T],
) -> Vec<PathBuf> {
    let mut vec_of_path_bufs = arg_slice_of_strings
        .iter()
        .map(|item| PathBuf::from(format!("{}", item)))
        .collect::<Vec<PathBuf>>();
    vec_of_path_bufs.sort_by(|item_path_buf_left, item_path_buf_right| {
        item_path_buf_left.cmp(item_path_buf_right)
    });
    vec_of_path_bufs
}
