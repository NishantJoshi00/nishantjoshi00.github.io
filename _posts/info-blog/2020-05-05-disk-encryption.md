---
layout: post
title: Disk encryption using LUKS
tags: [cryptography, linux, filesystem]
---

# Disk Encryption

Given below is a method of encrypting your hard-drive partition (SSD/HDD) using luks encryption method, But this file system can only be recognised in a Linux Box and thus can only be decrypted in Linux. As the partition format is unrecognized in windows filesystem.

## Formatting the Partition

> Here we are formatting the partition in **LUKS** (**L**inux **U**nified **K**ey **S**etup)

Here we use `luksFormat` to create a encryption layer. On the partition. By doing so the partition will be completed formated so make sure it is a empty partition.

```bash
$ cryptsetup luksFormat /dev/sdxY
```

Here x is a alphabet, and Y is a number, which reperesent the block file for the Yth partition of disk x.

---

## Initialise LUKS devices

Here we use the `luksOpen` which temporarily disables the encryption layer.

This devices will then be available in /dev/mapper/<name> 

here <name> is a arbitrary value used to identify the block file.

```bash
$ cryptsetup luksOpen /dev/sdxY <name>
```

---

## Format this decrypted device

As **luks** acts as an encrypted wrapper over the underlying partition. we can use any `mkfs` cmd to format it. We can then normally mount it. This is done only after unlocking the encryption layer.

---

## Mounting the Device

> Sometimes connecting the device with a encrypted partition, automatically provokes a initialising and mounting procedure and prompts you to enter your password, in some Linux operating systems.

The device can be mounted simply using the mount command

```bash
$ mount /dev/mapper/<name> <mountpoint>
```



---

## Re-Encrypt the partition

Here the drive must be unmounted first, then perform a re-encryption using:

```bash
$ cryptsetup luksClose /dev/mapper/<name>
```



## Changing the Password of the partition

This requires you to know the old password given to the partition.

```bash
$ cryptsetup luksChangeKey /dev/sdxY
```

