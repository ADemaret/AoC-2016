use std::fs::File;

//use log::{error, warn, info, debug, trace};
use log::LevelFilter;

// use std::env;
// use simple_logger::SimpleLogger;
use simplelog::*;

pub fn init_log() {
    // with env_logger :
    // std::env::set_var("RUST_LOG", "debug"); // before init() !!
    // env_logger::init();

    // with simple_logger :
    // SimpleLogger::new()
    //     //.with_colors(false)
    //     .without_timestamps()
    //     .with_level(LevelFilter::Trace)
    //     .init()
    //     .unwrap();

    // with simplelog
    let config = ConfigBuilder::new()
        .set_level_color(Level::Error, Some(Color::Rgb(191, 0, 0)))
        .set_level_color(Level::Warn, Some(Color::Rgb(255, 127, 0)))
        .set_level_color(Level::Info, Some(Color::Rgb(192, 192, 0)))
        .set_level_color(Level::Debug, Some(Color::Rgb(63, 127, 0)))
        .set_level_color(Level::Trace, Some(Color::Rgb(127, 127, 255)))
        .build();

    // with simplelog :
    CombinedLogger::init(vec![
        // only errors, warns & info go on screen
        TermLogger::new(
            LevelFilter::Error,
            //Config::default(),
            config,
            TerminalMode::Stdout, //Mixed,
            ColorChoice::Auto,
        ),
        // all the previous ones + debug & trace go in the log file
        WriteLogger::new(
            LevelFilter::Error,
            Config::default(),
            //config,
            File::create("log/with_simplelog.log").unwrap(),
        ),
    ])
    .unwrap();
}
