extern crate baus;
extern crate skim;
use blockish::render_image_fitting_terminal;
use nvim_rs::rpc::handler::Dummy;
use nvim_rs::Neovim;
use parity_tokio_ipc::{Connection, Endpoint};
use regex::Regex;
use skim::prelude::*;
use std::collections::HashMap;
use std::env;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use std::{error::Error, fmt};
use tokio::io::{split, WriteHalf};
use tokio_util::compat::Compat;
use tokio_util::compat::TokioAsyncReadCompatExt;
use tokio_util::compat::TokioAsyncWriteCompatExt;

use rand::seq::IteratorRandom;
use std::fs;

#[derive(Debug, Clone)]
struct Item {
    text: String,
}

impl SkimItem for Item {
    fn text(&self) -> Cow<str> {
        Cow::Borrowed(&self.text)
    }

    fn preview(&self, _context: PreviewContext) -> ItemPreview {
        ItemPreview::Text(self.text.to_owned())
    }
}

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}

#[derive(Debug, Clone)]
struct HomeDirNotFound;

impl Error for HomeDirNotFound {}

impl fmt::Display for HomeDirNotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "home directory was not retrieved")
    }
}

fn home() -> Result<String, Box<dyn Error>> {
    Ok(dirs::home_dir()
        .ok_or_else(|| Box::new(HomeDirNotFound))?
        .to_string_lossy()
        .to_string())
}

fn random_image() -> Result<Option<String>, Box<dyn Error>> {
    let mut rng = rand::thread_rng();
    let wallpapers_dir = vmux_wallpapers_path()?;
    if Path::new(&wallpapers_dir).is_dir() {
        let files = fs::read_dir(wallpapers_dir)?;
        match files.choose(&mut rng) {
            Some(Ok(file)) => Ok(Some(file.path().display().to_string())),
            _ => Ok(None),
        }
    } else {
        Ok(None)
    }
}

fn list_sessions() -> Result<Vec<String>, Box<dyn Error>> {
    let session_regx = Regex::new(r".*vmux-session.*")?;
    Ok(diss::list_sessions()?
        .into_iter()
        .filter(|x| session_regx.is_match(x))
        .collect())
}

fn show_session_list() -> Result<(), Box<dyn Error>> {
    for session in list_sessions()? {
        println!("{}", session);
    }
    Ok(())
}

fn list_sessions_with_baus(previous_session_name: String) -> Result<Vec<String>, Box<dyn Error>> {
    let args = baus::Args {
        name: "vmux".to_string(),
        action: baus::Action::Sort,
        value: baus::SavedValue::Timestamp,
        desc: true,
        cleanup: true,
    };
    let cache_file_path = baus::get_cache_file_path(&args)?;
    let mut lines_backup = baus::get_lines_backup(&cache_file_path)?;
    let sessions_list = list_sessions()?;
    let sessions_list = baus::sort(&args, sessions_list, &mut lines_backup, &cache_file_path)?;
    let mut sessions_with_previous: Vec<String> = sessions_list
        .clone()
        .into_iter()
        .filter(|x| x.contains(&previous_session_name))
        .collect();
    let mut res: Vec<String> = sessions_list
        .into_iter()
        .filter(|x| !x.contains(&previous_session_name))
        .collect();
    res.append(&mut sessions_with_previous);
    Ok(res)
}

fn save_with_baus(val: String) -> Result<Vec<String>, Box<dyn Error>> {
    let args = baus::Args {
        name: "vmux".to_string(),
        action: baus::Action::Save,
        value: baus::SavedValue::Timestamp,
        desc: true,
        cleanup: true,
    };
    let cache_file_path = baus::get_cache_file_path(&args)?;
    let lines_backup = baus::get_lines_backup(&cache_file_path)?;
    let res = vec![val];
    baus::save(&args, res, lines_backup, &cache_file_path)
}

fn vmux_wallpapers_path() -> Result<String, Box<dyn Error>> {
    Ok(format!("{}/.config/vmux/wallpapers/", home()?))
}

fn vmux_hook_path(hook_name: &str) -> Result<String, Box<dyn Error>> {
    Ok(format!("{}/.config/vmux/hooks/{}.sh", home()?, hook_name))
}

fn list_sessions_name_hook() -> Result<Vec<String>, Box<dyn Error>> {
    let list_session_name_path = vmux_hook_path("list_sessions_names")?;
    let list_session_name_f = Path::new(&list_session_name_path);
    if list_session_name_f.is_file() {
        let output = Command::new("bash").arg(list_session_name_path).output()?;
        Ok(output
            .stdout
            .lines()
            .map(|x| format!("New: {}", x.unwrap()))
            .collect())
    } else {
        Ok(vec![])
    }
}

fn session_name_hook(session_prefix: String) -> Result<Vec<String>, Box<dyn Error>> {
    let session_name_path = vmux_hook_path("session_name")?;
    let res = if Path::new(&session_name_path).is_file() {
        let output = Command::new("/usr/bin/bash")
            .arg(&session_name_path)
            .arg(&session_prefix)
            .output()?;
        output
            .stdout
            .lines()
            .map(|x| x.unwrap())
            .into_iter()
            .collect()
    } else {
        vec![]
    };
    Ok(res)
}

fn list(previous_session_name: String) -> Result<Vec<String>, Box<dyn Error>> {
    let mut res = list_sessions_with_baus(previous_session_name)?;
    let hook = list_sessions_name_hook()?;
    let cmds = vec!["Detach".to_string(), "New".to_string()];
    res.extend(cmds);
    res.extend(hook);
    Ok(res)
}

fn attach(session: String) -> Result<(), Box<dyn Error>> {
    let empty = Vec::new();
    let empty2 = HashMap::new();
    diss::run(&session, &empty, empty2, Some("g".into()))?;
    selector(session)
}

fn run_switch_result(res: String) -> Result<(), Box<dyn Error>> {
    let new2_reg = Regex::new(r"^New: ")?;
    if res == "Detach" {
        println!("done")
    } else if res == "New" {
        let mut line = String::new();
        println!("enter session name:");
        std::io::stdin().read_line(&mut line)?;
        trim_newline(&mut line);
        start_session(line)?;
    } else if new2_reg.is_match(&res) {
        start_session(res.replace("New: ", ""))?;
    } else {
        let re = Regex::new(".*\t")?;
        save_with_baus(res.clone())?;
        attach(re.replace(&res, "").to_string())?;
    }
    Ok(())
}

pub fn selector(previous_session_name: String) -> Result<(), Box<dyn Error>> {
    print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));

    let lines = list(previous_session_name)?;

    let s = termion::terminal_size()?;
    let columns = s.0;
    let lines_n = s.1;

    let height = lines.len() as u16;
    let width = lines
        .clone()
        .into_iter()
        .fold(0, |acc, x| std::cmp::max(acc, x.len())) as u16;

    let margin_h = if columns > width + 8 {
        columns - width - 8
    } else {
        0
    };

    let margin_r = margin_h / 5;
    let margin_l = 4 * margin_h / 5;

    // adjust for skim UI components
    let height = height + 2;

    let margin_v = if lines_n > height {
        (lines_n - height) / 2
    } else {
        lines_n / 5
    };

    let mut options = SkimOptions::<'_> {
        no_clear_start: true,
        nosort: true,
        ..Default::default()
    };
    options.no_clear_start = true;
    options.nosort = true;
    let margin = format!("{},{},{},{}", margin_v, margin_r, margin_v, margin_l);
    options.margin = Some(&margin);
    if let Some(img) = random_image()? {
        render_image_fitting_terminal(&img)
    }

    let item_reader_option = SkimItemReaderOption::default();

    let cmd_collector = Rc::new(RefCell::new(SkimItemReader::new(item_reader_option)));
    options.cmd_collector = cmd_collector;

    let (tx, rx): (SkimItemSender, SkimItemReceiver) = unbounded();

    for line in lines {
        tx.send(Arc::new(Item { text: line }))?;
    }
    drop(tx);

    let selected_items = Skim::run_with(&options, Some(rx))
        .map(|out| out.selected_items)
        .unwrap_or_else(Vec::new);
    for item in selected_items.iter() {
        let res = item.output();
        run_switch_result(res.to_string())?;
    }
    Ok(())
}

fn help() {
    println!("please provide an action (new|attach|list)");
}

fn start_session(session_prefix: String) -> Result<(), Box<dyn Error>> {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let session_suffix = "-vmux-session";
    let id = format!("{}-{}", session_prefix, since_the_epoch.as_secs());
    let server_file = format!("/tmp/vim-server-{}", id);
    let session_name = format!("{}{}", id, session_suffix);
    // TODO select vim/neovim via VMUX_EDITOR?
    let env_regx = Regex::new(r"^([^=]*)=(.*)$")?;
    let lines: Vec<String> = session_name_hook(session_prefix)?;
    let mut env_vars: HashMap<String, String> = lines
        .into_iter()
        .map(|line| {
            let x = env_regx.captures(&line).unwrap();
            (
                x.get(1).map_or("", |m| m.as_str()).to_string(),
                x.get(2).map_or("", |m| m.as_str()).to_string(),
            )
        })
        .collect();
    env_vars.insert("vmux_server_file".to_string(), server_file.clone());

    let command = vec![
        "nvim".to_string(),
        "--cmd".to_string(),
        "let g:confirm_quit_nomap = 0".to_string(),
        "--cmd".to_string(),
        format!("let g:server_addr = serverstart('{}')", server_file),
    ];
    diss::run(&session_name, &command, env_vars, Some("g".into()))?;
    selector(session_name)
}

async fn send(command: &str) -> Result<(), Box<dyn Error>> {
    let vmux_server_file = env::var("vmux_server_file")?;
    let handler = Dummy::new();
    let path = Path::new(&vmux_server_file);
    let stream = Endpoint::connect(path).await?;
    let (reader, writer) = split(stream);
    let (neovim, io) = Neovim::<Compat<WriteHalf<Connection>>>::new(
        reader.compat(),
        writer.compat_write(),
        handler,
    );
    let _io_handle = tokio::spawn(io);
    neovim.command(command).await?;
    Ok(())
}

async fn edit(edited_file_path: &str) -> Result<(), Box<dyn Error>> {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let done_file_path = format!("/tmp/vmux_lock_{}", since_the_epoch.as_millis());
    send(&format!(
        ":let g:vmux_edited_file_path = \"{}\"",
        edited_file_path
    ))
    .await?;
    send(&format!(
        ":let g:vmux_done_file_path = \"{}\"",
        done_file_path
    ))
    .await?;
    send(&format!(":winc l|split {}", edited_file_path)).await?;
    send(":call VmuxAddDoneEditingCallback()").await?;
    println!("waiting for {} to be created...", done_file_path);
    while !Path::new(&done_file_path).exists() {
        std::thread::sleep(Duration::from_millis(200));
    }
    Ok(())
}

fn run_or_selector(f: impl Fn(String) -> Result<(), Box<dyn Error>>) -> Result<(), Box<dyn Error>> {
    match std::env::args().nth(2) {
        Some(session_prefix) => f(session_prefix),
        None => selector("".to_string()),
    }
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let arg1 = std::env::args().nth(1);
    match arg1 {
        Some(action) => {
            if action == "select" {
                run_or_selector(selector)?;
            } else if action == "attach" {
                run_or_selector(attach)?;
            } else if action == "new" {
                run_or_selector(start_session)?;
            } else if action == "list" {
                show_session_list()?;
            } else if action == "send" {
                let args: Vec<String> = env::args().collect();
                let command = &args[2..].join(" ");
                send(command).await?;
            } else if action == "edit" {
                let args: Vec<String> = env::args().collect();
                let edited_file_path = &args[2..].join(" ");
                edit(edited_file_path).await?;
            } else {
                help();
            }
        }
        None => help(),
    }
    Ok(())
}
