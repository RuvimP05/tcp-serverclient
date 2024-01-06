<h2>Basic TCP server:</h2>
<ul>
<li>multithreaded</li>
<li>key authentication **not encrypted**</li>
<li>server logs all messages to console</li>
<li>Client sends message, server recieves and sends back empty response (to keep client active)</li>
<li>username functionality</li>
<li>highly customizable to suit basic needs</li>
</ul>

<h3>build commands: (select between client or server)</h3>

```
cargo build --bin <client/server> --release
```

<h3>how to use:</h3>
create a file called key, and type in any key you'd like into it, and distribute it to the server binary directory, and all the clients' binary directories </br>
</br>

```
./client <IP.ADDR:PORT> (server is hardcoded to port 6969)
```
```
./server
```
