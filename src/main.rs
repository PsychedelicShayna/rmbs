use anyhow as ah;
use minreq as mr;
use serde_json as sj;
use std::io::{stdout, Read, Write};
use std::sync::{atomic, Arc};
use std::thread::{self, JoinHandle};
use std::time::Duration;

const ANSI_RESET: &str = "\x1b[0m";
const ANSI_RED: &str = "\x1b[31m";
const ANSI_YELLOW: &str = "\x1b[33m";
const ANSI_BRIGHT_RED: &str = "\x1b[91m";

const ENDPOINT: &str =
    "uggcf://ohyyfuvg-erzbire-shapgvba-ncc.nmherjrofvgrf.arg/ncv/i1/erzbirOhyyfuvg";

const NORMIE_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36";

const COMMON_HEADERS: [(&str, &str); 5] = [
    ("Accept", "*/*"),
    ("Accept-Encoding", "gzip, deflate"),
    ("Connection", "close"),
    ("Origin", "https://www.bullshitremover.com"),
    ("User-Agent", NORMIE_USER_AGENT),
];

fn rot13(string: &str) -> String {
    string
        .chars()
        .map(|c| match c {
            'a'..='m' | 'A'..='M' => (c as u8 + 13) as char,
            'n'..='z' | 'N'..='Z' => (c as u8 - 13) as char,
            _ => c,
        })
        .collect()
}

fn remove_bullshit(bullshit: &str) -> ah::Result<String> {
    let response = mr::options(rot13(ENDPOINT))
        .with_headers(COMMON_HEADERS)
        .with_header(rot13("Npprff-Pbageby-Erdhrfg-Zrgubq"), "POST")
        .with_header(
            rot13("Npprff-Pbageby-Erdhrfg-Urnqref"),
            rot13("pbagrag-glcr,k-shapgvbaf-xrl"),
        )
        .send()?;

    if response.status_code != 200 {
        ah::bail!(
            "Couldn't send preflight request, received HTTP {}",
            response.status_code
        );
    }

    let response = mr::post(rot13(ENDPOINT))
        .with_headers(COMMON_HEADERS)
        .with_header(
            rot13("k-shapgvbaf-xrl"),
            rot13("Ons30XUQJ4FPxn5oH2tmrqgJUWXf1n3HdP1eRdK12W7wNmSh9ggpDj=="),
        )
        .with_body(serde_json::json!({ "text": bullshit }).to_string())
        .send()?;

    if response.status_code != 200 {
        ah::bail!(
            "Received an HTTP {} when sending POST request!",
            response.status_code
        );
    }

    #[allow(non_snake_case)]
    #[derive(serde::Deserialize)]
    struct ResponseModel {
        cleanedText: String,
    }

    Ok(sj::from_str::<ResponseModel>(response.as_str()?).map(|rm| rm.cleanedText)?)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut quiet: bool = false;

    let mut bullshit: Option<String> = None;

    for arg in args {
        match arg.as_str() {
            "--help" | "-h" => {
                println!("Usage: rmbs [--quiet|-q] <text>\nReads from stdin if no string given.");
                std::process::exit(0);
            }

            "--quiet" | "-q" => {
                quiet = true;
            }

            text => bullshit = Some(text.into()),
        }
    }

    let bullshit = bullshit.unwrap_or_else(|| {
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer).unwrap();
        buffer
    });

    let stop_signal = Arc::new(atomic::AtomicBool::new(false));

    let mut animation_thread: Option<JoinHandle<()>> = None;

    if !quiet {
        println!();
        let stop = Arc::clone(&stop_signal);

        animation_thread = Some(thread::spawn(move || {
            let progress_chars = ['|', '/', '-', '\\'];
            let mut index: usize = 0;

            while !stop.load(atomic::Ordering::SeqCst) {
                let loop_char: char = progress_chars[index % progress_chars.len()];
                index += 1;

                let mut stdout = stdout().lock();

                print!(
                    "\r{}Removing Bullshit {}{}{}{} ",
                    ANSI_BRIGHT_RED, ANSI_RESET, ANSI_YELLOW, loop_char, ANSI_RESET,
                );

                let _ = stdout.flush();

                thread::sleep(Duration::from_millis(100));
            }
        }));
    }

    let result = remove_bullshit(&bullshit);

    if let Some(join_handle) = animation_thread {
        stop_signal.store(true, atomic::Ordering::SeqCst);
        let _ = join_handle.join();
    }

    let message = result
        .map(|bullshit_removed| {
            if quiet {
                bullshit_removed
            } else {
                format!("{}\r{}\r{}", ANSI_RESET, " ".repeat(25), bullshit_removed)
            }
        })
        .unwrap_or_else(|e| {
            format!(
                "{}\n {}ERROR !!! {}{}\n\n",
                ANSI_RESET, ANSI_RED, e, ANSI_RESET
            )
        });

    println!("{}", message);
}
