# Calc New Roman
Calculator using Roman numerical system.

# Guide

Compiling
---------

- download and install Git from https://git-scm.com/
- download and install Rust programming language at https://www.rust-lang.org/tools/install
- clone this repository into your machine by using the following command in your console: 
        git clone https://github.com/lubosmudrak/calc_new_roman.git
- compile and run Calc New roman by using this command in your console while you are in app's root directory:
        cargo run


Usage
-----

commands:
help - display this manual
exit - exit application

Application usage:
you can input roman numbers with the mathematical operations you want to perform. State that your mathematical operation is finished by putting  \"=\" in it's last place.
Note: support for correct ordering of mathematical operations is not yet implemented. Operations are executed in the order in which you put them in.
There are 4 basic principles for writing Roman numerals as listed below:

The letters I, X, C can be repeated thrice in succession. Additionally, L, V, D cannot be repeated or the number is considered to be invalid.
If a lower value digit is written to the left of a higher value digit, it is subtracted.
If a lower value digit is written to the right of a higher value digit, it is added.
Only I, X, and C can be used as subtractive numerals.

License
---------
Licensed under MIT license https://opensource.org/licenses/MIT