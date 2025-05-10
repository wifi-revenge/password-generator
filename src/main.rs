use std::{
    env, fs,
    io::{BufWriter, Write},
    path::Path,
    process::Command,
    sync::{atomic::{AtomicUsize, Ordering}, Arc, Mutex},
    time::Instant
};
use rayon::{ThreadPoolBuilder, prelude::*};
use memmap2::Mmap;
use indicatif::{ProgressBar, ProgressStyle};

const CHUNK_SIZE: usize = 50;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("USAGE: {} [FILE_SERIALI] [FILE_OUTPUT] [NUM_CORE]", args[0]);
        return;
    }

    let file_serials = &args[1];
    let file_passwords = &args[2];

    // Validazione input
    if !Path::new(file_serials).exists() {
        eprintln!("{} non esiste.", file_serials);
        return;
    }
    if !Path::new(file_serials).is_file() {
        eprintln!("{} deve essere un file.", file_serials);
        return;
    }

    // Configurazione core
    let max_num_cores = if args.len() >= 4 {
        args[3].parse().unwrap_or_else(|_| num_cpus::get().saturating_sub(1))
    } else {
        num_cpus::get().saturating_sub(1)
    };

    // Conta linee in modo efficiente
    let file_serials_length = count_lines(file_serials);
    println!("Trovati {} seriali da processare", file_serials_length);

    // Configura thread pool
    ThreadPoolBuilder::new()
        .num_threads(max_num_cores)
        .build_global()
        .unwrap();

    let start_time = Instant::now();
    let counter = Arc::new(AtomicUsize::new(0));

    // Progress bar con stile identico a Python
    let pb = ProgressBar::new(file_serials_length as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{wide_bar} {pos}/{len} [{elapsed}<{eta}]")
        .unwrap()
        .progress_chars("##-"));

    // Processamento parallelo
    let output_file = fs::File::create(file_passwords).unwrap();
    let writer = Arc::new(Mutex::new(BufWriter::new(output_file)));

    get_chunks(file_serials).par_bridge().for_each(|chunk| {
        let results: Vec<String> = chunk.iter()
            .map(|serial| {
                let password = serial_to_password(serial);
                if password == "CYKFHBLFNMGQ7J8X" {
                    println!("{}:{}", serial, password);
                }
                format!("{}:{}", serial, password)
            })
            .collect();

        let mut writer = writer.lock().unwrap();
        writeln!(writer, "{}", results.join("\n")).unwrap();

        let current = counter.fetch_add(chunk.len(), Ordering::Relaxed);
        pb.set_position(current as u64);
    });

    pb.finish();
    println!("\nTempo totale: {:?}", start_time.elapsed());
}

fn count_lines(filename: &str) -> usize {
    let file = fs::File::open(filename).unwrap();
    let mmap = unsafe { Mmap::map(&file).unwrap() };
    mmap.iter().filter(|&&c| c == b'\n').count()
}

fn get_chunks(filename: &str) -> impl Iterator<Item = Vec<String>> {
    let file = fs::File::open(filename).unwrap();
    let mmap = unsafe { Mmap::map(&file).unwrap() };
    
    let mut chunks = Vec::new();
    let mut current_chunk = Vec::with_capacity(CHUNK_SIZE);
    let mut start = 0;
    
    for (i, &c) in mmap.iter().enumerate() {
        if c == b'\n' {
            let line = &mmap[start..i];
            let serial = String::from_utf8_lossy(line).trim().to_string();
            if !serial.is_empty() {
                current_chunk.push(serial);
                
                if current_chunk.len() == CHUNK_SIZE {
                    chunks.push(current_chunk);
                    current_chunk = Vec::with_capacity(CHUNK_SIZE);
                }
            }
            start = i + 1;
        }
    }
    
    if !current_chunk.is_empty() {
        chunks.push(current_chunk);
    }
    
    chunks.into_iter()
}

fn serial_to_password(serial: &str) -> String {
    let output = Command::new("zykgen")
        .args(&["-c", "-l", "16", serial])
        .output()
        .unwrap_or_else(|_| panic!("Fallito esecuzione zykgen per {}", serial));
    
    String::from_utf8(output.stdout)
        .unwrap()
        .trim()
        .to_string()
}
