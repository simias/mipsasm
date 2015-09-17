mod assembler;

use assembler::syntax::*;

fn main() {
    let mut asm = assembler::Assembler::new(0x80010000);

    let r =
        asm.assemble(&[
            Global("main"),

            Local("loop"),
            Bgez(R4, Label::Global("main")),
            Sll(R2, R1, 4),
            Srav(AT, K1, RA),
            Nop,

            Local("loop"),

            Local("otherlabel"),

            Move(S0, T1),
            Subu(R0, R3, R4),
            Bgez(R4, Label::Local("loop", 'b')),
            Nop,

            Local("loop"),

            Jal(Label::Global("main")),
            Nop
            ]);

    println!("Assembly result: {:?}", r);

    asm.dump();
}
