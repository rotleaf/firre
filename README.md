# firre

A thin Firebase HTTP client — Rust crate + Python package (via PyO3/maturin) + CLI.
Wraps the Firebase Identity Toolkit / Secure Token REST APIs for auth, and the
Firestore REST API for document reads/writes. No `firebase_admin`, no gRPC.

```
pip install firre
```

## Auth

Get an `Auth` handle from `auth` getter on a `Firebase` instance — `Auth` has no
public constructor, you can't build it directly.

```python
import firre

frb = firre.Firebase(API_KEY)
auth = frb.auth          # Auth instance, bound to this api key
```

### Sign in with email/password

```python
resp = auth.emailPwdSignIn(email, password)        # -> AuthResponse
resp = auth.emailPwdSignUp(email, password)         # -> AuthResponse, creates the account
```

`AuthResponse` fields: `.idToken`, `.refreshToken`, `.email`, `.userId`,
`.expiresIn`, `.raw`, `.json`, `.getAuthHeader` (dict with `Authorization: Bearer <idToken>`).

### Refresh an existing session

```python
resp = auth.refreshTokenAuth(refresh_token)         # -> RefreshResponse
```

`RefreshResponse` fields: `.idToken`, `.accessToken`, `.refreshToken`, `.userId`,
`.expiresIn`, `.raw`, `.json`, `.getAuthHeader`. `.accessToken` and `.idToken`
carry the same JWT value here — either works as the Firestore bearer token below.

All three methods take an optional trailing `headers: dict[str, str]` to merge
into the outgoing request.

**Don't hardcode the api key or refresh token in source** — a refresh token is a
standing credential with no expiry until revoked. Pull both from env:

```python
import os
frb = firre.Firebase(os.environ["FIREBASE_API_KEY"])
auth = frb.auth.refreshTokenAuth(os.environ["FIREBASE_REFRESH_TOKEN"])
```

## Firestore

```python
store = frb.Firestore(auth.accessToken, "your-project-id")   # -> FirestoreClient
```

### References: `doc()` and `collection()`

```python
doc = store.doc("users/abc123")     # Document reference, no request sent yet
col = store.collection("users")     # Collection reference, no request sent yet
```

Both only build the reference object — nothing is fetched until you call a
method on it. A `Collection` can also chain into a `Document`:

```python
doc = store.collection("users").doc("abc123")   # same reference as store.doc("users/abc123")
```

### Documents

```python
resp = doc.get()            # -> FirestoreResponse
print(resp.json)            # parsed dict
print(resp.raw)             # raw response text

doc.delete()                # deletes this document; returns FirestoreResponse (raw == "{}" on success)
```

Failed calls (any non-2xx) raise `RuntimeError` rather than returning an error
object — see **Error handling** below.

### Writing a field

```python
doc.patch(field_type, field_path, field_value, headers=None)
```

`field_type` must be one of: `stringValue`, `integerValue`, `doubleValue`,
`booleanValue`, `nullValue`, `timestampValue` (RFC 3339, e.g.
`"2026-03-19T15:37:04.272Z"`). `field_value` is always passed as a string and
parsed/validated by `firre` based on `field_type`. `field_path` supports dotted
nesting (`profile.displayName`) and builds the corresponding Firestore
`mapValue` structure automatically.

```python
doc.patch("stringValue", "displayName", "name")
doc.patch("integerValue", "loginCount", "1")
doc.patch("booleanValue", "active", "true")
```

### Field-level ops (timestamp / increment / delete)

```python
field = doc.Field("loginCount")
field.serverTimestamp()       # set field to server request time
field.increment(1.0)          # atomic numeric increment
field.delete()                # remove the field from the document
```

### Collections

```python
col = store.collection("users")

ids = col.getDocumentIds()         # -> list[str], paginated internally, bare doc IDs only
docs = col.getDocuments()          # -> list[Document], paginated internally, full field data

for doc in docs:
    data = doc.json               # parsed dict for this document
    if some_condition(data):
        doc.delete()              # each entry is a full Document reference — get/patch/delete all work

count = col.deleteAll()            # deletes every document in the collection, batched (500/req)
print(f"deleted {count} documents")
```

Notes:
- `getDocumentIds()` uses a `__name__`-only field mask, so it's cheap even on
  large collections — use it when you just need IDs to loop against.
- `getDocuments()` fetches full documents and returns real `Document` objects
  (not plain JSON), so each one is independently actionable — `.get()`,
  `.patch()`, `.delete()` all work directly on entries from the list.
- `deleteAll()` lists then batch-deletes (up to 500 per request) and pages
  through the full collection automatically, returning the total count deleted.
- None of these cascade into subcollections — nested subcollections need their
  own `collection(path).deleteAll()` call.

## Response objects

Every request-issuing method returns an object with `.raw` (response body as
text) and `.json` (same, parsed via Python's `json.loads`). Non-2xx responses
raise a `RuntimeError` with the HTTP status and body rather than returning an
error object — wrap calls in `try/except RuntimeError` if you need to handle
auth failures, permission-denied, etc. gracefully.

```python
try:
    doc = store.doc("users/abc123").get()
except RuntimeError as e:
    print("firestore call failed:", e)
```

## Full example

```python
import os
import firre

frb = firre.Firebase(os.environ["FIREBASE_API_KEY"])
auth = frb.auth.refreshTokenAuth(os.environ["FIREBASE_REFRESH_TOKEN"])

store = frb.Firestore(auth.accessToken, "your-project-id")

# single doc
user = store.doc("users/abc123").get()
print(user.json)

# list a collection
users = store.collection("users").getDocuments()
for u in users:
    print(u.json)

# write
store.doc("users/abc123").patch("integerValue", "loginCount", "42")

# delete a document
store.doc("users/abc123").delete()

# delete an entire collection
deleted = store.collection("users").deleteAll()
print(f"deleted {deleted} documents")
```
