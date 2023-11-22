# Trashik

> _Traefik, but trash!_

## What is this

This is a small reverse proxy I am playing around with.

I wanted to learn Rust, try some TCP networking stuff out,
and maybe even build something I can use to proxy my own Minecraft
servers.

## Learnings so far...

### Routing, and the magic of SNI

The hard part so far seems to be knowing _where_ to send the traffic.
In HTTP you get a bunch of handy information in the request headers
(the `Host` header for example).

There's also another method that is a bit more versatile.
In HTTPS, you can leverage a feature of TLS called SNI
(networking has too many acronyms btw).

SNI means **server name indicator**, and it's a bit of metadata included in TLS
handshakes that lets the server know the hostname you want to go to. This is
useful in the case that a server has multiple services with multiple SSL certs,
and it needs to know which one to reply with.

Since this SNI hostname data is just sitting there, we can read it for our
reverse proxy! This is neat because it gives us an easy way to route not just
HTTPS but also any other TLS enabled connection.

### BUT MINECRAFT!

Pfft, web servers are so yesterday. Minecraft doesn't use TLS though, so it's
back to square one.

Luckily, the closed-source Minecraft protocol has been reverse-engineered to
death and is [so well detailed by the community](https://wiki.vg/Protocol) that it could probably
be accepted as an RFC.

This is what I am working on now (I will return to HTTP later), and so far so
good. We can parse the basics integers and strings needed to decipher the
server address the player wants to connect to.

## TODO:

- [ ] Read the rest of the handshake packet
- [ ] Proxy the connection to the client
- [ ] Maybe look into tokio for async?
- [ ] Try and make a little dashboard
