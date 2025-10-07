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

Options:                                                                  -
    --version                   Print the version of the program, and quit.
        -V                      Will ignore other commands or options used.

    --no-warn                   Disables output of warnings when compiling.
        -W

    --soft-silent               Disables all stdout output from the program
        -S                      excluding error messages needing attention.

    --silent                    Disables all stdout output from the program
        -s                      except for output from the --version option
                                and the equivalent \"[...] version\" command.

    --verbose                   Makes the program output [ INFO ] level log
        -v                      entries to aid making detailed bug reports.

",
        filename,
        filename,
        filename,
        filename,
        filename,
        filename
    );
}
