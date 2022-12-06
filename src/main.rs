extern crate baus;
extern crate skim;
use blockish::render_image_fitting_terminal;
use clap::Parser;
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
struct ConfigDirNotFound;

impl Error for ConfigDirNotFound {}

impl fmt::Display for ConfigDirNotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "home directory was not retrieved")
    }
}

fn random_image(
    configuration_directory_path: Option<String>,
) -> Result<Option<String>, Box<dyn Error>> {
    let mut rng = rand::thread_rng();
    let wallpapers_dir = vmux_wallpapers_path(configuration_directory_path)?;
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

fn list_sessions(session_group: Option<String>) -> Result<Vec<String>, Box<dyn Error>> {
    let session_regx = Regex::new(&format!(
        ".*vmux-session{}.*",
        session_group.unwrap_or_else(|| "".to_string())
    ))?;
    Ok(diss::list_sessions()?
        .into_iter()
        .filter(|x| session_regx.is_match(x))
        .collect())
}

fn show_session_list(session_group: Option<String>) -> Result<(), Box<dyn Error>> {
    for session in list_sessions(session_group)? {
        println!("{}", session);
    }
    Ok(())
}

fn list_sessions_with_baus(
    previous_session_name: String,
    session_group: Option<String>,
) -> Result<Vec<String>, Box<dyn Error>> {
    let args = baus::Args {
        name: "vmux".to_string(),
        action: baus::Action::Sort,
        value: baus::SavedValue::Timestamp,
        desc: true,
        cleanup: true,
    };
    let cache_file_path = baus::get_cache_file_path(&args)?;
    let mut lines_backup = baus::get_lines_backup(&cache_file_path)?;
    let sessions_list = list_sessions(session_group)?;
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

fn config_dir_path(configuration_directory_path: Option<String>) -> Result<String, Box<dyn Error>> {
    match configuration_directory_path {
        Some(dir) => Ok(dir),
        None => Ok(format!(
            "{}/vmux/",
            dirs::config_dir()
                .ok_or_else(|| Box::new(ConfigDirNotFound))?
                .to_string_lossy()
                .to_string()
        )),
    }
}

fn vmux_wallpapers_path(
    configuration_directory_path: Option<String>,
) -> Result<String, Box<dyn Error>> {
    Ok(format!(
        "{}/wallpapers/",
        config_dir_path(configuration_directory_path)?
    ))
}

fn vmux_hook_path(
    hook_name: &str,
    configuration_directory_path: Option<String>,
) -> Result<String, Box<dyn Error>> {
    Ok(format!(
        "{}/hooks/{}.sh",
        config_dir_path(configuration_directory_path)?,
        hook_name
    ))
}

fn list_sessions_name_hook(
    configuration_directory_path: Option<String>,
) -> Result<Vec<String>, Box<dyn Error>> {
    let list_session_name_path =
        vmux_hook_path("list_sessions_names", configuration_directory_path)?;
    let list_session_name_f = Path::new(&list_session_name_path);
    if list_session_name_f.is_file() {
        let output = Command::new(list_session_name_path).output()?;
        Ok(output
            .stdout
            .lines()
            .map(|x| format!("New: {}", x.unwrap()))
            .collect())
    } else {
        Ok(vec![])
    }
}

fn session_name_hook(
    session_prefix: String,
    configuration_directory_path: Option<String>,
) -> Result<Vec<String>, Box<dyn Error>> {
    let session_name_path = vmux_hook_path("session_name", configuration_directory_path)?;
    let res = if Path::new(&session_name_path).is_file() {
        let output = Command::new(session_name_path)
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

fn list(
    previous_session_name: String,
    configuration_directory_path: Option<String>,
    session_group: Option<String>,
) -> Result<Vec<String>, Box<dyn Error>> {
    let mut res = list_sessions_with_baus(previous_session_name, session_group)?;
    let hook = list_sessions_name_hook(configuration_directory_path)?;
    let cmds = vec!["Detach".to_string(), "New".to_string()];
    res.extend(cmds);
    res.extend(hook);
    Ok(res)
}

fn attach(
    session: String,
    escape_key: Option<String>,
    configuration_directory_path: Option<String>,
    session_group: Option<String>,
) -> Result<(), Box<dyn Error>> {
    let empty = Vec::new();
    let empty2 = HashMap::new();
    diss::run(&session, &empty, empty2, escape_key.clone())?;
    selector(
        session,
        escape_key,
        configuration_directory_path,
        session_group,
    )
}

fn run_switch_result(
    res: String,
    escape_key: Option<String>,
    configuration_directory_path: Option<String>,
    session_group: Option<String>,
) -> Result<(), Box<dyn Error>> {
    let new2_reg = Regex::new(r"^New: ")?;
    if res == "Detach" {
        println!("done")
    } else if res == "New" {
        let mut line = String::new();
        println!("enter session name:");
        std::io::stdin().read_line(&mut line)?;
        trim_newline(&mut line);
        start_session(
            line,
            escape_key,
            configuration_directory_path,
            session_group,
        )?;
    } else if new2_reg.is_match(&res) {
        start_session(
            res.replace("New: ", ""),
            escape_key,
            configuration_directory_path,
            session_group,
        )?;
    } else {
        let re = Regex::new(".*\t")?;
        save_with_baus(res.clone())?;
        attach(
            re.replace(&res, "").to_string(),
            escape_key,
            configuration_directory_path,
            session_group,
        )?;
    }
    Ok(())
}

pub fn selector(
    previous_session_name: String,
    escape_key: Option<String>,
    configuration_directory_path: Option<String>,
    session_group: Option<String>,
) -> Result<(), Box<dyn Error>> {
    print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));

    let lines = list(
        previous_session_name,
        configuration_directory_path.clone(),
        session_group.clone(),
    )?;

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
    if let Some(img) = random_image(configuration_directory_path.clone())? {
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
        run_switch_result(
            res.to_string(),
            escape_key.clone(),
            configuration_directory_path.clone(),
            session_group.clone(),
        )?;
    }
    Ok(())
}

fn help() {
    println!("please provide an action (new|attach|list)");
}

fn start_session(
    session_prefix: String,
    escape_key: Option<String>,
    configuration_directory_path: Option<String>,
    session_group: Option<String>,
) -> Result<(), Box<dyn Error>> {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let session_suffix = format!(
        "-vmux-session{}",
        session_group.clone().unwrap_or("".to_string())
    );
    let id = format!("{}-{}", session_prefix, since_the_epoch.as_secs());
    let server_file = format!("/tmp/vim-server-{}", id);
    let session_name = format!("{}{}", id, session_suffix);
    let env_regx = Regex::new(r"^([^=]*)=(.*)$")?;
    let lines: Vec<String> =
        session_name_hook(session_prefix, configuration_directory_path.clone())?;
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

    let vmux_editor = env::var("VMUX_EDITOR").unwrap_or("nvim".to_string());
    let mut command = if vmux_editor.contains("nvim") {
        vec![
            vmux_editor,
            "--cmd".to_string(),
            "let g:confirm_quit_nomap = 0".to_string(),
            "--cmd".to_string(),
            format!("let g:server_addr = serverstart('{}')", server_file),
        ]
    } else {
        vec![
            vmux_editor,
            "--cmd".to_string(),
            "let g:confirm_quit_nomap = 0".to_string(),
            "--servername".to_string(),
            server_file,
        ]
    };
    env_vars.get("VMUX_ADDITIONAL_ARGUMENTS").map(|args| {
        args.split(" ")
            .for_each(|arg| command.push(arg.to_string()))
    });
    diss::run(&session_name, &command, env_vars, escape_key.clone())?;
    selector(
        session_name,
        escape_key,
        configuration_directory_path,
        session_group,
    )
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

fn run_or_selector(
    f: impl Fn(String, Option<String>, Option<String>, Option<String>) -> Result<(), Box<dyn Error>>,
    args: Args,
) -> Result<(), Box<dyn Error>> {
    match args.command.get(1) {
        Some(session_prefix) => f(
            session_prefix.to_string(),
            args.escape_key,
            args.configuration_directory_path,
            args.session_group,
        ),
        None => selector(
            "".to_string(),
            args.escape_key,
            args.configuration_directory_path,
            args.session_group,
        ),
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    // escape key
    #[clap(short, long, value_parser)]
    escape_key: Option<String>,

    // configuration directory path
    #[clap(short, long, value_parser)]
    configuration_directory_path: Option<String>,

    // name of the group of session
    #[clap(short, long, value_parser)]
    session_group: Option<String>,

    // command
    command: Vec<String>,
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let arg1 = args.command.get(0);
    match arg1 {
        Some(action) => {
            if action == "select" {
                run_or_selector(selector, args)?;
            } else if action == "attach" {
                run_or_selector(attach, args)?;
            } else if action == "new" {
                run_or_selector(start_session, args)?;
            } else if action == "list" {
                show_session_list(args.session_group)?;
            } else if action == "send" {
                let command = &args.command[1..].join(" ");
                send(command).await?;
            } else if action == "edit" {
                let edited_file_path = &args.command[1..].join(" ");
                edit(edited_file_path).await?;
            } else {
                help();
            }
        }
        None => help(),
    }
    Ok(())
}
