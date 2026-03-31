# firre

A small Firebase Authentication client for **Python** and **Rust**, with a bundled **CLI**.

Built in Rust with **PyO3**, `firre` gives you:

- a Python package
- a Rust crate
- a command-line interface for common auth flows

Supported flows:

- email/password sign-in
- email/password sign-up
- refresh-token exchange

## Installation

### Python

```bash
pip install firre
```

`firre` is published as a Python package built with `maturin`, with Python support declared as `>=3.8`. 1

### Rust

```toml
[dependencies]
firre = "0.1.20"
```

## Python usage

```python
import firre

fire = firre.Firebase("YOUR_API_KEY")
```

### Sign in

```python
response = fire.auth.emailPwdSignIn("user@example.com", "password")

print(response.idToken)
print(response.refreshToken)
print(response.email)
print(response.userId)
print(response.expiresIn)
```

### Sign up

```python
response = fire.auth.emailPwdSignUp("newuser@example.com", "password")

print(response.idToken)
print(response.refreshToken)
print(response.email)
print(response.userId)
print(response.expiresIn)
```

### Refresh a token

```python
response = fire.auth.refreshTokenAuth("your-refresh-token")

print(response.idToken)
print(response.accessToken)
print(response.refreshToken)
print(response.userId)
print(response.expiresIn)
```

### Optional headers

All Python auth methods accept an optional `headers` argument:

```python
response = fire.auth.emailPwdSignIn(
    "user@example.com",
    "password",
    headers={
        "X-Custom-Header": "value"
    }
)
```

## Python API

### `firre.Firebase(api_key)`

Creates a Firebase client bound to your Firebase Web API key. The Python `Firebase` class exposes an `auth` property that returns an `Auth` object. 2

### `fire.auth.emailPwdSignIn(email, password, headers=None)`

Signs in a user with email and password.

Returns an `AuthResponse` with:

| Field | Description |
|---|---|
| `idToken` | Firebase ID token |
| `refreshToken` | Token used to refresh the session |
| `email` | User email |
| `userId` | Firebase user UID |
| `expiresIn` | Seconds until token expiry |
| `raw` | Raw JSON response |

### `fire.auth.emailPwdSignUp(email, password, headers=None)`

Creates a new user with email and password.

Returns an `AuthResponse` with the same fields as above. 3

### `fire.auth.refreshTokenAuth(refresh_token, headers=None)`

Exchanges a refresh token for a new session.

Returns a `RefreshResponse` with:

| Field | Description |
|---|---|
| `idToken` | Fresh Firebase ID token |
| `accessToken` | OAuth access token |
| `refreshToken` | New refresh token |
| `userId` | Firebase user UID |
| `expiresIn` | Seconds until token expiry |
| `raw` | Raw JSON response |

## Error handling

On failure, Python methods raise `RuntimeError` with the Firebase or HTTP error message.

```python
try:
    fire.auth.emailPwdSignIn("user@example.com", "wrongpassword")
except RuntimeError as err:
    print(err)
```

Typical errors include:

- `INVALID_PASSWORD`
- `EMAIL_NOT_FOUND`
- `EMAIL_EXISTS`
- `USER_DISABLED`
- `TOKEN_EXPIRED`
- `WEAK_PASSWORD`

The Rust request layer also returns descriptive errors for request, parse, and HTTP failures. 4

## CLI usage

Download the binary for your platform from the repository releases. The repo currently lists release `v0.1.20` as the latest. 5

### Sign in

```bash
firre -k YOUR_API_KEY auth sign-in -e user@example.com -p password
```

### Sign up

```bash
firre -k YOUR_API_KEY auth sign-up -e user@example.com -p password
```

### Refresh token

```bash
firre -k YOUR_API_KEY auth refresh-token -t your-refresh-token
```

CLI output is raw JSON from the Firebase API.

## Rust usage

The crate includes request helpers for the same auth flows. The current source exposes `core_email_pwd_sign_in`, `core_email_pwd_sign_up`, and `core_refresh_token`. 6

```rust
use firre::firebase::auth::requests::core_email_pwd_sign_in;

fn main() {
    let result = core_email_pwd_sign_in(
        "API_KEY",
        "user@example.com",
        "password",
        None,
    );

    match result {
        Ok(json) => println!("{json}"),
        Err(err) => eprintln!("Error: {err}"),
    }
}
```

## Android / Termux

Prebuilt Android `aarch64` wheels are referenced in the project README via GitHub Releases. 7

## Tech stack

- Rust
- PyO3
- reqwest
- maturin
- clap

These are declared in the repository source and packaging files. 8

## License

MIT
