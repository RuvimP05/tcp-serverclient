build commands:
'''
cargo build --bin <client/server> --release
'''

Basic TCP server:
multithreaded
key authentication ***not encrypted***
server logs all messages to console
Client sends message, server recieves and sends back empty response (to keep client active)
highly customizable to suit basic needs
