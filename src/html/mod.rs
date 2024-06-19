use std::{fs::{File, OpenOptions}, io::{Read, Seek, SeekFrom, Write}, path::Path, sync::{Arc, Mutex, MutexGuard}, time::Instant};

use hyper::StatusCode;
use lazy_static::lazy_static;
use main_menu::MainMenu;
use rand::{distributions::Alphanumeric, thread_rng, Rng};

mod init_menu;
pub mod main_menu;
mod manager_menu;

pub fn generate_random_string(n: usize) -> String {
    let mut rng = thread_rng();

    (0..n).map(|_| rng.sample(Alphanumeric) as char).collect()
}

pub struct Session {
    password: String,
    time: Instant,
}

impl Session {
    pub fn new() -> Self {
        Self {
            password: generate_random_string(16),
            time: Instant::now(),
        }
    }

    pub fn update(&mut self) {
        *self = Self::new();
    }

    pub fn get_password<'a>(&'a self) -> &'a str {
        &self.password
    }

    pub fn validate(&self, password: &[u8]) -> bool {
        self.time.elapsed().as_secs() < 3600 && self.password.as_bytes() == password
    }
}

pub struct Htmls {
    pub main_menu: String,
    pub main_menu_struct: MainMenu,
    pub password: String,
    pub manager_password: String,
    pub session: Session,
}

lazy_static! {
    pub static ref INIT_MENU: String = init_menu::get_init_menu_html();
    pub static ref HTMLS: Arc<Mutex<Htmls>> = Arc::new(Mutex::new(Htmls::new()));
}

impl Htmls {
    pub fn new() -> Self {
        let main_menu: MainMenu = {
            let mut file = match File::open("main.json") {
                Ok(file) => file,
                Err(_) => {
                    let mut file = OpenOptions::new()
                        .write(true)
                        .read(true)
                        .create(true)
                        .open("main.json")
                        .unwrap();
                    let json = serde_json::to_string(&MainMenu(Vec::new())).unwrap();
                    file.write_all(json.as_bytes()).unwrap();
                    file.seek(SeekFrom::Start(0)).unwrap();
                    file
                }
            };
            let mut json = String::new();
            file.read_to_string(&mut json).unwrap();

            serde_json::from_str(json.trim()).unwrap()
        };

        let (password, manager_password) = {
            let password_path = Path::new("password.txt");
            let manager_password_path = Path::new("manager_password.txt");
            if !password_path.is_file() {
                File::create(password_path).unwrap();
            }
            if !manager_password_path.is_file() {
                File::create(manager_password_path).unwrap();
            }
            let mut password = String::new();
            File::open(password_path).unwrap().read_to_string(&mut password).unwrap();
            password = password.trim().to_string();
            let mut manager_password = String::new();
            File::open(manager_password_path).unwrap().read_to_string(&mut manager_password).unwrap();
            manager_password = manager_password.trim().to_string();

            (password, manager_password)
        };

        Self {
            main_menu: main_menu.to_html(),
            main_menu_struct: main_menu,
            password,
            manager_password,
            session: Session::new(),
        }
    }

    pub fn get() -> Result<MutexGuard<'static, Htmls>, StatusCode> {
        match HTMLS.lock() {
            Ok(htmls) => Ok(htmls),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    pub fn update_main_menu(&mut self) {
        self.main_menu = self.main_menu_struct.to_html();
    }
}