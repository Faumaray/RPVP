#![feature(associated_type_defaults)]
mod executor;
mod lab_one;
mod lab_third;
mod lab_two;

use clap::{Args, Parser, Subcommand};
use lab_one::*;
use lab_two::LabTwo;
use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
};
use systemstat::Platform;

use self::executor::Executor;

use crate::lab_third::LabThree;
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    laboratory: Labs,
    #[arg(short, long, default_value_t = log::LevelFilter::Info)]
    verbose: log::LevelFilter,
}
#[derive(Subcommand)]
enum Labs {
    /// First Laboratory
    First(First),
    /// Second Laboratory
    Second(Second),
    /// Third Laboratory
    Third(Third),
    /// Fourth Laboratory
    Four
}
#[derive(Args)]
struct Third {
    rows: usize,
    columns: usize,
    #[arg(short, long, default_value_t = false)]
    random: bool,
}

#[derive(Args)]
struct First {
    /// Name of the method
    #[arg(short, long, default_value_t = String::from("ring"),value_parser = clap::builder::PossibleValuesParser::new(["ring", "broadcast", "gather", "alltoall"]))]
    name: String,
    /// Size of the message buffer
    #[arg(short, long, default_value_t = 1)]
    buffer_size: usize,
}

#[derive(Args)]
struct Second {
    /// Name of the method
    #[arg(short, long, default_value_t = String::from("midpoint"), value_parser = clap::builder::PossibleValuesParser::new(["midpoint", "montecarlo"]))]
    name: String,
    /// Variant from last number
    #[arg(short, long, default_value_t = 6)]
    variant: usize,
    /// Number of iterations for monte_carlo
    #[arg(short, long)]
    count: Option<usize>,
}

fn setup_loger(file: Option<&str>, level: log::LevelFilter) {
    let stdout: ConsoleAppender = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{h({m})}\n")))
        .build();

    let log_config = if let Some(filename) = file {
        let logfile = FileAppender::builder()
            .encoder(Box::new(PatternEncoder::new("{h({m})}\n")))
            .build(filename)
            .unwrap();

        log4rs::config::Config::builder()
            .appender(Appender::builder().build("stdout", Box::new(stdout)))
            .appender(
                Appender::builder()
                    .filter(Box::new(ThresholdFilter::new(log::LevelFilter::Trace)))
                    .build("logfile", Box::new(logfile)),
            )
            .build(
                Root::builder()
                    .appender("logfile")
                    .appender("stdout")
                    .build(level),
            )
    } else {
        log4rs::config::Config::builder()
            .appender(Appender::builder().build("stdout", Box::new(stdout)))
            .build(Root::builder().appender("stdout").build(level))
    };

    log4rs::init_config(log_config.unwrap()).unwrap();
}

fn main() {
    let cli = Cli::parse();
    match &cli.laboratory {
        Labs::First(lab) => match lab.name.as_str() {
            "ring" => {
                setup_loger(Some("ring"), cli.verbose);
                ring(lab.buffer_size);
            }
            "broadcast" => {
                setup_loger(Some("broadcast"), cli.verbose);
                broadcast(lab.buffer_size);
            }
            "gather" => {
                setup_loger(Some("gather"), cli.verbose);
                gather(lab.buffer_size);
            }
            "alltoall" => {
                setup_loger(Some("alltoall"), cli.verbose);
                alltoall(lab.buffer_size);
            }
            _ => {
                unreachable!();
            }
        },
        Labs::Second(lab) => match lab.name.as_str() {
            "midpoint" => {
                setup_loger(None, cli.verbose);
                let universer = mpi::initialize().unwrap();
                let executor = Executor::new(universer.world());
                let _: f32 = Executor::midpoint_rule(
                    executor,
                    0.000001,
                    |x| (1.0 - (0.7 / x).exp()) / (2.0 + x),
                    (1.0, 2.0),
                );
            }
            "montecarlo" => {
                setup_loger(None, cli.verbose);
                let universer = mpi::initialize().unwrap();
                let executor = Executor::new(universer.world());
                let _: f32 = Executor::monte_carlo(
                    executor,
                    |x, y| {
                        if x < 0.0 || x > 1.0 || y < 2.0 || y > 5.0 {
                            return None;
                        }
                        Some(x / y.powi(2))
                    },
                    0.0..1.0,
                    2.0..5.0,
                    lab.count.unwrap().try_into().unwrap(),
                );
            }
            _ => {
                unreachable!();
            }
        },
        Labs::Third(lab) => {
            setup_loger(None, cli.verbose);
            let universer = mpi::initialize().unwrap();
            let executor = Executor::new(universer.world());
            {
                let needed = bytesize::ByteSize::b(
                    ((32 * lab.columns as u64 * lab.rows as u64)
                        + (32 * lab.columns as u64 * executor.size() as u64 * 2)
                        + (32 * executor.size() as u64))
                        / 8,
                );
                let (mem, swap) = systemstat::System::new().memory_and_swap().unwrap();
                if mem.free + swap.free <= needed {
                    if executor.rank() == 0 {
                        log::error!("Not enough available RAM + SWAP");
                        log::error!("Available total: {}", mem.total);
                        log::error!("Free: {}", mem.free);
                        log::error!("Swap: {}", swap.total);
                        log::error!("Swap free: {}", swap.free);
                        log::error!("Needed {}", needed);
                    }
                    return;
                }
                if executor.rank() == 0 {
                    log::warn!("Will be used {} of memory", needed);
                }
            }
            let _: Vec<f32> = executor.sgemv(lab.random, lab.rows, lab.columns);
        },
        Labs::Four => {
            
            setup_loger(None, cli.verbose);
            let universer = mpi::initialize().unwrap();
            let executor = Executor::new(universer.world());
            executor.topological_ring();
        }
    }
}
