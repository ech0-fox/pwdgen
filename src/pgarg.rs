use clap::{Command, Arg};

// Construct command line logic
pub fn cmds() -> clap::ArgMatches {
    return Command::new("pwdgen")
    .arg(
        Arg::new("length")
            .short('l')
            .long("length")
            .value_name("LENGTH")
            .help("Set the base length"),
    )
    .arg(
        Arg::new("mode")
            .short('m')
            .long("mode")
            .value_name("MODE")
            .help("Set the character class mode"),
    )
    .arg(
        Arg::new("bypass_primary_length_check")
            .long("bypass_primary_length_check")
            .alias("bplc")
            .num_args(0)
            .help("Bypass the primary length check"),
    )
    .get_matches();
}