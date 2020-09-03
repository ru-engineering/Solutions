fn main() {
    println!("Hello, world!");
}


/*
ANWSERS TO OTHER STUFF

U8; 0-255
U32; 0-4294967295
F32; 1.175494351E-38 - 3.402823466E+38
i16; -32768-32767

INTEGER OVERFLOW happens when the maximum or minimum value is passed.
Normaly this means that the number weaps around.
FX. if you have a u8 and do the operation 255+1 you would wrap to 0
However in some systems this can corrupt memory of surroding registers

in rust while debugging it does not allow you to overflow and the program will panic at runtime.
however overflow is allowed if the relese flag is enabled

*/
