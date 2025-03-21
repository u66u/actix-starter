// Add these imports at the top
use actix_web::{get, HttpResponse};

// Add these services to your App in the main function (around line 63)
// Add these handler functions at the end of the file
#[get("/login-page")]
pub async fn login_page() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(r#"
        <html>
            <head>
                <title>Login</title>
                <style>
                    body { font-family: Arial, sans-serif; max-width: 500px; margin: 0 auto; padding: 20px; }
                    .form-group { margin-bottom: 15px; }
                    label { display: block; margin-bottom: 5px; }
                    input { width: 100%; padding: 8px; box-sizing: border-box; }
                    button { padding: 10px 15px; background-color: #4CAF50; color: white; border: none; cursor: pointer; }
                    .error { color: red; display: none; margin-top: 10px; }
                </style>
            </head>
            <body>
                <h1>Login</h1>
                <div id="error-message" class="error"></div>
                <div class="form-group">
                    <label for="email">Email:</label>
                    <input type="email" id="email" name="email" required>
                </div>
                <div class="form-group">
                    <label for="password">Password:</label>
                    <input type="password" id="password" name="password" required>
                </div>
                <button onclick="login()">Login</button>
                <p>Don't have an account? <a href="/signup-page">Sign up</a></p>

                <script>
                    async function login() {
                        const email = document.getElementById('email').value;
                        const password = document.getElementById('password').value;
                        const errorMsg = document.getElementById('error-message');
                        
                        try {
                            const response = await fetch('/login', {
                                method: 'POST',
                                headers: {
                                    'Content-Type': 'application/json'
                                },
                                body: JSON.stringify({ email, password })
                            });
                            
                            if (response.ok) {
                                window.location.href = '/profile-page';
                            } else {
                                const data = await response.json();
                                errorMsg.textContent = data || 'Login failed';
                                errorMsg.style.display = 'block';
                            }
                        } catch (error) {
                            errorMsg.textContent = 'An error occurred. Please try again.';
                            errorMsg.style.display = 'block';
                        }
                    }
                </script>
            </body>
        </html>
    "#)
}

#[get("/signup-page")]
pub async fn signup_page() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(r#"
        <html>
            <head>
                <title>Sign Up</title>
                <style>
                    body { font-family: Arial, sans-serif; max-width: 500px; margin: 0 auto; padding: 20px; }
                    .form-group { margin-bottom: 15px; }
                    label { display: block; margin-bottom: 5px; }
                    input { width: 100%; padding: 8px; box-sizing: border-box; }
                    button { padding: 10px 15px; background-color: #4CAF50; color: white; border: none; cursor: pointer; }
                    .error { color: red; display: none; margin-top: 10px; }
                </style>
            </head>
            <body>
                <h1>Sign Up</h1>
                <div id="error-message" class="error"></div>
                <div class="form-group">
                    <label for="email">Email:</label>
                    <input type="email" id="email" name="email" required>
                </div>
                <div class="form-group">
                    <label for="password">Password:</label>
                    <input type="password" id="password" name="password" required>
                </div>
                <button onclick="signup()">Sign Up</button>
                <p>Already have an account? <a href="/login-page">Login</a></p>

                <script>
                    async function signup() {
                        const email = document.getElementById('email').value;
                        const password = document.getElementById('password').value;
                        const errorMsg = document.getElementById('error-message');
                        
                        try {
                            const response = await fetch('/register', {
                                method: 'POST',
                                headers: {
                                    'Content-Type': 'application/json'
                                },
                                body: JSON.stringify({ email, password })
                            });
                            
                            if (response.ok) {
                                window.location.href = '/login-page';
                            } else {
                                const data = await response.json();
                                errorMsg.textContent = data || 'Registration failed';
                                errorMsg.style.display = 'block';
                            }
                        } catch (error) {
                            errorMsg.textContent = 'An error occurred. Please try again.';
                            errorMsg.style.display = 'block';
                        }
                    }
                </script>
            </body>
        </html>
    "#)
}

#[get("/profile-page")]
pub async fn profile_page() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(r#"
        <html>
            <head>
                <title>Profile</title>
                <style>
                    body { font-family: Arial, sans-serif; max-width: 500px; margin: 0 auto; padding: 20px; }
                    .profile { background-color: #f5f5f5; padding: 20px; border-radius: 5px; }
                    .profile-info { margin-top: 20px; }
                    button { padding: 10px 15px; background-color: #f44336; color: white; border: none; cursor: pointer; margin-top: 20px; }
                </style>
            </head>
            <body>
                <h1>Your Profile</h1>
                <div class="profile">
                    <div id="loading">Loading your profile...</div>
                    <div id="profile-info" class="profile-info" style="display: none;">
                        <p><strong>User ID:</strong> <span id="user-id"></span></p>
                        <p><strong>Email:</strong> <span id="user-email"></span></p>
                    </div>
                    <div id="error" style="color: red; display: none;">
                        You are not logged in. <a href="/login-page">Login here</a>
                    </div>
                    <button id="logout-btn" onclick="logout()" style="display: none;">Logout</button>
                </div>

                <script>
                    document.addEventListener('DOMContentLoaded', async () => {
                        try {
                            const response = await fetch('/me');
                            
                            if (response.ok) {
                                const userData = await response.json();
                                document.getElementById('user-id').textContent = userData.id;
                                document.getElementById('user-email').textContent = userData.email;
                                document.getElementById('loading').style.display = 'none';
                                document.getElementById('profile-info').style.display = 'block';
                                document.getElementById('logout-btn').style.display = 'block';
                            } else {
                                document.getElementById('loading').style.display = 'none';
                                document.getElementById('error').style.display = 'block';
                            }
                        } catch (error) {
                            document.getElementById('loading').style.display = 'none';
                            document.getElementById('error').style.display = 'block';
                        }
                    });

                    async function logout() {
                        try {
                            await fetch('/logout', { method: 'POST' });
                            window.location.href = '/login-page';
                        } catch (error) {
                            alert('Logout failed. Please try again.');
                        }
                    }
                </script>
            </body>
        </html>
    "#)
}
