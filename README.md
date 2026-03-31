# firre

A Firebase client library for Python and Rust, with a CLI binary included.

Built with Rust and PyO3 — fast, typed, and works as both a pip package and a Rust crate.

## Installation

```bash
pip install firre
Python Usage
import firre

fire = firre.Firebase("YOUR_API_KEY")

# Refresh token
r = fire.auth.refreshTokenAuth("your-refresh-token")
print(r.idToken)
print(r.userId)
print(r.accessToken)

# Sign in
r = fire.auth.signIn("user@example.com", "password")
print(r.idToken)
print(r.userId)
print(r.refreshToken)

# Sign up
r = fire.auth.signUp("newuser@example.com", "password")
print(r.idToken)
print(r.userId)
Response Fields
signIn / signUp returns AuthResponse:
| Field | Description |
|---|---|
| idToken | Firebase ID token |
| refreshToken | Token to refresh the session |
| email | User email |
| userId | Firebase user UID |
| expiresIn | Seconds until token expires |
| raw | Full JSON response string |
refreshTokenAuth returns RefreshResponse:
| Field | Description |
|---|---|
| idToken | Fresh Firebase ID token |
| accessToken | OAuth2 access token |
| refreshToken | New refresh token |
| userId | Firebase user UID |
| expiresIn | Seconds until token expires |
| raw | Full JSON response string |
Error Handling
All methods raise RuntimeError with the Firebase error message on failure:
try:
    r = fire.auth.signIn("user@example.com", "wrongpassword")
except RuntimeError as e:
    print(e)  # INVALID_PASSWORD
Common errors: INVALID_PASSWORD, EMAIL_NOT_FOUND, EMAIL_EXISTS, USER_DISABLED, TOKEN_EXPIRED, WEAK_PASSWORD
CLI Usage
Download the binary for your platform from Releases.
# Sign in
firre -k YOUR_API_KEY auth sign-in -e user@example.com -p password

# Sign up
firre -k YOUR_API_KEY auth sign-up -e user@example.com -p password

# Refresh token
firre -k YOUR_API_KEY auth refresh-token -t your-refresh-token
Output is raw JSON from the Firebase API.
Rust Usage
[dependencies]
firre = "0.1.0"
use firre::firebase::auth::requests::{core_sign_in, core_refresh_token};

fn main() {
    let result = core_sign_in("API_KEY", "user@example.com", "password");
    match result {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("Error: {}", e),
    }
}
Android / Termux
Pre-built wheels for Android (aarch64) are available on the Releases page.
pip install https://github.com/rotleaf/firre/releases/latest/download/firre-latest-cp310-abi3-linux_aarch64.whl
License
MIT
