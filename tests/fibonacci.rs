mod common;
use common::*;

const PROGRAM: &'static str = "\
	mov r9, 0
	mov r10, 1
	mov r8, 0
loop:
	cmp r8, r1
	b.ge end
	add r11, r9, r10
	mov r9, r10
	mov r10, r11
	add r8, r8, 1
	b loop
end:
	mov r8, r9
	csww 76, r0
";

#[test]
fn fibonacci() {
	let program = assemble(PROGRAM);
	assert_eq!(run(&program, 0), 0);
	assert_eq!(run(&program, 1), 1);
	assert_eq!(run(&program, 2), 1);
	assert_eq!(run(&program, 3), 2);
	assert_eq!(run(&program, 4), 3);
	assert_eq!(run(&program, 5), 5);
	assert_eq!(run(&program, 6), 8);
	assert_eq!(run(&program, 7), 13);
}
