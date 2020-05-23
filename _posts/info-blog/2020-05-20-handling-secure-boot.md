---
layout: post
title: Handling Secure Boot
tags: [security, booting, cryptography]
author: Nishant Joshi
---



# Handling Secure Boot

## What is Secure Boot?

Secure boot is a feature provided by many manufacturing companies, this feature is present inside the BIOS of your system. The prime goal of this feature is to check whether the bootloader that is being executed is safe (Key verified) of not.

## Why do self-signing?

Self-signing allows the user the sign manually which bootloaders are allowed to be executed, or to sign some binaries that you have created and you want to execute at boot time. It uses the MOK manager to adds the key as trusted signing agent. 

## How to Self-sign

1. Creating your own key, certificate to add

   ```bash
   $ openssl req -new -x509 -newkey rsa:2048 -keyout MOK.key -out MOK.crt \
   	-nodes -days 3650 -subj "/CN: <Your Name>/"
   $ openssl x509 -in MOK.crt -out MOK.cer
   # <Your Name> : Enter your name
   ```

   Now there would be three files in the current folder `MOK.key, MOK.crt, MOK.cer`

   > Here the **MOK.key** is very important and should be kept safe.

2. Importing the certificate into the MOK manager

   ```bash
   $ sudo mokutil --import MOK.cer
   # Here you would be prompted for a input password.
   # This password has further uses so do not forget it
   ```

Here after, it depends on what you want to sign

---
### Signing Binaries
   This is a low risk easy to perform task, go to the directory where the binaries are stored

   ```bash
   sbsign --key /path/to/MOK.key --cert /path/to/MOK.crt\ --output <ofilename>.efi <filename>.efi
   # Here the ofilename can be same as filename
   # filename is the file you want to sign
   ```

---

### Signing Kernel modules

In alternative to signing binaries some times kernel modules can also cause the secure boot check to fail. Signing the kernel modules is as simple as signing any other binary.

```bash
$ kmodsign sha512 MOK.key MOK.cer module.ko module.ko
# here we are signing the module.ko using the MOK.key
```

---

## Alternatives to self-signing

Now, you must be wondering, are there any other alternatives to self signing, definitly!. But for thing alternatives you lose the freedom that is granted to you by the self-signing technique. The alternatives are 

- shim
- PreLoader

There are other methods but shim it the one used in ubuntu to make it possible to boot into ubuntu even if the secure boot in on. i.e. because the ubuntu bootloader is signed by microsoft which grants to access to boot into the computer.

---



#### Resources

Please Do read this amazing blog post on

: [how-to-sign-things-for-secure-boot](ubuntu.com/blog/how-to-sign-things-for-secure-boot)

---



Done !!