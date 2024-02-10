mod common;
use common::*;

const PROGRAM: &'static str = "\
	mov %l0, 0
	mov %l1, 0
	mov %l2, 1
loop:
	cmp %l0, %a0
	b.ge end
	add %l3, %l1, %l2
	mov %l1, %l2
	mov %l2, %l3
	add %l0, %l0, 1
	b loop
end:
	mov %o0, %l1
	swi
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
