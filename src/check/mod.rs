use console::{Term, style};
use indicatif::{ProgressBar, ProgressStyle};


extern crate difference;
use difference::Changeset;
use difference::Difference as Df;

use std::io::Result;
use std::io::prelude::*;
use std::process::{Command, Stdio};

use std::time::SystemTime;
use std::path::PathBuf;

use crate::paths::{fix_ex_path, path_child};

mod stats;

struct Out{
    #[allow(dead_code)]
    took: u64, 
    res: Vec<u8> 
}

enum TestRes{
    Ok,
    Diff{
       ds: Changeset, 
       gen: Vec<u8>
    }
}

struct Test{
    res: TestRes,
    tbrute: u64, 
    tsolve: u64
}

#[derive(Debug)]
struct Config{
    tc: u64,
    brute: PathBuf, 
    solve: PathBuf, 
    gen: PathBuf,
    out: PathBuf
}


/**
 * Get test cout from args
 */
fn get_config(matches: &clap::ArgMatches) -> Config{
    let tc: u64 = matches.value_of("t").unwrap_or("100").parse().unwrap_or(100) as u64;
    Config{
        tc:tc,
        gen: fix_ex_path(&matches.value_of("g").unwrap_or("gen")),
        solve: fix_ex_path(&matches.value_of("s").unwrap_or("solve")),
        brute: fix_ex_path(&matches.value_of("b").unwrap_or("brute")),
        out: PathBuf::from(&matches.value_of("o").unwrap_or("check"))
    }
}

/**
 * Run single executable fname and pass gen as input
 */
fn run(fname: &PathBuf, gen: &[u8]) -> Result<Out>{
    let time_start = SystemTime::now();
    let mut pc = Command::new(fname)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    pc.stdin.as_mut().expect(&format!("Failed to write to stdin of {}", fname.to_str().expect("Failed to parse path"))).write(&gen)?;
    let pc_out = pc.wait_with_output()?;
    let time_took = SystemTime::now().duration_since(time_start).unwrap().as_millis();
    Ok(Out{
        took: time_took as u64,
        res: pc_out.stdout
    })
}

/**
 * Run test 
 */
fn run_test(conf: &Config) -> Result<Test> {
    let gen_out = Command::new(&conf.gen).output()?.stdout;

    let brute = run(&conf.brute, &gen_out)?;
    let solve = run(&conf.solve, &gen_out)?;

    if brute.res != solve.res {
        std::fs::write(path_child(&conf.out,"gen.txt"), &gen_out).ok();
        std::fs::write(path_child(&conf.out,"brute.txt"), &brute.res).ok();
        std::fs::write(path_child(&conf.out,"solve.txt"), &solve.res).ok();

        let str_brute = std::str::from_utf8(&brute.res).unwrap_or("NO BRUTE RES");
        let str_sovle = std::str::from_utf8(&solve.res).unwrap_or("NO SOLVE RES");
        let set = Changeset::new(str_brute, str_sovle, "\n");
        write_diff(&path_child(&conf.out,"diff.txt"),&set.diffs);

        return Ok(Test{
            res: TestRes::Diff{
                ds: set, 
                gen: gen_out
            },
            tbrute: brute.took, 
            tsolve: solve.took
        });
    }

    Ok(Test{res: TestRes::Ok{}, tbrute: brute.took, tsolve: solve.took})
}

/**
 * write difference to diff.txt
 */
fn write_diff(path: &PathBuf, ds: &Vec<Df>){
    let mut out = String::new(); 
    for dif in ds{
        match dif{
            Df::Same(st) => out.push_str(&format!("{}\n", st)),
            Df::Add(st) =>  out.push_str(&format!("b: {}\n", st)),
            Df::Rem(st) =>  out.push_str(&format!("s: {}\n", st)),
        }
    }
    std::fs::write(path, &out).ok();
}

/**
 * write difference to stdout
 */
fn write_diff_stdout(ds: &Changeset){
    for d in &ds.diffs{
        match d{
            Df::Same(st) => println!("{}", style(st)),
            Df::Add(st) =>  println!("{}", style(st).red()),
            Df::Rem(st) =>  println!("{}", style(st).green()),
        }
    }
}

/**
 * Handle case where answers differ
 */
fn handle_err(term: &Term, ds: &Changeset, gen: &[u8]){
    println!("Got {} results!", style("different").red());
    match term.read_char().expect("!!"){
        's' => {
            term.clear_screen().ok();
            println!("Gen: \n{}", std::str::from_utf8(&gen).unwrap());
            println!("Res: ");
            write_diff_stdout(&ds);
            term.read_char().ok();
            term.clear_screen().ok();
        },
        _ => {
            term.clear_last_lines(1).ok();
        }
    };
}

pub fn launch(args: &clap::ArgMatches){
    let term = Term::stdout();
    let conf = get_config(args); 

    let bar = ProgressBar::new(conf.tc);
    bar.set_style(ProgressStyle::default_bar()
        .template("[{elapsed}] {wide_bar} {pos} | {per_sec} {eta}")
        .progress_chars("## ")
    );
    bar.inc(0);

    let mut stats = stats::Stats::raw();

    for _ in 0..conf.tc {
        let res = run_test(&conf);
        match res{
            Err(e) => {
                println!("{:#?}", conf);
                println!("Encountered IO error {}",e);
                std::process::exit(0);
            },
            Ok(test) =>{
                stats.report( match test.res{TestRes::Ok=>false,_=>true}, test.tbrute, test.tsolve);
                match test.res{
                    TestRes::Ok => (),
                    TestRes::Diff{
                        ds: set, 
                        gen: g
                    } => handle_err(&term, &set, &g)
                }
            }
        }
        bar.inc(1);
    }
    bar.finish();

    println!("{}", stats);
}
