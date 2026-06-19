use std::env;
use std::process::ExitCode;

use libome_rs::*;

fn usage() {
    eprint!(
        "\
Usage:
  libome-rs eval <OME> reg <as> <LM> <NF> <x>
  libome-rs eval <OME> plus <as> <LM> <NF> <x>
  libome-rs eval <OME> delta <as> <LM> <NF>
  libome-rs eval <OME> reg-trunc <order> <as> <LM> <NF> <x>
  libome-rs eval <OME> plus-trunc <order> <as> <LM> <NF> <x>
  libome-rs eval <OME> delta-trunc <order> <as> <LM> <NF>"
    );

    #[cfg(feature = "mellin")]
    eprint!(
        "
  libome-rs moment <OME> <N> <as> <LM> <NF> [eps_abs] [eps_rel]"
    );

    eprintln!(
        "

OMEs with reg, plus, delta parts:
  AqqQNSEven  AqqQNSOdd  AggQ
  polAqqQNSEven  polAqqQNSOdd  polAggQ

OMEs with reg part only:
  AQqPS  AQqPSs  AqqQPS  AqgQ  AgqQ  AQg
  polAQqPS  polAQqPSs  polAqqQPS  polAqgQ  polAgqQ  polAQg"
    );
}

fn parse_f64(s: &str, name: &str) -> Result<f64, String> {
    s.parse::<f64>().map_err(|_| format!("invalid {name}: {s}"))
}

fn parse_i32(s: &str, name: &str) -> Result<i32, String> {
    s.parse::<i32>().map_err(|_| format!("invalid {name}: {s}"))
}

macro_rules! dispatch_reg {
    ($name:expr, $body:expr) => {
        match $name {
            "AqqQNSEven" => {
                type Ome = AqqQNSEven;
                $body
            }
            "AqqQNSOdd" => {
                type Ome = AqqQNSOdd;
                $body
            }
            "AggQ" => {
                type Ome = AggQ;
                $body
            }
            "AQqPS" => {
                type Ome = AQqPS;
                $body
            }
            "AQqPSs" => {
                type Ome = AQqPSs;
                $body
            }
            "AqqQPS" => {
                type Ome = AqqQPS;
                $body
            }
            "AqgQ" => {
                type Ome = AqgQ;
                $body
            }
            "AgqQ" => {
                type Ome = AgqQ;
                $body
            }
            "AQg" => {
                type Ome = AQg;
                $body
            }
            "polAqqQNSEven" => {
                type Ome = PolAqqQNSEven;
                $body
            }
            "polAqqQNSOdd" => {
                type Ome = PolAqqQNSOdd;
                $body
            }
            "polAggQ" => {
                type Ome = PolAggQ;
                $body
            }
            "polAQqPS" => {
                type Ome = PolAQqPS;
                $body
            }
            "polAQqPSs" => {
                type Ome = PolAQqPSs;
                $body
            }
            "polAqqQPS" => {
                type Ome = PolAqqQPS;
                $body
            }
            "polAqgQ" => {
                type Ome = PolAqgQ;
                $body
            }
            "polAgqQ" => {
                type Ome = PolAgqQ;
                $body
            }
            "polAQg" => {
                type Ome = PolAQg;
                $body
            }
            other => return Err(format!("unknown OME: {other}")),
        }
    };
}

macro_rules! dispatch_rpd {
    ($name:expr, $body:expr) => {
        match $name {
            "AqqQNSEven" => {
                type Ome = AqqQNSEven;
                $body
            }
            "AqqQNSOdd" => {
                type Ome = AqqQNSOdd;
                $body
            }
            "AggQ" => {
                type Ome = AggQ;
                $body
            }
            "polAqqQNSEven" => {
                type Ome = PolAqqQNSEven;
                $body
            }
            "polAqqQNSOdd" => {
                type Ome = PolAqqQNSOdd;
                $body
            }
            "polAggQ" => {
                type Ome = PolAggQ;
                $body
            }
            other => return Err(format!("{other} does not have plus/delta parts")),
        }
    };
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage();
        return Err("no command given".into());
    }

    match args[1].as_str() {
        "eval" => cmd_eval(&args[2..]),
        #[cfg(feature = "mellin")]
        "moment" => cmd_moment(&args[2..]),
        "help" | "--help" | "-h" => {
            usage();
            Ok(())
        }
        other => {
            usage();
            Err(format!("unknown command: {other}"))
        }
    }
}

fn cmd_eval(args: &[String]) -> Result<(), String> {
    if args.len() < 2 {
        return Err("eval requires <OME> <part> ...".into());
    }
    let ome_name = args[0].as_str();
    let part = args[1].as_str();

    match part {
        "reg" => {
            if args.len() < 6 {
                return Err("eval reg requires <as> <LM> <NF> <x>".into());
            }
            let as_ = parse_f64(&args[2], "as")?;
            let lm = parse_f64(&args[3], "LM")?;
            let nf = parse_f64(&args[4], "NF")?;
            let x = parse_f64(&args[5], "x")?;
            dispatch_reg!(ome_name, {
                println!("{:.17e}", Ome::reg(as_, lm, nf, x));
            });
        }
        "plus" => {
            if args.len() < 6 {
                return Err("eval plus requires <as> <LM> <NF> <x>".into());
            }
            let as_ = parse_f64(&args[2], "as")?;
            let lm = parse_f64(&args[3], "LM")?;
            let nf = parse_f64(&args[4], "NF")?;
            let x = parse_f64(&args[5], "x")?;
            dispatch_rpd!(ome_name, {
                println!("{:.17e}", Ome::plus(as_, lm, nf, x));
            });
        }
        "delta" => {
            if args.len() < 5 {
                return Err("eval delta requires <as> <LM> <NF>".into());
            }
            let as_ = parse_f64(&args[2], "as")?;
            let lm = parse_f64(&args[3], "LM")?;
            let nf = parse_f64(&args[4], "NF")?;
            dispatch_rpd!(ome_name, {
                println!("{:.17e}", Ome::delta(as_, lm, nf));
            });
        }
        "reg-trunc" => {
            if args.len() < 7 {
                return Err("eval reg-trunc requires <order> <as> <LM> <NF> <x>".into());
            }
            let order = parse_i32(&args[2], "order")?;
            let as_ = parse_f64(&args[3], "as")?;
            let lm = parse_f64(&args[4], "LM")?;
            let nf = parse_f64(&args[5], "NF")?;
            let x = parse_f64(&args[6], "x")?;
            dispatch_reg!(ome_name, {
                println!("{:.17e}", Ome::reg_trunc_as(order, as_, lm, nf, x));
            });
        }
        "plus-trunc" => {
            if args.len() < 7 {
                return Err("eval plus-trunc requires <order> <as> <LM> <NF> <x>".into());
            }
            let order = parse_i32(&args[2], "order")?;
            let as_ = parse_f64(&args[3], "as")?;
            let lm = parse_f64(&args[4], "LM")?;
            let nf = parse_f64(&args[5], "NF")?;
            let x = parse_f64(&args[6], "x")?;
            dispatch_rpd!(ome_name, {
                println!("{:.17e}", Ome::plus_trunc_as(order, as_, lm, nf, x));
            });
        }
        "delta-trunc" => {
            if args.len() < 6 {
                return Err("eval delta-trunc requires <order> <as> <LM> <NF>".into());
            }
            let order = parse_i32(&args[2], "order")?;
            let as_ = parse_f64(&args[3], "as")?;
            let lm = parse_f64(&args[4], "LM")?;
            let nf = parse_f64(&args[5], "NF")?;
            dispatch_rpd!(ome_name, {
                println!("{:.17e}", Ome::delta_trunc_as(order, as_, lm, nf));
            });
        }
        other => return Err(format!("unknown part: {other}")),
    }
    Ok(())
}

#[cfg(feature = "mellin")]
fn cmd_moment(args: &[String]) -> Result<(), String> {
    if args.len() < 5 {
        return Err("moment requires <OME> <N> <as> <LM> <NF> [eps_abs] [eps_rel]".into());
    }
    let ome_name = args[0].as_str();
    let n = parse_i32(&args[1], "N")?;
    let as_ = parse_f64(&args[2], "as")?;
    let lm = parse_f64(&args[3], "LM")?;
    let nf = parse_f64(&args[4], "NF")?;
    let eps_abs = if args.len() > 5 {
        parse_f64(&args[5], "eps_abs")?
    } else {
        0.0
    };
    let eps_rel = if args.len() > 6 {
        parse_f64(&args[6], "eps_rel")?
    } else {
        3e-11
    };

    dispatch_reg!(ome_name, {
        let res = Ome::mellin_moment(n, as_, lm, nf, eps_abs, eps_rel);
        if res.status != IntegrationStatus::Success {
            eprintln!("warning: integration status {:?}", res.status);
        }
        println!("{:.17e} +/- {:.17e}", res.value, res.abs_error);
    });
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}
