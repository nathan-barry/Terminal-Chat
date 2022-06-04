# NIRC

Nathan's Internet Relay Chat (NIRC) is Nathan's implementation of an encrypted terminal-based chat application that allows anyone to host their own server and allow others to join. I have zero clue on how IRC actually works, so my implementation will most likely be nothing like how IRC actually functions.

## How To Start

Current functionallity only includes local host (127.0.0.1:6000)

To host the server, type in the command:

`cargo run host <NAME>`

To join a server, type in:

`cargo run connect <NAME>`

---

## Roadmap:
- [ ] Encryption
- [ ] Make beautiful
- [ ] Figure out hosting (not on Local)
- [x] Refractored code (instead of just one spaghetti code file)
- [x] Usernames
- [x] Allow host to simultaniously be a client
- [x] Get messages across multiple clients working
- [x] CLI functional
