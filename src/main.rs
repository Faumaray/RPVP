use clap::{Args, Parser, Subcommand};
use lab_one::*;
use log::LevelFilter;
use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
};
use mpi::traits::Communicator;

use self::executor::Executor;
// use self::lab_third::Matrix;
mod executor;
mod lab_one;
mod lab_third;
use crate::lab_third::LabThree;
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    laboratory: Labs,
}
#[derive(Subcommand)]
enum Labs {
    /// First Laboratory
    First(First),
    /// Second Laboratory
    Second(Second),
    /// Third Laboratory
    Third(Third),
}
#[derive(Args)]
struct Third {
    rows: usize,
    columns: usize,
    #[arg(default_value_t = false)]
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

fn setup_loger(file: Option<&str>) {
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
                    .filter(Box::new(ThresholdFilter::new(LevelFilter::Info)))
                    .build("logfile", Box::new(logfile)),
            )
            .build(
                Root::builder()
                    .appender("logfile")
                    .appender("stdout")
                    .build(LevelFilter::Trace),
            )
    } else {
        log4rs::config::Config::builder()
            .appender(Appender::builder().build("stdout", Box::new(stdout)))
            .build(Root::builder().appender("stdout").build(LevelFilter::Trace))
    };

    log4rs::init_config(log_config.unwrap()).unwrap();
}

fn main() {
    let cli = Cli::parse();
    match &cli.laboratory {
        Labs::First(lab) => match lab.name.as_str() {
            "ring" => {
                setup_loger(Some("ring"));
                ring(lab.buffer_size);
            }
            "broadcast" => {
                setup_loger(Some("broadcast"));
                broadcast(lab.buffer_size);
            }
            "gather" => {
                setup_loger(Some("gather"));
                gather(lab.buffer_size);
            }
            "alltoall" => {
                setup_loger(Some("alltoall"));
                alltoall(lab.buffer_size);
            }
            _ => {
                unreachable!();
            }
        },
        Labs::Second(lab) => match lab.name.as_str() {
            "midpoint" => {
                setup_loger(None);
            }
            "montecarlo" => {
                setup_loger(None);
            }
            _ => {
                unreachable!();
            }
        },
        Labs::Third(lab) => {
            let universer = mpi::initialize().unwrap();
            let executor = Executor::new(universer.world());
            let result: Vec<f32> = executor.sgemv(lab.random, lab.rows, lab.columns);
            if universer.world().rank() == 0 {
                println!("result = {:?}", result);
            }
            // if lab.random {
            //     let mut rand = rand::thread_rng();
            //     let matrix = {
            //         let mut inner: Vec<Vec<f32>> = Vec::new();
            //         for _ in 0..lab.rows {
            //             let mut tmp: Vec<f32> = Vec::new();
            //             for _ in 0..lab.columns {
            //                 let value: f32 = rand.gen();
            //                 tmp.push(value);
            //             }
            //             inner.push(tmp);
            //         }
            //         Matrix::new(inner)
            //     };
            //     let multi = {
            //         let mut out = Vec::new();
            //         for _ in 0..lab.columns {
            //             let value: f32 = rand.gen();
            //             out.push(value);
            //         }
            //         out
            //     };
            //     matrix.sgemv(multi);
            // } else {
            //     let vector = vec![0.75; lab.columns];
            //     let matrix = Matrix::new(vec![vec![2.5; lab.columns]; lab.rows]);
            //     // println!("{}", matrix);
            //     // println!("{:?}", vector);
            //     matrix.sgemv(vector);
            // }
            // debug!("{}", matrix);
            // println!("vector = {:?}", multi);
            // let result = matrix.sgemv(multi);
        }
    }
}
