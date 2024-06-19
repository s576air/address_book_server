use std::{fs::{self, File, OpenOptions}, io::{Read, Write}, net::SocketAddr, path::Path};

use axum::{body::{Body, Bytes}, http::request::Parts, response::{Html, IntoResponse, Response}, routing::{get, post}, Form, Router};
use html::{generate_random_string, INIT_MENU};
use hyper::{HeaderMap, StatusCode, Uri};
use serde::Deserialize;

mod html;
mod manager;

use crate::html::Htmls;


fn parse_addr(s: &str) -> Option<SocketAddr> {
    let (ip, port) = s.trim().split_once(':')?;
    let mut ip = ip.split('.');
    let ip = [
        ip.next()?.parse::<u8>().ok()?,
        ip.next()?.parse::<u8>().ok()?,
        ip.next()?.parse::<u8>().ok()?,
        ip.next()?.parse::<u8>().ok()?,
    ];
    let port = port.parse::<u16>().ok()?;

    Some(SocketAddr::from((ip, port)))
}

#[tokio::main]
async fn main() {
    {
        let path = Path::new("image");
        if !path.is_dir() {
            fs::create_dir(path).unwrap();
        }
    }

    {
        let path = Path::new("addr.txt");
        if !path.is_file() {
            File::create(path).unwrap();
        }
    }

    let app = Router::new()
        .route("/", get(init_menu_handler))
        .route("/", post(post_handler))
        .route("/image", get(profile_image_handler))
        .route("/upload", post(upload_handler))
        .route("/password", get(password_handler))
        .route("/logo.png", get(logo_image))
        .route("/arrow.png", get(arrow_image))
        .route("/search.png", get(search_image))
        .route("/plus.png", get(plus_image))
        .route("/white_plus.png", get(white_plus_image))
        .route("/edit.png", get(edit_image))
        .route("/white_edit.png", get(white_edit_image))
        .route("/delete.png", get(delete_image))
        .route("/white_delete.png", get(white_delete_image));
    
    let addr = {
        let mut content = String::new();
        File::open("addr.txt").unwrap().read_to_string(&mut content).unwrap();
        println!("addr.txt 내용: {content}");
        match parse_addr(&content) {
            Some(addr) => addr,
            None => SocketAddr::from(([127, 0, 0, 1], 3000)),
        }
    };
    println!("{}로 서버를 엽니다.", addr);

    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn init_menu_handler() -> Html<&'static str> {
    // 기본 메뉴 반환
    Html(&INIT_MENU)
}

#[derive(Deserialize)]
struct Data {
    data: String,
}

async fn post_handler(Form(data): Form<Data>) -> Result<Html<String>, StatusCode> {
    let body = data.data;
    if body.chars().any(|char| matches!(char, '<' | '>' | '"' | '\'' | '&')) { return Err(StatusCode::BAD_REQUEST) }
    let mut body = body.split("#$#");

    let mut htmls = Htmls::get()?;

    match body.next() { // password
        Some(password) => {
            if password == htmls.password {
                let html = htmls.main_menu.to_string();
                return Ok(Html(html))
            } else if password == htmls.manager_password {
                htmls.session.update();
                let manager_menu = htmls.main_menu_struct.to_manager_html(htmls.session.get_password());
                return Ok(Html(manager_menu))
            } else if !htmls.session.validate(password.as_bytes()) {
                return Ok(Html(r#"<script>window.location.href=""</script>"#.to_string()))
            }
        },
        None => return Err(StatusCode::BAD_REQUEST)
    }

    {
        let command = body.next();
        if command.is_none() { return Err(StatusCode::BAD_REQUEST) }
        let command = command.unwrap();
        
        let result = match command { // command
            "editGroup" => manager::edit_group(body, &mut htmls),
            "deleteGroup" => manager::delete_group(body, &mut htmls),
            "addGroup" => manager::add_group(body, &mut htmls),
            "editProfile" => manager::edit_profile(body, &mut htmls),
            "deleteProfile" => manager::delete_profile(body, &mut htmls),
            "addProfile" => manager::add_profile(body, &mut htmls),
            _ => return Err(StatusCode::BAD_REQUEST),
        };

        match result {
            Some(()) => {
                {
                    let Ok(mut file) = OpenOptions::new()
                        .write(true)
                        .truncate(true)
                        .open("main.json")
                    else {
                        return Err(StatusCode::INTERNAL_SERVER_ERROR);
                    };
                    let main_menu_json = serde_json::to_string(&htmls.main_menu_struct).unwrap();
                    let _ = file.write_all(main_menu_json.as_bytes());
                }
                let manager_menu = htmls.main_menu_struct.to_manager_html(htmls.session.get_password());
                Ok(Html(manager_menu))
            },
            None => Err(StatusCode::BAD_REQUEST),
        }
    }    
}

async fn profile_image_handler(parts: Parts) -> Result<Vec<u8>, StatusCode> {
    let Some(query) = parts.uri.query() else { return Err(StatusCode::BAD_REQUEST) };
    if query.as_bytes().len() != 8 { return Err(StatusCode::BAD_REQUEST) }
    let mut content = Vec::new();
    let Ok(mut file) = File::open(format!("image/{query}.png")) else { return Err(StatusCode::BAD_REQUEST) };
    let Ok(_) = file.read_to_end(&mut content) else { return Err(StatusCode::BAD_REQUEST) };

    Ok(content)
}

fn bytes_to_usize(bytes: &[u8]) -> Option<usize> {
    let mut value = 0;
    for i in 0..bytes.len() {
        let mut byte = bytes[i];
        if byte < b'0' { return None } 
        byte -= b'0';
        if byte > 9 { return None }
        value = value * 10 + byte as usize;
    }

    Some(value)
}

fn upload_handler_inner(header: HeaderMap, image_bytes: Bytes) -> Option<()> {
    let password: &axum::http::HeaderValue = header.get("password")?;
    let mut htmls = Htmls::get().ok()?;
    if !htmls.session.validate(password.as_bytes()) { return None }
    let group_number = bytes_to_usize(header.get("group")?.as_bytes())?;
    let profile_number = bytes_to_usize(header.get("profile")?.as_bytes())?;
    let image = &image_bytes[..];
    loop {
        // 순서 잘 조정바람
        let new_name = generate_random_string(8);
        let new_path = format!("image/{new_name}.png");
        if fs::metadata(&new_path).is_err() {
            // 파일에 저장(이미지)
            let mut image_file = File::create(&new_path).ok()?;
            image_file.write_all(image).ok()?;

            let old_path = &mut htmls.main_menu_struct.0.get_mut(group_number)?.profiles.get_mut(profile_number)?.image_path;
            if !old_path.is_empty() {
                let _ = fs::remove_file(format!("image/{old_path}.png"));
            }
            old_path.clear();
            old_path.push_str(&new_name);
            // json으로 저장
            let mut json_file = OpenOptions::new().write(true).truncate(true).open("main.json").ok()?;
            let main_menu_json = serde_json::to_string(&htmls.main_menu_struct).ok()?;
            json_file.write_all(main_menu_json.as_bytes()).ok()?;

            break
        }
    }println!("모든 과정 성공적으로 완료");

    Some(())
}

async fn upload_handler(header: HeaderMap, image_bytes: Bytes) -> Result<(), StatusCode> {
    match upload_handler_inner(header, image_bytes) {
        Some(()) => Ok(()),
        None => Err(StatusCode::BAD_REQUEST),
    }
}

// 비번 바꾸는 법: 페이지에 주소/password?세션 비밀번호$새 비밀번호$새 관리자 비밀번호
async fn password_handler(uri: Uri) -> StatusCode {
    let Some(query) = uri.query() else { return StatusCode::BAD_REQUEST };
    let mut query = query.split('$');
    let Some(session_password) = query.next() else { return StatusCode::BAD_REQUEST };
    let Ok(mut htmls) = Htmls::get() else { return StatusCode::BAD_REQUEST };
    if !htmls.session.validate(session_password.as_bytes()) { return StatusCode::NON_AUTHORITATIVE_INFORMATION };

    let Some(new_password) = query.next() else { return StatusCode::BAD_REQUEST };
    let Some(new_manager_password) = query.next() else { return StatusCode::BAD_REQUEST };
    if query.next().is_some() { return StatusCode::BAD_REQUEST };

    if !new_password.is_empty() {
        htmls.password.clear();
        htmls.password.push_str(new_password);
        let Ok(mut file) = File::create("password.txt") else { return StatusCode::INTERNAL_SERVER_ERROR };
        let Ok(_) = file.write_all(new_password.as_bytes()) else { return StatusCode::INTERNAL_SERVER_ERROR };
    }
    if !new_manager_password.is_empty() {
        htmls.manager_password.clear();
        htmls.manager_password.push_str(new_manager_password);
        let Ok(mut file) = File::create("manager_password.txt") else { return StatusCode::INTERNAL_SERVER_ERROR };
        let Ok(_) = file.write_all(new_manager_password.as_bytes()) else { return StatusCode::INTERNAL_SERVER_ERROR };
    }

    StatusCode::ACCEPTED
}

fn image_response(data: &'static [u8]) -> Result<impl IntoResponse, StatusCode> {
    Response::builder()
        .header("Content-Type", "image/png")
        .body(Body::from(Bytes::from_static(data)))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn logo_image() -> Result<impl IntoResponse, StatusCode> {
    let data = include_bytes!("../src/assets/logo.png");
    image_response(data)
}

async fn arrow_image() -> Result<impl IntoResponse, StatusCode> {
    let data = include_bytes!("../src/assets/arrow.png");
    image_response(data)
}

async fn search_image() -> Result<impl IntoResponse, StatusCode> {
    let data = include_bytes!("../src/assets/search.png");
    image_response(data)
}

async fn plus_image() -> Result<impl IntoResponse, StatusCode> {
    let data = include_bytes!("../src/assets/plus.png");
    image_response(data)
}

async fn white_plus_image() -> Result<impl IntoResponse, StatusCode> {
    let data = include_bytes!("../src/assets/white_plus.png");
    image_response(data)
}

async fn edit_image() -> Result<impl IntoResponse, StatusCode> {
    let data = include_bytes!("../src/assets/edit.png");
    image_response(data)
}

async fn white_edit_image() -> Result<impl IntoResponse, StatusCode> {
    let data = include_bytes!("../src/assets/white_edit.png");
    image_response(data)
}

async fn delete_image() -> Result<impl IntoResponse, StatusCode> {
    let data = include_bytes!("../src/assets/delete.png");
    image_response(data)
}

async fn white_delete_image() -> Result<impl IntoResponse, StatusCode> {
    let data = include_bytes!("../src/assets/white_delete.png");
    image_response(data)
}