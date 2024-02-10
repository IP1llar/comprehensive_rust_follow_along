fn main() {
    // Blocks: each block has a value and a type, which are those of the last expression of the
    // block
    // If the last expression ends with `;`, then the resulting value and type is `()`.
    let z = 13;
    let x = {
        let y = 10;
        println!("y: {y}");
        z - y
    };
    println!("x: {x}");

    // Scopes and shadowing
    // A variable's scope is limited to the enclosing block.
    // You can shadow variables, both those from outer scopes and variables from the same scope
    let a = 10;
    println!("before: {a}");
    {
        let a = "hello";
        println!("inner scope: {a}");

        let a = true;
        println!("shadowed in inner scope: {a}");
    }

    println!("after: {a}");
    // Shadowing is different from mutation, because after shadowing both variables' memory
    // locations exist at the same time. Both are available under the same name, depending where
    // you use it in the code.
    // Shadowing is convenient for holding on to values after `.unwrap()`
}
