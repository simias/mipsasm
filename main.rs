mod assembler;

use assembler::syntax::*;

fn main() {
    let mut asm = assembler::Assembler::new(0);

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
            Nop,

            Li(T3, 0x80000000),
            Li(T4, 0x80000001),
            Li(T5, 0x00000002),
            Li(T6, 0x00000000),
            Li(T7, 0x0000ffff),

            Global("end"),

            J(Label::Global("end")),

            La(RA, Label::Global("end")),

            ]);

    println!("Assembly result: {:?}", r);

    asm.dump();
}
