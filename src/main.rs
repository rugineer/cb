use cli_clipboard::{ClipboardContext, ClipboardProvider};
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::sync::mpsc::channel;
use std::thread::{sleep, spawn};
use std::time::Duration;

fn main() {
    let mut ctx = ClipboardContext::new().expect("clipboard context error");
    let mut args = std::env::args();
    let param = args.nth(1).unwrap_or(String::from("")).to_lowercase();
    if param.eq("-p") {
        println!(
            "{}",
            ctx.get_contents()
                .unwrap_or(String::from("clipboard error"))
        );
        return;
    }
    let (tx, cx) = channel::<String>();
    spawn(move || {
        let mut buffer = Vec::new();
        let mut stdin = io::stdin();
        stdin.read_to_end(&mut buffer).expect("read error");
        let str = String::from_utf8_lossy(&buffer).to_string();
        tx.send(str).unwrap()
    });
    sleep(Duration::from_millis(100));
    let mut str = cx.try_recv().unwrap_or(String::from(""));
    if str.eq("") {
        sleep(Duration::from_millis(500));
        str = cx.try_recv().unwrap_or(String::from(""));
        if str.eq("") {
            return;
        }
    }
    if param.eq("-v") {
        println!("{}", &str)
    }
    if param.ne("") {
        let file;
        if param.eq("-a") {
            if let Some(filename) = args.next() {
                file = File::options().create(true).append(true).open(&filename);
            } else {
                file = File::options().create(true).write(true).open(&param);
            }
            let mut file = file.expect(&format!("file {} create error", &param));
            file.write_all(str.as_bytes())
                .expect(&format!("write to file '{}' error", &param));
        }
    }
    ctx.set_contents(str).expect("clipboard error");
}
