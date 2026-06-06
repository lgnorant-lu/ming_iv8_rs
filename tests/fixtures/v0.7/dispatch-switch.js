// Switch dispatch: switch(B[P++]) { case ... }
var bytecode = [0, 1, 2, 0];
var pc = 0;
var stack = [];
while (pc < bytecode.length) {
    switch(bytecode[pc++]) {
        case 0: stack.push(1); break;
        case 1: stack.push(2); break;
        case 2: stack.push(stack.pop() + stack.pop()); break;
    }
}
