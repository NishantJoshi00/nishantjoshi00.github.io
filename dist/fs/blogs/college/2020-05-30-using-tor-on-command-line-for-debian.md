## Introduction

### What is Tor ??

Tor is a network that is used to provide higher security that any existing VPN by enveloping your data into multiple security layers which renders the source untrackable.

> Unless you share your location to the website you wish to visit or Google

### Why from Command Line ??

Basically because it is fun, but also most of the work you would do, ends up done on the terminal in linux, like i have a habit of opening firefox using

`$ firefox &>/dev/null &` instead of clicking on the icon. But other than that when your are trying to access a API locally and would like to prevent the API from knowing your location. Even trying to learn pentesting it is usually done on CLI and its better to be safe from how you are going to expose your system to the internet in such cases.

### Why Debian ??

Firstly, due to experience I know that it would definitely work on Debian

Secondly, I am also sure that it would work on any other linux distribution, the only changes would be to use the proper package manager. But i have not tried it on any other operating system and thus cannot validate it.

## Installation

- For Debian users `apt install tor` will do the trick.

- It you are more interested in getting your hands dirty. Follow the process given below to install tor manually

  First download the source code for tor from their official website, or [click here](https://2019.www.torproject.org/download/download.html.en)

  Then open a terminal in the directory where it is placed and start getting your hands dirty

  ```bash
  # For this to work you need tar install which is usually present
  tar -xzf tor-0.X.Y.Z.W.tar.gz
  cd tor-0.X.Y.Z.W
  # Here your would require some extra libries to make the configuration of the source successful, that are openssl and zlib or just run:
  sudo apt-get install libssl-dev
  sudo apt-get install zlib1g
  # Then comes the fun part
  # For this step you need gcc installed
  # -------------------
  sudo apt-get install gcc
  # -------------------

  ./configure
  # This looks a lot like cool hacking stuff we see on movies
  # Then just run make
  # -----------------
  sudo apt-get install make
  # -----------------

  sudo make && sudo make install

  ```

## Application

To make your terminal torrifed you can use either of two things **torsocks**, **socat**

Here I am going to show how to use torsocks cause it is simple to setup and use

- Here you have two choices your could install it directly using `apt install torsocks`

- Or get you hands dirty again:

  ```bash
  git clone https://git.torproject.org/torsocks.git
  cd torsocks
  ./autogen.sh
  ./configure
  make && make install
  ```

  Here I am not explaining the IN's / OUT's because well you are smart and you will figure it out.

## Getting started using TOR

- First start the tor service

- Now then just run : `source torsocks on`

  and you are good to go.

- For turning it off run : `source torsocks off`
