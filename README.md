# easy_paths

Convenience library for talented devs on tight schedules.

This library exists to streamline the data-type juggling normally associated with disk pathing by accepting generic
strings ( str / String ) and returning String types.

## Github

https://github.com/jrhazlett/easy_paths

## Rust crate

https://crates.io/crates/easy_paths

## Warnings

This library is tested on mac and linux.

This is completely *untested* on Windows.

## License: MIT

## Usage examples

All arguments can be of type 'str' or 'String'.

### get_absolute_path

    let string_path = "./test/test_b/test_c/../";
    assert_eq!( easy_paths::get_absolute_path( &string_path ), format!( "{}/test/test_b", env!( "CARGO_MANIFEST_DIR" ) ), );
    
    let string_result = easy_paths::get_absolute_path( &"~/test" );
    if string_result.contains( "~" ) { panic!( "Result contains tilde." ) }
    if !string_result.ends_with( "/test" ) { panic!( "Result doesn't end with '/test'." ) }

### get_absolute_path_or_error

    let string_path = "./test/test_b/test_c/../";
    let result = match easy_paths::get_absolute_path_or_error( &string_path ) {
        Ok( string_result ) => string_result,
        Err( err ) => panic!( "{}", err, )
    };
    let expected = format!( "{}/test/test_b", env!( "CARGO_MANIFEST_DIR" ).to_string(), );
    assert_eq!( result, expected )

### get_base_name_with_extension

    let string_path = "test/test_a/text_a_a.txt";
    let result = match easy_paths::get_base_name( &string_path ) {
        Some( string_result ) => string_result,
        None => panic!( "" )
    };
    let expected = "text_a_a.txt".to_string();
    assert_eq!( result, expected )

### get_base_name_on_dir

    let string_path = "test/test_b/test_c/text_b_c_a.txt";
    let result = match easy_paths::get_base_name( &string_path ) {
        Some( string_result ) => string_result,
        None => panic!( "" )
    };
    let expected = "text_b_c_a.txt".to_string();
    assert_eq!( result, expected )

### get_common_path

    let slice_of_strings = [
        "src/helpers_disk/A/B/C",
        "src/helpers_disk/A/B",
        "src/helpers_disk/A",
    ];
    let result = match easy_paths::get_common_path( &slice_of_strings ) {
        Ok( string_result ) => string_result,
        Err( err ) => panic!( "{}", err )
    };
    let expected = "src".to_string();
    assert_eq!( result, expected )
    
### get_common_prefix

    let slice_of_strings = [
        "src/helpers_disk/A/B/C",
        "src/helpers_disk/A/B",
        "src/helpers_disk/A",
    ];
    let result = match easy_paths::get_common_prefix( &slice_of_strings ) {
        Ok( string_result ) => string_result,
        Err( err ) => panic!( "{}", err )
    };
    let expected = "src/helpers_disk/A".to_string();
    assert_eq!( result, expected )

### get_dir_name

    let string_path = "test/test_b/test_c/text_b_c_a.txt";
    let result = match easy_paths::get_dir_name( &string_path ) {
        Some( string_result ) => string_result,
        None => panic!( "Failed" )
    };
    let expected = "test/test_b/test_c".to_string();
    assert_eq!( result, expected )

### get_dir_ancestor_n_levels_up

    let string_path = "test/test_b/test_c/text_b_c_a.txt";
    let int_layers_up: usize = 2;
    let result = match easy_paths::get_dir_ancestor_n_levels_up( &string_path, int_layers_up ) {
        Some( string_result ) => string_result,
        None => panic!( "Failed" )
    };
    let expected = "test/test_b".to_string();
    assert_eq!( result, expected )

### get_dir_ancestor_that_exists

    let string_path = "test/test_b/test_c/text_b_c_a.txt/A/B/C";
    let result = match easy_paths::get_dir_ancestor_that_exists( &string_path ) {
        Some( string_result ) => string_result,
        None => panic!( "Failed" ),
    };
    let expected = "test/test_b/test_c/text_b_c_a.txt".to_string();
    assert_eq!( result, expected )

### get_extension() {

    let string_path = "test/test_b/test_c/text_b_c_a.txt";
    let result = match easy_paths::get_extension( &string_path ) {
        Some( string_result ) => string_result,
        None => panic!( "Failed" )
    };
    let expected = "txt".to_string();
    assert_eq!( result, expected )

### get_only_dirs_from_slice

    let slice_of_strings = [
        "test/test_b",
        "test/test_b/test_c",
        "test/test_b/test_c/text_b_c_b.txt",
        "test/test_b/test_c/text_b_c_a.txt",
        "test/test_a",
        "test/test_a/text_a_a.txt",
    ];
    let result = easy_paths::get_only_dirs_from_slice( &slice_of_strings );
    let expected = [
        "test/test_b",
        "test/test_b/test_c",
        "test/test_a",
    ].iter().map( | item | { format!( "{}", item, ) } ).collect::<Vec<String>>();
    assert_eq!( result, expected, )

### get_only_file_paths_from_slice

    let slice_of_strings = [
        "test/test_b",
        "test/test_b/test_c",
        "test/test_b/test_c/text_b_c_b.txt",
        "test/test_b/test_c/text_b_c_a.txt",
        "test/test_a",
        "test/test_a/text_a_a.txt",
    ];
    let result = easy_paths::get_only_file_paths_from_slice( &slice_of_strings );
    let expected = [
        "test/test_b/test_c/text_b_c_b.txt",
        "test/test_b/test_c/text_b_c_a.txt",
        "test/test_a/text_a_a.txt",
    ].iter().map( | item | { format!( "{}", item, ) } ).collect::<Vec<String>>();
    assert_eq!( result, expected, )

### get_path_joined

    let slice_of_strings = [ "A", "B", "C" ];
    let result = match easy_paths::get_path_joined( &slice_of_strings ) {
        Some( string_result ) => string_result,
        None => panic!( "Failed" )
    };
    let expected = "A/B/C".to_string();
    assert_eq!( result, expected )

### get_paths_in_dir

    let string_path = "test";
    let result = match easy_paths::get_paths_in_dir( &string_path ) {
        Ok( vec_result ) => vec_result,
        Err( err ) => panic!( "{}", err, )
    };
    let expected = [
        "test/test_a",
        "test/test_b",
    ].iter().map( | item_str | { item_str.to_string() } ).collect::<Vec<String>>();
    assert_eq!( result, expected )

### get_paths_in_dir_and_sub_dirs

    let string_path = "test";
    let result = match easy_paths::get_paths_in_dir_and_sub_dirs( &string_path ) {
        Ok( vec_result ) => vec_result,
        Err( err ) => panic!( "{}", err )
    };
    let expected = [
        "test/test_b",
        "test/test_b/test_c",
        "test/test_b/test_c/text_b_c_b.txt",
        "test/test_b/test_c/text_b_c_a.txt",
        "test/test_a",
        "test/test_a/text_a_a.txt",
    ].iter().map( | item_str | { item_str.to_string() } ).collect::<Vec<String>>();
    assert_eq!( result, expected )

### get_paths_sorted_by_size_starting_with_shortest

    let slice_of_strings = [
        "/A/B/C",
        "/A",
        "/A/B",
    ];
    let result = match easy_paths::get_paths_sorted_by_size_starting_with_shortest( &slice_of_strings ) {
        Ok( vec_result ) => vec_result,
        Err( err ) => panic!( "{}", err ),
    };
    let expected = [
        "/A",
        "/A/B",
        "/A/B/C"
    ].iter().map( | item_str | { item_str.to_string() } ).collect::<Vec<String>>();
    assert_eq!( result, expected )

### get_paths_to_only_dirs_in_dir_and_sub_dirs

    let string_path = "test";
    let result = match easy_paths::get_paths_to_only_dirs_in_dir_and_sub_dirs( &string_path ) {
        Ok( vec_result ) => vec_result,
        Err( err ) => panic!( "{}", err, )
    };
    let expected = [
        "test/test_b",
        "test/test_b/test_c",
        "test/test_a",
    ].iter().map(|item_str| {item_str.to_string()}).collect::<Vec<String>>();
    assert_eq!( result, expected )

### get_paths_to_only_files_in_dir_and_sub_dirs

    let string_path = "test";
    let result = match easy_paths::get_paths_to_only_files_in_dir_and_sub_dirs( &string_path ) {
        Ok( vec_result ) => vec_result,
        Err( err ) => panic!( "{}", err, )
    };
    let expected = [
        "test/test_b/test_c/text_b_c_b.txt",
        "test/test_b/test_c/text_b_c_a.txt",
        "test/test_a/text_a_a.txt",
    ].iter().map( | item_str | { item_str.to_string() } ).collect::<Vec<String>>();
    assert_eq!( result, expected )

### get_relative_path

    let string_path_abs_root = "/A/B/C";
    let string_path_abs = "/A/B/C/D";
    let result = match easy_paths::get_relative_path( &string_path_abs, &string_path_abs_root ) {
        Ok( string_result ) => string_result,
        Err( err ) => panic!( "{}", err, ),
    };
    let expected = "D".to_string();
    assert_eq!( result, expected )

### get_vec_by_splitting_path

    let string_path = "test/test_b/test_c/text_b_c_a.txt";
    let result = match easy_paths::get_vec_by_splitting_path( &string_path ) {
        Some( vec_result ) => vec_result,
        None => panic!( "Failed" )
    };
    let expected = [
        "test",
        "test_b",
        "test_c",
        "text_b_c_a.txt",
    ].iter().map(|item_str|{item_str.to_string()}).collect::<Vec<String>>();
    assert_eq!( result, expected )
 
### is_absolute
    
    assert_eq!( easy_paths::is_absolute( &"/A/B/C" ), true, )

### is_existing_path

    assert_eq!( easy_paths::is_existing_path( &"test/test_b/test_c/text_b_c_a.txt" ), true, )

### is_path_type

    assert_eq!( easy_paths::is_path_type( &Path::new( &"test/test_b/test_c/text_b_c_a.txt" ) ), true, )

### is_path_buf_type
    
    assert_eq!( easy_paths::is_path_buf_type( &PathBuf::from( "test/test_b/test_c/text_b_c_a.txt" ) ), true, )

### is_path_inside_dir_parent

    assert_eq!( easy_paths::is_path_inside_dir_parent( &"test/test_b/test_c/text_b_c_a.txt", &"test/test_b" ), true, )
 
### raise_error_if_path_is_not_in_project_absolute

    let mut string_path = "/badpath";
    match easy_paths::raise_error_if_path_is_not_in_project( &string_path ) {
        Ok( () ) => {}
        Err( err ) => { panic!( "{}", err ) }
    }

### raise_error_if_path_points_to_src

    match easy_paths::raise_error_if_path_points_to_src( &"src" ) {
        Ok( () ) => {}
        Err( err ) => { panic!( "{}", err ) }
    }

### raise_error_if_path_points_to_cargo_toml

    let mut string_path = format!( "{}/Cargo.toml", env!( "CARGO_MANIFEST_DIR" ), );
    match easy_paths::raise_error_if_path_points_to_cargo_toml( &string_path ) {
        Ok( () ) => {}
        Err( err ) => { panic!( "{}", err ) }
    }

### raise_error_if_path_points_to_main_rs

    let mut string_path = format!( "{}/src/main.rs", env!( "CARGO_MANIFEST_DIR" ), );
    match easy_paths::raise_error_if_path_points_to_main_rs( &string_path ) {
        Ok( () ) => {}
        Err( err ) => { panic!( "{}", err ) }
    }















































