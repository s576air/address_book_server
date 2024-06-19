use crate::html::main_menu::fit_string;

use super::main_menu::{MainMenu, MainMenuGroup, Profile};

impl Profile {
    fn to_manager_html(&self, group_order: usize, profile_order: usize) -> String {
        let group_order = group_order.to_string();
        let profile_order = profile_order.to_string();
        let name = &self.name;
        let business = &self.business;
        let phone = &self.phone;
        let email = &self.email;
        let image_path = &self.image_path;

format!(r#"
<div class="profile" id="profile{group_order}-{profile_order}" data-group="{group_order}" data-profile="{profile_order}">
    <div class="profile-add">
        <div style="display: flex;">
            <div class="dropzone" data-group="{group_order}" data-profile="{profile_order}">
                <img src="/image?{image_path}" alt="사진" class="profile-img">
            </div>
            <div>
                <div><input type="text" class="plus-input" placeholder=" 성명" value="{name}"></div>
                <div><input type="text" class="plus-input" placeholder=" 직장명" value="{business}"></div>
                <div><input type="text" class="plus-input" placeholder=" 휴대전화" value="{phone}"></div>
                <div><input type="text" class="plus-input" placeholder=" 이메일" value="{email}"></div>
            </div>
        </div>
        <div style="margin-left: 30px;">
            <img src="edit.png" alt="사진" class="icon" onclick="editProfile({group_order}, {profile_order})">
            <img src="delete.png" alt="사진" class="icon" onclick="deleteProfile({group_order}, {profile_order})">
        </div>
    </div>
    <hr class="my-hr">
</div>
"#)
    }
}

impl MainMenuGroup {
    fn to_manager_html(&self, group_number: usize, profile_number: &mut usize, last_hr: bool) -> String {
        let name = &self.name;
        let mut profiles = String::new();

        if !self.profiles.is_empty() {
            for i in 0..self.profiles.len() {
                profiles.push_str(&self.profiles[i].to_manager_html(group_number, *profile_number));
                *profile_number += 1;
            }
        }

        let hr = if last_hr { r#"<hr class="my-hr">"# } else { "" };

format!(r#"
<div class="group">
    <input type="text" class="plus-input" placeholder=" 그룹명" value="{name}" id="group-name{group_number}">
    <div>
        <img class="icon" src="white_edit.png" alt="edit" onclick="editGroup({group_number})">
        <img class="icon" src="white_delete.png" alt="delete" onclick="deleteGroup({group_number})">
        <img class="arrow" id="arrow{group_number}" src="arrow.png" alt="open close" onclick="toggleGroup('group{group_number}', 'arrow{group_number}')">
    </div>
</div>
<div class="profiles" id="group{group_number}">
    {profiles}
    <div id="profile-add{group_number}" data-group="{group_number}" data-profile="-">
        <div class="profile-add">
            <div style="display: flex;">
                <img src="face.png" alt="사진" class="profile-img">
                <div>
                    <div><input type="text" class="plus-input" placeholder=" 성명"></div>
                    <div><input type="text" class="plus-input" placeholder=" 직장명"></div>
                    <div><input type="text" class="plus-input" placeholder=" 휴대전화"></div>
                    <div><input type="text" class="plus-input" placeholder=" 거주지"></div>
                </div>
            </div>
            <img src="plus.png" alt="plus" class="plus" onclick="addProfile({group_number})">
        </div>
    </div>
    {hr}
</div>
"#)

    }
}


impl MainMenu {
    pub fn to_manager_html(&self, password: &str) -> String {
        let mut groups = String::new();
        let mut profile_number = 0;

        if !self.0.is_empty() {
            for i in 0..self.0.len()-1 {
                groups.push_str(&self.0[i].to_manager_html(i, &mut profile_number, false));
                profile_number = 0;
            }

            let last = self.0.last().unwrap();

            groups.push_str(&last.to_manager_html(self.0.len()-1, &mut profile_number, true));
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
            width: 120px;
        }}
        .main-info {{
            font-size: 20px;
            white-space: pre;
        }}
        .my-hr {{
            border-top: 1px solid rgb(13, 71, 161);
        }}
        .profile-add {{
            display: flex;
            justify-content: space-between;
        }}
        .icon {{
            margin: 10px;
            width: 23px;
            height: 23px;
        }}
        .plus-input {{
            margin-left: 5px;
            height: 28px;
        }}
        .plus {{
            margin: 10px;
            width: 23px;
            height: 23px;
            image-rendering: pixelated;
        }}
        .dropzone {{
            border: 2px dashed #cccccc;
            transition: border-color 0.3s ease;
        }}
        .highlight {{
            border-color: #666666;
        }}
    </style>
    <script>
        function searchProfile(profile, text) {{
            let mainInfo = profile.children[0].children[1].children;
            if (
                mainInfo[0].children[0].value.includes(text) ||
                mainInfo[1].children[0].value.includes(text)
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
        function send(body) {{
            const form = document.getElementById("postform");
            form.children[0].value = body;
            form.submit();
        }}
        function editGroup(n) {{
            const editedGroupName = document.getElementById('group-name' + n).value;
            const body = '{password}#$#editGroup#$#' + n + '#$#' + editedGroupName;
            send(body);
        }}
        function deleteGroup(n) {{
            const groupName = document.getElementById('group-name' + n).value;
            const body = '{password}#$#deleteGroup#$#' + n + '#$#' + groupName;
            send(body);
        }}
        function addGroup() {{
            const newGroupName = document.getElementById('plus-input0').value;
            const body = '{password}#$#addGroup#$#' + newGroupName;
            send(body);
        }}
        function getProfileInfo(profile_name) {{
            const profile = document.getElementById(profile_name);
            const groupOrder = profile.dataset.group;
            const profileOrder = profile.dataset.profile;
            const mainInfo = profile.children[0].children[0].children[1].children;
            const name = mainInfo[0].children[0].value;
            const business = mainInfo[1].children[0].value;
            const phone = mainInfo[2].children[0].value;
            const email = mainInfo[3].children[0].value;
            const body = '#$#' + groupOrder +
                '#$#' + profileOrder +
                '#$#' + name +
                '#$#' + business +
                '#$#' + phone +
                '#$#' + email;

            return body;
        }}
        function editProfile(gn, pn) {{
            let body = '{password}#$#editProfile' + getProfileInfo('profile' + gn + '-' + pn);
            send(body);
        }}
        function deleteProfile(gn, pn) {{
            let body = '{password}#$#deleteProfile' + getProfileInfo('profile' + gn + '-' + pn);
            send(body);
        }}
        function addProfile(n) {{
            let body = '{password}#$#addProfile' + getProfileInfo('profile-add' + n);
            send(body);
        }}
        document.addEventListener('DOMContentLoaded', function() {{
            const dropzones = document.querySelectorAll('.dropzone');

            dropzones.forEach(dropzone => {{
                dropzone.addEventListener('dragover', (e) => {{
                    e.preventDefault();
                    dropzone.classList.add('highlight');
                }});

                dropzone.addEventListener('dragleave', () => {{
                    dropzone.classList.remove('highlight');
                }});

                dropzone.addEventListener('drop', (e) => {{
                    const group_number = e.currentTarget.dataset.group;
                    const profile_number = e.currentTarget.dataset.profile;
                    e.preventDefault();
                    dropzone.classList.remove('highlight');

                    const files = e.dataTransfer.files;
                    if (files.length > 0) {{
                        handleFileUpload(files[0], group_number, profile_number);
                    }}
                }});
            }});
        }});

        function handleFileUpload(file, group_number, profile_number) {{
            fetch('/upload', {{
                method: 'POST',
                body: file,
                headers: {{
                    'password': '{password}',
                    'group': group_number,
                    'profile': profile_number,
                }}
            }})
            .catch((error) => {{
                console.error('Error:', error);
                alert('File upload failed');
            }});
        }}
    </script>
</head>
<body>
    <div id="title-box">
        <img id="logo" src="logo.png" alt="logo">
        <div id="title">주소록(관리자)</div>
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

    <div class="group">
        <input type="text" class="plus-input" placeholder=" 추가할 그룹명" id="plus-input0">
        <img class="plus" src="white_plus.png" alt="plus" onclick="addGroup()">
    </div>

    <form id="postform" action="" method="post" style="display:none;">
        <input type="text" name="data" value="example">
    </form>
</body>
</html>
"#);

    fit_string(&html)
    }
}

