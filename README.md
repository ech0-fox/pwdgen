# pwdgen v1.0
A simple command line tool that generates super-secure passwords, similar to those crated by the algorithms in Chrome and Firefox.

## to build
pwdgen is written in Rust. To build and run it properly, make sure Rust (v1.46 or newer) is installed.

First, clone this repository. Then, assuming Rust is installed, navigate to the newly created pwdgen directory and run the following command:

    cargo build --release

This will build and run a compiled executable of pwdgen, located in the project folder under <u>/target/debug</u>.

## dependencies
pwdgen was built using these crates (available through <u>**crates.io**</u>):

**rand** (v.0.8.5) - enables pwdgen's random number generation  
**clap** (v.4.3.12) - allows pwdgen to take parameters and flags dynamically from the command line

## usage
pwdgen is designed to be called from the command line. Calling **/path/to/pwdgen.exe** (or simply "pwdgen" if you add it to your PATH) with no additional arguments will yield a result like this:

    > pwdgen
    Password: qxKL<>xE$&%9W
    length = 13 characters (medium), mode = 15 (nums;alpha_lower;alpha_upper;special_chars;)

By default, pwdgen is configured to generate a string at the 'medium' length setting, in mode 15 (which tells pwdgen to include all available character classes in the generation process).
These values can be altered using parameter arguments.

## parameters
pwdgen has two primary runtime settings:
length (the root length setting for the final returned string), and
mode (the setting for which character types pwdgen pulls from).

passing the **--help** flag to pwdgen ("pwdgen --help" in the command line) will show more information:

    > pwdgen --help
    Usage: pwdgen.exe [OPTIONS]
    
    Options:
      -l, --length <LENGTH>              Set the base length
      -m, --mode <MODE>                  Set the character class mode
          --bypass_primary_length_check  Bypass the primary length check
      -h, --help                         Print help

### --length
The **--length** parameter has 5 different settings:

    super short (4 - 7 characters),
    short (8 - 11 characters),
    medium (12 - 15 characters),
    long (16 - 19 characters),
    super long (20 - 23 characters)

and can take one of any of a few classes of inputs:

    "super_short", "short", "medium", "long", "super_long",
    "ss", "s", "m", "l", "ll",
    "0", "1", "2", "3", "4"

Here's an example of the **--length** parameter being used:

    > pwdgen --length long
    Password: SR!eo@BSg.9LtG9+%
    length = 17 characters (long), mode = 15 (nums;alpha_lower;alpha_upper;special_chars;)

Ordinarily, pwdgen won't be able to generate a string any longer than 23 characters; however, this can be worked around with the **--bypass_primary_length_check** flag, which I'll explain in a second.

### --mode
The **--mode** parameter has a total of 15 different settings, all referring to the different combinations of the four available character classes that can be included in pwdgen's active character pool.

The four character classes in pwdgen are *numbers, lower alphas, upper alphas,* and *special characters.* The 15 different modes correspond to the following combinations of these classes:

    mode 1 - special chars only
    mode 2 - upper alphas only
    mode 3 - upper alphas + special chars
    mode 4 - lower alphas only
    mode 5 - lower alphas + special chars
    mode 6 - lower alphas + upper alphas
    mode 7 - lower alphas + upper alphas + special chars
    mode 8 - nums only
    mode 9 - nums + special chars
    mode 10 - nums + upper alphas
    mode 11 - nums + upper alphas + special chars
    mode 12 - nums + lower alphas
    mode 13 - nums + lower alphas + special chars
    mode 14 - nums + lower alphas + upper alphas
    mode 15 - nums + lower alphas + upper alphas + special chars (include everything)

**--mode** takes one of any of two classes of input:

    decimal integers:
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15"
    
    four-digit binary sequences (for 1 - 15):
    "0001", "0010", "0011", "0100", 0101", "0110", "0111", "1000", "1001", "1010", "1011", "1100", "1101", "1110", "1111"

The reason why **--mode** also takes binary arguments is because each binary digit is meant to be thought of as a switch for whether or not to include the corresponding class,
similarly to the functionality found in Linux's "chmod" command.
Here's a visual aid:

    0       0       0       0
    ^       ^       ^       ^ include special characters
    ^       ^       ^ include upper alphas
    ^       ^ include lower alphas
    ^ include numbers

*This visual aid can also be found in MODE.txt.*

Ergo, passing "0101" (mode 5) tells pwdgen to include lower alphas and special characters, but exclude numbers and upper alphas,
as per the corresponding set bits; and so on for all values 1 through 15 (or 0b0001 through 0b1111).

This, in turn, explains the initially offputting order of class combinations represented by each subsequent mode number.

It's worth noting that this logic does *technically* imply the existence of a mode 0 (include nothing),
but allowing for such a thing in the code would cause **rand** to run into problems; besides, it wouldn't be very useful anyway.
Any value for **--mode** that does not compute will cause pwdgen to default to mode 15.

Here's an example of a decimal integer being passed to **--mode**:

    > pwdgen --mode 12
    Password: hku4e8h20clj5
    length = 13 characters (medium), mode = 12 (nums;alpha_lower;)

Alternatively, here's the same setting passed in binary:

    > pwdgen --mode 1100
    Password: 25c0hz9eoae117
    length = 14 characters (medium), mode = 12 (nums;alpha_lower;)

As you can see, pwdgen recognizes both of these arguments as indicating mode 12, and this relationship holds true for all values for **--mode**.

### --bypass_primary_length_check
As previously stated, the **--bypass_primary_length_check** flag is used in conjunction with **--length** to generate strings that are longer than 23 characters.

Running pwdgen with a value greater than 4 for **--length** will not execute properly.
However, **--bypass_primary_length_check** tells pwdgen to pass the value for **--length** through a different filter that allows for larger root length settings.

Here's an example of **--bypass_primary_length_check** in action that uses its alias, "--bplc":

    > pwdgen -l 8 -m 12 --bplc
    Password: vut0elw8scbemd5yiizlq95eghg02pff1xlo
    length = 36 characters (Eggs Benedict), mode = 12 (nums;alpha_lower;)

(The reason why it says "Eggs Benedict" in the length field is because I originally didn't intend for pwdgen to be able to make strings any longer than "super_long"
and only put it in there as a dummy case. I ended up leaving it in cus it's funny.)

Note that if you pass **--bypass_primary_length_check**, the **--length** parameter can only take integer inputs,
even if you decide to arbitrarily pass **--bplc** in a case where **--length** is set from 0 - 4.
passing "medium", "short", "ll", etc. in these cases will cause pwdgen to not work properly.
