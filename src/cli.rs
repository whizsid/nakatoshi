pub fn ask<'a, 'b>() -> clap::App<'a, 'b> {
    clap::App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .arg(
            clap::Arg::with_name("starts-with")
                .short("s")
                .long("starts-with")
                .help("Start with prefix used to match addresses")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("case-sensitive")
                .short("c")
                .long("case-sensitive")
                .help("Use case sensitive to match addresses"),
        )
        .arg(
            clap::Arg::with_name("file")
                .short("f")
                .long("file")
                .help("File with starts-with prefixes to generate addresses")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("bech32")
                .conflicts_with("case-sensitive")
                .short("b")
                .long("bech32")
                .takes_value(false)
                .help("Use Bech32 addresses starting with bc1q (Lowercase address)"),
        )
        .arg(
            clap::Arg::with_name("threads")
                .short("t")
                .long("threads")
                .takes_value(false)
                // This is not the actual default value, it is here to pretty display
                // the information.
                .default_value("The number of CPUs available on the current system")
                .help("Number of threads to be used"),
        )
        .arg(
            clap::Arg::with_name("output-file")
                .short("o")
                .long("output-file")
                .takes_value(true)
                .help("Output file"),
        )
}
