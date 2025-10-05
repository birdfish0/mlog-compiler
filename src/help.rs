pub fn default_help_msg(filename: &str) {
    println!(
        "Usage:                                                                    -
    {} [COMMAND] [OPTIONS] [args]
    {} [OPTIONS] [COMMAND] [args]
    {} [COMMAND] [args] [OPTIONS]
    {} [COMMAND]        [OPTIONS]
    {} [OPTIONS]        [COMMAND]
    {}                  [COMMAND]

Commands:                                                                 -
    compile [file path]         Compile the file using default options from
                                <file.*> to <file.mlog>, error on conflict.
                                If the filename begins with a dash, precede
                                it with a backslash and put it in quotation
                                marks: <application file> compile \"\\--.txt\"

    help [command/option]       Display this help message, shows additional
                                about the provided command or option if one
                                is provided. Options should be placed after
                                a backslash, for example: \"help \\--no-warn\"

    version                     Print the version of the program, and quit.
                                Will ignore other commands or options used.

Options:
    --version                   Print the version of the program, and quit.
        -v                      Will ignore other commands or options used.

    --no-warn                   Disables output of warnings when compiling.
        -W

",
        filename,
        filename,
        filename,
        filename,
        filename,
        filename
    );
}
