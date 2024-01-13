+++
layout = "post"
title = "Wireguard VPN for linux"
[taxonomies]
    tags = ["linux", "VPN", "blog"]
+++
# Overview of wireguard

## What is wireguard ?

Wireguard is a new VPN client / server addition to the linux kernel for 5.6. This is a small but powerful VPN client / server which can be setup and implemented within no time. It is build over and similar to the concept of SSH and moch.

## Installation

Source: [wireguard.com/install/](https://wireguard.com/install/)

Those with kernel >= 5.6 you already got it. For Windows & Android users checkout their official website : [wireguard.com](wireguard.com)

For linux there are couple of ways to install it

- install using the default package manager
- compile for source (but why would you do that)

## Setting up

There are two ways of setting up the wireguard VPN interface

- Through the configuration file
- Through command line

### Through command line

1. After installation is done, generation of private key for the tunneling is required

   ```bash
   $ wg genkey > private.key
   # This generates the base64 encoded private key
   $ chmod 600 private.key
   # Securing it against any unwanted tampering
   ```

2. Setting up the network interface for the VPN

   ```bash
   $ ip link add wg0 type wireguard
   # It could be anything wg0 - wgn
   ```

3. Setting IP Address for the newly created interface

   ```bash
   $ ip addr add <ip-address>/<sn-mask> dev wg0
   # Here <sn-mask> : subnet mask
   # wg0 is just the example you have to use the one specified in step 2
   ```

4. Configuring the interface

   ```bash
   $ wg set wg0 private-key ./private.key
   # This is the assign the private key to the interface
   ```

5. To start / stop the interface

   ```bash
   # To start the interface
   $ ip link set wg0 up
   # To stop the interface
   $ ip linke set wg0 down
   ```

After this is done the configuration of the interface is done

Now, you might need to connect to someone's private network, to do this we have to configure the peer for the interface, this can be done multiple times with multiple peers

```bash
wg set wg0 peer <peer-pub-key> allowed-ips <ip-addr(with subnet mask)> endpoint <current-ip>:<port>
# <peer-pub-key> : This is the public key of the peer
# This can be generated by the peer after generating the private key
# Once he has his own private.key and interface configured, then
$ wg pubkey < private.key > public.key
# Here your could also provide the private key directly and get the public key
# $ echo <private-key> | wg pubkey

# This content of the file public.key is the public key
# <ip-addr> : This is the last known ip address of the host in the private network with subnetmask
# <current-ip>:<port> : This is the IP address of the peer which is accessable to us, generally the port : 51820 but it could be anything to check what the port of the peer is ask him to :
$ wg
# This output of the command contains the listening port for the peer's interface
# The endpoint is optional
```
---
### Using configuration file

Here we can generate a configuration file and then pass it to the wg command line interface, but there is a much better way, let's see what that is:

1. Go the the /etc/wireguard

2. Now, create a configuration file wg0.conf (*from wg0.conf - wgn.conf*)

3. Generate and copy a private key using the wg command

   ```bash
   $ wg genkey
   ```

4. Let's add content to the configuration file

   ```properties
   [ Interface ]
   PrivateKey = <Private-key> # Generated in step 3
   ListenPort = 51820 # It could be anything really, this is often used
   
   [ Peer ] # This section can be used multiple times depending on the no. of peers
   # This section can be ignored if you just want to setup the interface
   PublicKey = <peer-pub-key> # This is explained in the CLI peer setup step above
   Endpoint = <current-ip>:<port> # This is the ip of the peer the is visible to us (optional)
   AllowedIPs = <ip-addr> # This is the last known IP of the peer in the private net
   ```

5. After saving this you can spin up the interface with a single command.

   ```bash
   $ systemctl start wg-quick@wg0.service
   # The wg0 -> can be replaced by the interface name you have choosen
   ```

---

## Thank you

source : [wireguard.com](wireguard.com)