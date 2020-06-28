use pyo3::prelude::*;
use pyo3::types::IntoPyDict;
use std::env;
use std::thread;
mod mutation;

fn fuzz_urllib(py: Python, fuzzcase: String) -> PyResult<()> {
    let locals = [("url", py.import("urllib3.util")? ), ("pprint", py.import("pprint")?)].into_py_dict(py);
    let code = format!("pprint.PrettyPrinter().pformat(url.parse_url('{}'))", fuzzcase);
    let val = py.eval(&code, None, Some(&locals));
    let _a = match val {
        Ok(string) => println!("test: {} res: {}", fuzzcase, string),
        Err(err) => println!("test: {}, res: {:#?}", fuzzcase, err)
    };
    return Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage ./alo <test cases per thread> <threads>");
        println!("For example ./alo 1000 2 will run 2000 test cases");
        std::process::exit(0);
    } else {
        let rounds  = &args[1].parse::<u64>().unwrap();
        let round = *rounds;
        let threads = &args[2].parse::<u64>().unwrap();
        for _i in 0..*threads { 
            let handle =thread::spawn(move || {
                let gil = Python::acquire_gil();
                for _counter in 0..round {
                    let fuzzcase = mutation::generate_mutation("http://www.google.com/?a=bcd");
                    fuzz_urllib(gil.python(), fuzzcase).unwrap();
                }
            });
            let _res = handle.join().unwrap();
        }
    }
}
