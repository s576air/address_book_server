use serde::{Deserialize, Serialize};

pub fn is_phone_number(s: &str) -> bool {
    let s = s.as_bytes();
    
    if s.len() != 13 { return false }

    for i in (0..3).chain(4..8).chain(9..13) {
        if !matches!(s[i], b'0'..=b'9') {
            return false
        }
    }

    for i in [3, 8] {
        if s[i] != b'-' {
            return false
        }
    }

    true
}

fn is_email(s: &str) -> bool {
    let Some((username, domain)) = s.split_once('@') else { return false };
    
    if username.is_empty() { return false }

    let Some((domain_name, domain_type)) = domain.split_once('.') else { return false };

    if domain_name.is_empty() { return false }
    if domain_type.is_empty() { return false }

    true
}


pub fn html_tel(tel: &str) -> String {
    if is_phone_number(tel) {
        format!(r#"<a href="tel:+82{tel}">{tel}</a>"#)
    } else {
        tel.to_string()
    }
}

pub fn html_email(email: &str) -> String {
    if is_email(email) {
        format!(r#"<a href="mailto:{email}">{email}</a>"#)
    } else {
        email.to_string()
    }
}

pub fn fit_string(old: &str) -> String {
    let mut new = String::new();

    for line in old.lines() {
        let line = line.trim_start();
        new.push_str(line);
    }

    new
}

#[derive(Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub business: String,
    pub phone: String,
    pub email: String,
    pub image_path: String,
}

impl Profile {
    fn to_html(&self, hr: bool) -> String {
        let name = &self.name;
        let business = &self.business;
        let phone = html_tel(&self.phone);
        let email = html_email(&self.email);
        let hr = if hr { r#"<hr class="my-hr">"# } else { "" };
        let image_path = &self.image_path;

format!(r#"
<div class="profile">
    <div class="profile-top">
        <img src="image?{image_path}" alt="사진" class="profile-img">
        <div>
            <div class="main-info">성명&#x3000;&#x3000;   {name}</div>
            <div class="main-info">직장명&#x3000;   {business}</div>
            <div class="main-info">휴대전화   {phone}</div>
            <div class="main-info">이메일&#x3000;   {email}</div>
        </div>
    </div>
    {hr}
</div>
"#)

    }
}

#[derive(Serialize, Deserialize)]
pub struct MainMenuGroup {
    pub name: String,
    pub profiles: Vec<Profile>,
}

impl MainMenuGroup {
    pub fn new(name: String) -> Self {
        MainMenuGroup { name, profiles: Vec::new() }
    }
    fn to_html(&self, n: usize, last_hr: bool) -> String {
        let name = &self.name;
        let mut profiles = String::new();

        if !self.profiles.is_empty() {
            for i in &self.profiles[..self.profiles.len()-1] {
                profiles.push_str(&i.to_html(true));
            }

            let last = self.profiles.last().unwrap();

            profiles.push_str(&last.to_html(last_hr));
        }

format!(r#"
<div class="group">
    <div class="group-name">{name}</div>
    <img class="arrow" id="arrow{n}" src="arrow.png" alt="open close" onclick="toggleGroup('group{n}', 'arrow{n}')">
</div>
<div class="profiles" id="group{n}">
    {profiles}
</div>
"#)

    }
}

#[derive(Serialize, Deserialize)]
pub struct MainMenu(pub Vec<MainMenuGroup>);

impl MainMenu {
    pub fn to_html(&self) -> String {
        let mut groups = String::new();

        if !self.0.is_empty() {
            for i in 0..self.0.len()-1 {
                groups.push_str(&self.0[i].to_html(i, false))
            }

            let last = self.0.last().unwrap();

            groups.push_str(&last.to_html(self.0.len()-1, true));
        }

        let group_number = self.0.len();

let html = format!(r#"
<!DOCTYPE html>
<html lang="kr">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>주소록</title>
    <style>
        * {{
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }}
        body {{
            font-family: Arial, sans-serif;
        }}
        #title-box {{
            width: 100%;
            background-color: rgb(13, 71, 161);
            height: 50px;
            align-items: center;
            display: flex;
        }}
        #logo {{
            justify-content: center;
            align-items: center;
            max-width: 170px;
            margin-left: 10px;
        }}
        #title {{
            margin-left: 30px;
            font-size: 20px;
            color: white;
        }}
        #search-toggle-container {{
            align-items: center;
            display: flex;
            height: 5vh;
            justify-content: space-between;
        }}
        #search-container {{
            display: flex;
            float: left;
        }}
        #search {{
            border: 1px solid rgb(13, 71, 161);
            height: 28px;
            margin-left: 10px;
        }}
        #search-button-container {{
            width: 28px;
            height: 28px;
            background-color: rgb(13, 71, 161);
        }}
        #search-button {{
            margin: 5px;
            width: 18px;
            height: 18px;
        }}
        .button {{
            height: 28px;
            margin-right: 10px;
        }}
        .group {{
            border-bottom: 1px solid white;
            width: 100%;
            height: 5vh;
            background-color: rgb(13, 71, 161);
            align-items: center;
            display: flex;
            justify-content: space-between;
        }}
        .group-name {{
            color: white;
            margin: 10px;
            font-size: 16px;
        }}
        .arrow {{
            margin: 10px;
            width: 19.75px;
            height: 25px;
            transform: rotate(90deg);
        }}
        .hidden {{
            opacity: 0.5;
        }}
        .profiles {{
            display: none;
            flex-direction: column;
        }}
        .profile-top {{
            display: flex;
        }}
        .profile-img {{
            margin: 0px; /*더미 정보*/
            width: 120px;
        }}
        .main-info {{
            font-size: 20px;
            white-space: pre;
        }}
        .my-hr {{
            border-top: 1px solid rgb(13, 71, 161);
        }}
    </style>
    <script>
        function searchProfile(profile, text) {{
            if (text === '') {{ return true; }}
            let mainInfo = profile.children[0].children[1].children;
            let home = profile.children[5];
            if (
                mainInfo[0].textContent.includes(text) ||
                mainInfo[1].textContent.includes(text)
            ) {{
                return true;
            }}
            

            return false
        }}
        function searchAll() {{
            setAllDisplay(true);
            const text = document.getElementById('search').value;
            const profiles = document.getElementsByClassName('profile');

            for (let i = 0; i < profiles.length; i++) {{
                const profile = profiles[i];

                if (searchProfile(profile, text)) {{
                    profile.style.display = 'block'
                }} else {{
                    profile.style.display = 'none'
                }}
            }}
        }}
        document.addEventListener('DOMContentLoaded', function() {{
            document.getElementById('search').addEventListener('input', searchAll);
        }});

        function setAllDisplay(is_visible) {{
            if (is_visible) {{
                var d = 'flex';
                var t = 'rotate(270deg)';
            }} else {{
                var d = 'none';
                var t = 'rotate(90deg)';
            }}
            for (let i = 0; i < {group_number}; i++) {{
                document.getElementById('group' + i).style.display = d;
                document.getElementById('arrow' + i).style.transform = t;
            }}
        }}
        function toggleGroup(groupId, imageId) {{
            const group = document.getElementById(groupId);
            const image = document.getElementById(imageId);
            if (group.style.display === 'flex') {{
                group.style.display = 'none';
                image.style.transform = 'rotate(90deg)';
            }} else {{
                group.style.display = 'flex';
                image.style.transform = 'rotate(270deg)';
            }}
        }}
    </script>
</head>
<body>
    <div id="title-box">
        <img id="logo" src="logo.png" alt="logo">
        <div id="title">주소록</div>
    </div>
    <div id="search-toggle-container">
        <div id="search-container">
            <input type="text" id="search" placeholder=" 검색란">
            <div id="search-button-container"><img src="search.png" alt="search" id="search-button"></div>
        </div>
        <div>
            <button onclick="setAllDisplay(true)" class="button">전체 펼치기</button>
            <button onclick="setAllDisplay(false)" class="button">전체 접기</button>
        </div>
    </div>
    {groups}
</body>

"#);

    fit_string(&html)

    }
}

