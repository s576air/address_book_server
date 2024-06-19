pub fn get_init_menu_html() -> String {

format!(r#"
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
            background-color: #f0f0f0;
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
        #container {{
            justify-content: center;
            align-items: center;
        }}
        #passwordForm {{
            display: flex;
            align-items: center;
            justify-content: center;
            height: 90vh;
        }}
        .passwordLabel {{
            font-size: 22px;
        }}
        #passwordInput {{
            margin: 10px;
            height: 30px;
        }}
        #loginButton {{
            font-size: 20px;
            padding-left: 5px;
            padding-right: 5px;
        }}
    </style>
</head>
<body>
    <div id="title-box">
        <img id="logo" src="logo.png" alt="logo">
        <div id="title">주소록</div>
    </div>
    <form id="passwordForm" action="" method="post">
        <label class="passwordLabel" for="passwordInput">비밀번호</label>
        <input type="password" id="passwordInput" name="data" placeholder="">
        <button id="loginButton" type="submit">로그인</button>
    </form>
</body>
"#)
}