use clap::Parser;
use std::io::{self, BufWriter, Write};
use indicatif;
use log::{info, warn};
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct CustomError(String);

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn banner() {

    env_logger::init();

    let font = r#"
                                                                                                                                             
@@@@@@@@@@  @@@ @@@    @@@@@@@@ @@@ @@@@@@@   @@@@@@ @@@@@@@    @@@@@@@  @@@  @@@  @@@@@@ @@@@@@@                           
@@! @@! @@! @@! !@@    @@!      @@! @@!  @@@ !@@       @!!      @@!  @@@ @@!  @@@ !@@       @!!                                        
@!! !!@ @!@  !@!@!     @!!!:!   !!@ @!@!!@!   !@@!!    @!!      @!@!!@!  @!@  !@!  !@@!!    @!!                                          
!!:     !!:   !!:      !!:      !!: !!: :!!      !:!   !!:      !!: :!!  !!:  !!!     !:!   !!:                                          
:::     :::  .:        :::      ::: ::! : :   ::.: :   !::      ::: ::!  :.:: :::  .::!::   !::                                           
                                                                                                                                         
                                                                                                                                         
                                                                                                                                         
                                                                                                                                         
                                                                                                                                         
                                                                                                                                         
                                                                                                                                         
                                                                                                                                         
                                                                                                                                         
@@@  @@@ @@@@@@@@ @@@      @@@       @@@@@@         @@@  @@@ @@@@@@@@ @@@  @@@  @@@    @@@  @@@  @@@  @@@@@@  @@@@@@@  @@@      @@@@@@@  
@@!  @@@ @@!      @@!      @@!      @@!  @@@        @@!@!@@@ @@!      @@!  @@!  @@!    @@!  @@!  @@! @@!  @@@ @@!  @@@ @@!      @@!  @@@ 
@!@!@!@! @!!!:!   @!!      @!!      @!@  !@!        @!@@!!@! @!!!:!   @!!  !!@  @!@    @!!  !!@  @!@ @!@  !@! @!@!!@!  @!!      @!@  !@! 
!!:  !!! !!:      !!:      !!:      !!:  !!! !:!    !!:  !!! !!:       !:  !!:  !!      !:  !!:  !!  !!:  !!! !!: :!!  !!:      !!:  !!! 
:   : : : :: ::  : ::.: : : ::.: :  : :. :  ::     ::    :  : :: ::    ::.:  :::        ::.:  :::    : :. :   :   : : : ::.: : :: :  :  
                                             :                                                                                           
                                             "#;

    info!("{}",font);
}

fn main() -> Result<(), CustomError> {

    let stdout = io::stdout();
    let handle = io::BufWriter::new(stdout);
    let handle = Arc::new(Mutex::new(handle));

    {
        writeln!(handle.lock().unwrap(), "Hello, world!").unwrap();
    }

    let args =  Cli::parse();
    let path = args.path;
    
    writeln!(handle.lock().unwrap(), "pattern {:?}, path: {:?}", args.pattern, path).unwrap();

    let content = std::fs::read_to_string(&path)
                    .map_err(|err| CustomError(format!("error reading `{:?}`: {}", path, err)))?;
    
    writeln!(handle.lock().unwrap(), "content: {}", content).unwrap();
    handle.lock().unwrap().flush();

    run_spinner(Arc::clone(&handle));

    Ok(())
}


fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn run_spinner(handle: Arc<Mutex<BufWriter<io::Stdout>>>) {
    
    let pb = indicatif::ProgressBar::new(100);
    let mut thread_pool = Vec::new();

    for i in 0..50{
        let handle = Arc::clone(&handle);
        let new_thread = thread::spawn(move || {
            for j in 0..10 {
                writeln!(handle.lock().unwrap(), "Spawning thread {} @ {} iter", i, j+1).unwrap();
                handle.lock().unwrap().flush();
            }
        });
        
        thread_pool.push(Some(new_thread));
    }

    while !thread_pool.is_empty() {
        for curr_join_handle in &mut thread_pool {
            if let Some(join_handle) = curr_join_handle.take() {
                if join_handle.is_finished() {
                    let thread_id = join_handle.thread().id();
                    match join_handle.join() {
                        Ok(_) => {
                            pb.println(format!("[+] finished #{:?}", thread_id));
                            pb.inc(1);
                            handle.lock().unwrap().flush();
                        }
                        Err(e) => {
                            writeln!(handle.lock().unwrap(), "Could not join thread {:?}", thread_id);
                        }
                    }
                } else {
                    *curr_join_handle = Some(join_handle);
                }
            } 
        }
        thread_pool.retain(|curr_join_handle| curr_join_handle.is_some());
    }

    pb.finish_with_message("done");

    //  RUST_LOG=info cargo run VV ~/Desktop/test_file.txt 
    banner();
}
