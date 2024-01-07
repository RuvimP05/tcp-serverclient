<h2>Basic TCP server:</h2>
<ul>
<li>multithreaded</li>
<li>key authentication</li>
<li>TLS encryption</li>
<li>server logs all messages to console</li>
<li>Client sends message, server recieves and sends back message as response</li>
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
</br>
Certificates are included (VALID UNTIL 1/7/2025. CN is RuvimP05, Passkey is 1234567890), but if you want to make your own, here are the commands:</br>
</br>

```
openssl req -x509 -newkey rsa:2048 -keyout private_key.pem -out cert.pem -days 365
```
```
openssl pkcs12 -export -out certificate.p12 -inkey private_key.pem -in cert.pem
```
