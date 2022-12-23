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
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use std::{error::Error, fmt};
use tokio::io::{split, WriteHalf};
use tokio::runtime::Handle;
use tokio_util::compat::Compat;
use tokio_util::compat::TokioAsyncReadCompatExt;
use tokio_util::compat::TokioAsyncWriteCompatExt; // 1.0.2 // 1.0.2

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

#[derive(Debug, Clone)]
struct SessionNotFound {
    display_name: String,
}

impl Error for SessionNotFound {}

impl fmt::Display for SessionNotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "seession not found {}", self.display_name)
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

struct Session {
    name: String,
    display_name: String,
}

fn list_sessions(session_group: &String) -> Result<Vec<Session>, Box<dyn Error>> {
    let session_regx = Regex::new(&format!(".*{}", session_suffix(&session_group)))?;
    Ok(diss::list_sessions()?
        .into_iter()
        .filter(|x| session_regx.is_match(x))
        .map(|x| Session {
            name: x.clone(),
            display_name: x.replace(&session_suffix(&session_group), ""),
        })
        .collect())
}

fn get_session_full_name(
    display_name: String,
    session_group: &String,
) -> Result<String, Box<dyn Error>> {
    list_sessions(session_group)?
        .into_iter()
        .filter(|x| x.display_name == display_name)
        .map(|x| x.name)
        .collect::<Vec<String>>()
        .get(0)
        .map(|x| x.clone())
        .ok_or(Box::new(SessionNotFound { display_name }))
}

fn show_session_list(session_group: String) -> Result<(), Box<dyn Error>> {
    for session in list_sessions(&session_group)? {
        println!("{}", session.display_name);
    }
    Ok(())
}

fn get_session_display_name(
    name: String,
    session_list: &Vec<Session>,
) -> Result<String, Box<dyn Error>> {
    session_list
        .into_iter()
        .filter(|x| x.name == name)
        .map(|x| x.display_name.clone())
        .collect::<Vec<String>>()
        .get(0)
        .map(|x| x.clone())
        .ok_or(Box::new(SessionNotFound { display_name: name }))
}

fn list_sessions_with_baus(
    previous_session_name: String,
    session_group: String,
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
    let sessions_list_detailed = list_sessions(&session_group)?;
    let previous_session_name =
        get_session_display_name(previous_session_name, &sessions_list_detailed)
            .ok()
            .unwrap_or("".to_string());
    let a = list_sessions(&session_group)?
        .into_iter()
        .map(|x| x.name)
        .collect();
    let sessions_list = baus::sort(&args, a, &mut lines_backup, &cache_file_path)?
        .into_iter()
        .map(|x| get_session_display_name(x, &sessions_list_detailed).unwrap())
        .collect::<Vec<String>>();
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
    hook_extension: &str,
    configuration_directory_path: Option<String>,
) -> Result<String, Box<dyn Error>> {
    Ok(format!(
        "{}/hooks/{}{}",
        config_dir_path(configuration_directory_path)?,
        hook_name,
        hook_extension
    ))
}

fn session_hook(
    hook_name: &str,
    configuration_directory_path: &Option<String>,
) -> Result<Option<PathBuf>, Box<dyn Error>> {
    let list_session_name_path =
        vmux_hook_path(hook_name, "", configuration_directory_path.clone())?;
    let list_session_name_f = Path::new(&list_session_name_path);
    let list_session_name_sh_path =
        vmux_hook_path(hook_name, ".sh", configuration_directory_path.clone())?;
    let list_session_name_sh_f = Path::new(&list_session_name_sh_path);
    if list_session_name_f.is_file() {
        Ok(Some(list_session_name_f.to_path_buf()))
    } else if list_session_name_sh_f.is_file() {
        Ok(Some(list_session_name_sh_f.to_path_buf()))
    } else {
        Ok(None)
    }
}

fn list_sessions_name_hook(
    configuration_directory_path: Option<String>,
) -> Result<Vec<String>, Box<dyn Error>> {
    if let Some(list_session_name_path) =
        session_hook("list_sessions_names", &configuration_directory_path)?
    {
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
    } else {
        Ok(vec![])
    }
}

fn session_name_hook(
    session_prefix: String,
    configuration_directory_path: Option<String>,
) -> Result<Vec<String>, Box<dyn Error>> {
    if let Some(session_name_path) = session_hook("session_name", &configuration_directory_path)? {
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
    } else {
        Ok(vec![])
    }
}

fn list(
    previous_session_name: String,
    configuration_directory_path: Option<String>,
    session_group: String,
) -> Result<Vec<String>, Box<dyn Error>> {
    let mut res = list_sessions_with_baus(previous_session_name, session_group)?;
    let hook = list_sessions_name_hook(configuration_directory_path)?;
    let cmds = vec!["Detach".to_string(), "New".to_string()];
    res.extend(cmds);
    res.extend(hook);
    Ok(res)
}

fn enable_mouse() {
    // https://stackoverflow.com/questions/5966903/how-to-get-mousemove-and-mouseclick-in-bash
    print!("\x1b[?1000h");
}

fn attach(
    handle: &Handle,
    session_prefix: String,
    escape_key: Option<String>,
    configuration_directory_path: Option<String>,
    session_group: String,
) -> Result<(), Box<dyn Error>> {
    let session = format!("{}{}", session_prefix, session_suffix(&session_group));
    let (_, server_file) = get_server_file(&session_prefix, &session_group)?;
    run_diss_and_selector(
        &handle,
        server_file,
        &session,
        &Vec::new(),
        HashMap::new(),
        escape_key.clone(),
        configuration_directory_path,
        session_group,
    )
}

fn run_diss_and_selector(
    handle: &Handle,
    server_file: String,
    session_name: &str,
    command: &[String],
    env: HashMap<String, String>,
    escape_key: Option<String>,
    configuration_directory_path: Option<String>,
    session_group: String,
) -> Result<(), Box<dyn Error>> {
    enable_mouse();
    trigger_in_vim_hook(handle, server_file.clone(), "Attach".into())?;
    diss::run(&session_name, &command, env, escape_key.clone())?;
    trigger_in_vim_hook(handle, server_file, "Detach".into())?;
    selector(
        handle,
        session_name.into(),
        escape_key,
        configuration_directory_path,
        session_group,
    )
}

fn run_switch_result(
    handle: &Handle,
    res: String,
    escape_key: Option<String>,
    configuration_directory_path: Option<String>,
    session_group: String,
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
            handle,
            line,
            escape_key,
            configuration_directory_path,
            session_group,
        )?;
    } else if new2_reg.is_match(&res) {
        start_session(
            handle,
            res.replace("New: ", ""),
            escape_key,
            configuration_directory_path,
            session_group,
        )?;
    } else {
        let full_name = get_session_full_name(res.clone(), &session_group)?;
        save_with_baus(full_name.clone())?;
        attach(
            handle,
            res,
            escape_key,
            configuration_directory_path,
            session_group,
        )?;
    }
    Ok(())
}

pub fn selector(
    handle: &Handle,
    previous_session_name: String,
    escape_key: Option<String>,
    configuration_directory_path: Option<String>,
    session_group: String,
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
        no_mouse: false,
        ..Default::default()
    };
    options.no_clear_start = true;
    options.nosort = true;
    options.no_mouse = false;
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
            handle,
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

fn session_suffix(session_group: &String) -> String {
    format!("-vmux-session{}", session_group.clone())
}

fn sessions_contains_full(sessions: &Vec<Session>, full_name: &String) -> bool {
    sessions.iter().filter(|x| &x.name == full_name).count() > 0
}

fn unique_prefix(
    session_prefix: &String,
    session_group: &String,
) -> Result<String, Box<dyn Error>> {
    let sessions = list_sessions(session_group)?;
    if !sessions_contains_full(&sessions, &session_prefix) {
        Ok(session_prefix.to_string())
    } else {
        let mut i = 0;
        let mut full;
        loop {
            full = format!("{}-{}", session_prefix, i);
            if !sessions_contains_full(&sessions, &full) {
                break;
            }
            i += 1;
        }
        Ok(full)
    }
}

fn get_server_file(
    session_prefix: &String,
    session_group: &String,
) -> Result<(String, String), Box<dyn Error>> {
    let id = unique_prefix(&session_prefix, &session_group)?;
    Ok((id.clone(), format!("/tmp/vim-server-{}", &id)))
}

fn start_session(
    handle: &Handle,
    session_prefix: String,
    escape_key: Option<String>,
    configuration_directory_path: Option<String>,
    session_group: String,
) -> Result<(), Box<dyn Error>> {
    let (id, server_file) = get_server_file(&session_prefix, &session_group)?;
    if Path::new(&server_file).exists() {
        fs::remove_file(&server_file)?;
    }
    let session_name = format!("{}{}", id, session_suffix(&session_group));
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
            server_file.clone(),
        ]
    };
    env_vars.get("VMUX_ADDITIONAL_ARGUMENTS").map(|args| {
        args.split(" ")
            .for_each(|arg| command.push(arg.to_string()))
    });
    run_diss_and_selector(
        &handle,
        server_file,
        &session_name,
        &command,
        env_vars,
        escape_key.clone(),
        configuration_directory_path,
        session_group,
    )
}

fn log(string: String) {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("/tmp/my-file")
        .unwrap();

    _ = writeln!(file, "{}", string)
}

fn trigger_in_vim_hook(
    handle: &Handle,
    server_file: String,
    hook_kind: String,
) -> Result<(), Box<dyn Error>> {
    log(format!(
        "trigger_in_vim_hook {} {}\n",
        server_file, hook_kind
    ));
    send_sync(
        handle,
        format!(":call Vmux{}Callback()", hook_kind),
        Some(server_file.into()),
    );
    Ok(())
}

fn send_sync(handle: &Handle, command: String, vmux_server_file: Option<String>) {
    let join_handle = handle.spawn(async move {
        let _ = send(command, vmux_server_file.clone()).await;
    });
    futures::executor::block_on(join_handle).unwrap();
}

async fn send(command: String, vmux_server_file: Option<String>) -> Result<(), Box<dyn Error>> {
    let vmux_server_file =
        vmux_server_file.unwrap_or_else(|| env::var("vmux_server_file").unwrap());
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
    neovim.command(&command).await?;
    Ok(())
}

async fn edit(edited_file_path: &str) -> Result<(), Box<dyn Error>> {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let done_file_path = format!("/tmp/vmux_lock_{}", since_the_epoch.as_millis());
    send(
        format!(":let g:vmux_edited_file_path = \"{}\"", edited_file_path),
        None,
    )
    .await?;
    send(
        format!(":let g:vmux_done_file_path = \"{}\"", done_file_path),
        None,
    )
    .await?;
    send(format!(":winc l|split {}", edited_file_path), None).await?;
    send(":call VmuxAddDoneEditingCallback()".to_string(), None).await?;
    println!("waiting for {} to be created...", done_file_path);
    while !Path::new(&done_file_path).exists() {
        std::thread::sleep(Duration::from_millis(200));
    }
    Ok(())
}

fn run_or_selector(
    handle: &Handle,
    f: impl Fn(&Handle, String, Option<String>, Option<String>, String) -> Result<(), Box<dyn Error>>,
    args: Args,
) -> Result<(), Box<dyn Error>> {
    match args.command.get(1) {
        Some(session_prefix) => f(
            handle,
            session_prefix.to_string(),
            args.escape_key,
            args.configuration_directory_path,
            args.session_group,
        ),
        None => selector(
            handle,
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
    #[clap(short, long, value_parser, default_value = "default")]
    session_group: String,

    // command
    command: Vec<String>,
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let handle = Handle::current();
    let args = Args::parse();
    let arg1 = args.command.get(0);
    match arg1 {
        Some(action) => {
            if action == "select" {
                run_or_selector(&handle, selector, args)?;
            } else if action == "attach" {
                run_or_selector(&handle, attach, args)?;
            } else if action == "new" {
                run_or_selector(&handle, start_session, args)?;
            } else if action == "list" {
                show_session_list(args.session_group)?;
            } else if action == "send" {
                let command = &args.command[1..].join(" ");
                send(command.to_string(), None).await?;
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
