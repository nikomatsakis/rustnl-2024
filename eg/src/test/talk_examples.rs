use expect_test::expect;
use formality_core::test_util::ResultTestExt;

use crate::execute_program;

#[test]
fn example1() {
    execute_program(
        "
        22 + 44
    ",
    )
    .assert_ok(expect!["66"])
}

#[test]
fn example2() {
    execute_program(
        "
        let x = (1, 2);
        let y = (22, 44);
        x + y
    ",
    )
    .assert_ok(expect!["(23, 46)"])
}

#[test]
fn example3() {
    execute_program(
        "
        fn add(x: u32, y: u32) -> u32 {
            x + y
        }
        
        @add(22, 44)
    ",
    )
    .assert_ok(expect!["66"])
}

#[test]
fn example4() {
    execute_program(
        "
        fn add<type A>(x: A, y: A) -> A {
            x + y
        }
        
        let x = @add<u32>(22, 44);
        let y = @add<(u32, u32)>((1, 2), (3, 4));
        (x, y)
    ",
    )
    .assert_ok(expect!["(66, (4, 6))"])
}

#[test]
fn example5() {
    execute_program(
        "
        3 + (4, 5)
    ",
    )
    .assert_err(expect![[r#"
        check program

        Caused by:
            judgment `type_expr { expr: 3 + (4, 5), env: Env { program: 3 + (4, 5), program_variables: {} } }` failed at the following rule(s):
              the rule "add" failed at step #0 (src/file.rs:LL:CC) because
                judgment `type_binary_expr { l: 3, r: (4, 5), env: Env { program: 3 + (4, 5), program_variables: {} } }` failed at the following rule(s):
                  the rule "type_binary_expr" failed at step #1 (src/file.rs:LL:CC) because
                    judgment `type_expr_as { expr: (4, 5), ty: u32, env: Env { program: 3 + (4, 5), program_variables: {} } }` failed at the following rule(s):
                      the rule "type_expr_as" failed at step #1 (src/file.rs:LL:CC) because
                        condition evaluted to false: `ty == ty_expected`
                          ty = (u32, u32)
                          ty_expected = u32"#]])
}
