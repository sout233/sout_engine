pub fn print_mlua_error_msg(message: String,path: &str,content: &str) {
    println!("Syntax error: {}", message);
    let which_line = message
        .split(":")
        .skip(3)
        .next()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    println!("File: {}, Line: {}", path, which_line);

    println!(".. | ...");
    let lua_file_line = content.lines().nth(which_line - 3).unwrap_or("");
    println!("{} | {}", which_line - 3, lua_file_line);
    let lua_file_line = content.lines().nth(which_line - 2).unwrap_or("");
    println!(
        "\x1b[1;31m{} | {}    <- Error here\x1b[0m",
        which_line - 2,
        lua_file_line
    );
    let lua_file_line = content.lines().nth(which_line - 1).unwrap_or("");
    println!("{} | {}", which_line - 1, lua_file_line);
}
