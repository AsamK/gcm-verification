# GCM verification for Jodel

This application provides two HTTP endpoints to create an Android account and to receive verification messages from GCM.
The server is written in Rust with async/await, tokio and axum.

Endpoints:
- GET http://127.0.0.1:9090/account <br>
  response: `{"android_account":{"android_id":"XXX","security_token":"XXX"},"gcm_token":"XXX"}`
- POST http://127.0.0.1:9090/verification <br>
  body: `{"android_id":"XXX","security_token":"XXX"}` <br>
  response: `{"verification":{"server_time":0,"type":"silent_verification","verification_code":"XXX"}}`

- POST http://127.0.0.1:9090/email/request <br>
  body: `{"email":"XXX@XXX"}`
- POST http://127.0.0.1:9090/email/confirm <br>
  body: `{"email":"XXX@XXX","link":"XXX"}` <br>
  response: `{"access_token":"XXX","user_id":"XXX"}`

Run server locally:

    cargo run

## License

Copyright: AsamK 2018-2022

Licensed under the GPLv3: http://www.gnu.org/licenses/gpl-3.0.html
