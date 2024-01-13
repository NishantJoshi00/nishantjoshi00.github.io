+++
layout = "post"
title = "Building CLIs with C"
draft = true
[taxonomies]
    tags = ["linux", "C", "filesystem", "CLI"]
+++

## Introduction

CLI (**C**ommand **L**ine **I**nterface) Application are the programs that we run on our terminal or console. All the linux command we use are CLIs e.g. `ls, cat, vim` . The programs which uses flags and values to communicate with the inherent program.

e.g.

```bash
$ ls -l
# This command displays all the files in the current directory in list format (-l : for list format)
```

## Why C ?

Now, why would you use C. For building such applications.

1. C is **fast** as compared to other programming languages
2. The source code is **obfuscated**.
3. C is better **compatible** with the **kernel** and runs seamlessly.

## Implementing CLI with C

### Taking Command Line Arguments

In C the command line arguments are taken in the main function as arguments. You might have seen the syntax used somewhere before and are wondering, why it is used

```c
int main(int argc, char* argv[]) {}
// This could also be void main(int argc, char* argv[]) {}
```

- `argc`: This is the count of arguments provided to the program.
- `argv`: This is where the arguments are stored in form of character array

> The minimum value of argc is always 1. Where the first argument is the actual name of the program that is been run. Thus the real arguments and flags start from index 1 for `argv`

eg.

```bash
cat << EOF > example.c
#include <stdio.h>
void main(int argc, char* argv[]) {
	printf("argc: %d, argv[0]: %s\n", argc, argv[0]);
}
EOF
gcc example.c -o example
./example
# Output:
# argc: 1, argv[0]: ./example
```

Now, you could directly use this knowledge to biuld your command line application, but managing this flags and the value that are passed to the flags becomes difficult after a certain point, for this case C has functions that can be used.

### Using `getopt()`

`getopt()` is a function from `unistd.h`. This function is used to parse single letter arguments which are followed by hyphen `-`.

eg.

```bash
$ ls -la
	  ^^
	  |\-- # Here `a` is a flag
	  \--- # Here `l` is a flag
```

#### How to implement it

We will take a program for example and then understand how it works

```c
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

int main(int argc, char* argv[]) {
    extern char *optarg;
    extern int optind;
    int out, err = 0;
    int lflag = 0, fflag = 0;
    char *fname;
    static char usage[] = "usage: %s [-l] -f fname name\n";

    while ((out = getopt(argc, argv, "lf:")) != -1) {
        switch (out) {
            case 'l':
                lflag = 1;
                break;
            case 'f':
                fflag = 1;
                fname = optarg;
                break;
            case '?':
                err = 1;
                break;
        }
    }
    if (fflag == 0) {
        fprintf(stderr, "%s: missing -f option\n", argv[0]);
        exit(1);
    } else if ((optind + 1) != argc) {
        if (optind == argc) fprintf(stderr, "%s: missing name\n", argv[0]);
        else fprintf(stderr, "%s: more that one name provided\n", argv[0]);
        fprintf(stderr, usage, argv[0]);
        exit(1);
    } else if (err) {
        fprintf(stderr, usage, argv[0]);
        exit(1);
    }
    if (lflag) printf("-l was used\n");
    printf("%s was provided for -f\n", fname);
    printf("name : %s\n", argv[optind]);
    return 0;
}
```

Above is is a simple example which has a usage line similar to:

`usage: ./example [-l] -f fname name`

Where `-l` is an optional flag, where as -f is compulsary.

Now, lets understand the code:

#### Undestanding the code

- The includes are done for the following reasons:

  - `stdio.h`: I/O functionality like printf, fprintf, etc
  - `stdlib.h`: standard C library for exit function, exit status codes, etc
  - `unistd.h`: This has the implementation of `getopt()`

- The main function has the command line arguments passed to it, as explained in the previous title.

- Variables defined are as follows:

  - `extern char *optarg`: This is for storing the values that are passed to a flag. Here for `-f` flag. Where this flag is recieved the variable points to the string that was passed to it.

  - `extern int optind`: This is the index of the argument that is being parsed.

  - `int out, err =0`: The out is for storing the current argument that is being parsed of -1 if EOArgument is reached. err is to check if any invaid flag is passed. it is used in the end of the switch statement.

  - `int lflag = 0, fflag = 0`: This variables are for knowning if the flags were triggered. Here, in my case `-l or -f` are triggered or not. `char *fname` similarly this is for storing the argument passed to the `-f` flag.
  - The `usage` string is just for providing with a useful help if they want to know which flags are to be used and how, if any special cases. There is a syntax to write this as well, the flags present in the `[]` (squarebrackets) are representing the optional flags and the others show the compulsary flags or arguments

- In the while loop, is where the magic happens

  ```c
  out = getopt(argc, argv, "lf:")
  ```

  ​ Here the arguments that are passed to the main function are directly passed to the getopt function, with an extra argument. This extra argument is the schema for parsing the flags.

  ​ Here, `"lf:"` means look for `-l` flag and look for `-f` flag with an argument `:`.

  ​ eg. `abe:w:` here `-a and -b` are the flags (toggles) and `-e` and `-w` would expect an argument.

  There is no special way to tell if a argument is permanent of optional this can be checked after the while loop manually, well this is C after all :sweat_smile:.

  ​ Every time the variable `out` will hold the flag. and if it equals `-1` then there are no further flags to parse.

- The switch case, this is where you parse the flags.

  ​ As stated in the top `out` holds the flag name. Thus we are checking if the flag that is present is one of the desired one, if so the we say that the flag is triggered by setting the variable corresponding to the flag to `1` i.e. `true` _(Ain't no boolean in C)_.

  - `optarg`: The variable stores the string (`char *`) that is passed to the flags with `:` in front of them, in the schema in the point above.

  - `case '?':` Now, we haven't added `?` to the schema, then why are we looking for it.

    ​ This cases tests if any flag other than those specified in the schema are passed. That's why we are setting `err = 1`. We will use this variable to raise an exception in the future.

- Now, we do the error handling

  - We first look if the `-f` was triggered or not. if not triggered then we print the error message to the STDERR and then exit with a status code of **1**.

  - The second condition is used to check if the all the arguments are parsed.

    we are comparing the `optind` and `argc`.

    `optind` points to the current argument that is being parsed. If it points to the unnamed variable if any left or it's value would be equal to that of `argc`. This would mean that all the variables are parsed and no unnamed variable are left.

  - We also look for the `err` variable which is triggered if any unnamed flag is invoked.

- We are finally going for the implementation.

  At the end you can use the flags that are invoked how ever you want. Here i am just printing if the flag is invoked. But you can get creative.

## Future of CLI

We were talking about C, but there are other languages with implement the command line interface much better that `getopt`. There are also other command line implementation in C, if you dig through the [internet](https://github.com/). But this is the default methods that are provided but the standard library (`unstd.h`).
