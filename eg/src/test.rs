use formality_core::test_util::ResultTestExt;

use crate::execute_program;

mod talk_examples;

#[test]
fn add_integers() {
    execute_program(
        "
        1 + 2
    ",
    )
    .assert_ok(expect_test::expect!["3"])
}

#[test]
fn add_tuples() {
    execute_program(
        "
        (1, 2) + (3, 4)
    ",
    )
    .assert_ok(expect_test::expect!["(4, 6)"])
}

#[test]
fn variable() {
    execute_program(
        "
        let x = (1, 2);
        let y = (3, 4);
        x + y 
    ",
    )
    .assert_ok(expect_test::expect!["(4, 6)"])
}

#[test]
fn ill_typed() {
    execute_program(
        "
        (1, 2) + 2
    ",
    )
    .assert_err(expect_test::expect![[r#"
        check program

        Caused by:
            judgment `type_expr { expr: (1, 2) + 2, env: Env { program: (1, 2) + 2, type_variables: [], program_variables: {} } }` failed at the following rule(s):
              the rule "add" failed at step #0 (src/file.rs:LL:CC) because
                judgment `type_binary_expr { l: (1, 2), r: 2, env: Env { program: (1, 2) + 2, type_variables: [], program_variables: {} } }` failed at the following rule(s):
                  the rule "type_binary_expr" failed at step #1 (src/file.rs:LL:CC) because
                    judgment `type_expr_as { expr: 2, ty: (u32, u32), env: Env { program: (1, 2) + 2, type_variables: [], program_variables: {} } }` failed at the following rule(s):
                      the rule "type_expr_as" failed at step #1 (src/file.rs:LL:CC) because
                        condition evaluted to false: `ty == ty_expected`
                          ty = u32
                          ty_expected = (u32, u32)"#]])
}

#[test]
fn declare_function() {
    execute_program(
        "
        fn test(x: (u32, u32)) -> (u32, u32) {
            x
        }

        0
    ",
    )
    .assert_ok(expect_test::expect!["0"])
}

#[test]
fn call_function() {
    execute_program(
        "
        fn test(x: (u32, u32)) -> (u32, u32) {
            x
        }

        @test((0, 0))
    ",
    )
    .assert_ok(expect_test::expect!["(0, 0)"])
}

#[test]
fn call_generic_function() {
    execute_program(
        "
        fn identity<type A>(x: A) -> A {
            x
        }

        @identity<(u32, u32)>((1, 2)) + (3, 4)
    ",
    )
    .assert_ok(expect_test::expect!["(4, 6)"])
}

#[test]
fn ill_typed_call_generic_function() {
    execute_program(
        "
        fn identity<type A>(x: A) -> A {
            x
        }

        @identity<(u32, u32)>((1, 2)) + 4
    ",
    )
    .assert_err(expect_test::expect![[r#"
        check program

        Caused by:
            judgment `type_expr { expr: @ identity <(u32, u32)> ((1, 2)) + 4, env: Env { program: fn identity <type> (x : ^type0_0) -> ^type0_0 { x } @ identity <(u32, u32)> ((1, 2)) + 4, type_variables: [], program_variables: {} } }` failed at the following rule(s):
              the rule "add" failed at step #0 (src/file.rs:LL:CC) because
                judgment `type_binary_expr { l: @ identity <(u32, u32)> ((1, 2)), r: 4, env: Env { program: fn identity <type> (x : ^type0_0) -> ^type0_0 { x } @ identity <(u32, u32)> ((1, 2)) + 4, type_variables: [], program_variables: {} } }` failed at the following rule(s):
                  the rule "type_binary_expr" failed at step #1 (src/file.rs:LL:CC) because
                    judgment `type_expr_as { expr: 4, ty: (u32, u32), env: Env { program: fn identity <type> (x : ^type0_0) -> ^type0_0 { x } @ identity <(u32, u32)> ((1, 2)) + 4, type_variables: [], program_variables: {} } }` failed at the following rule(s):
                      the rule "type_expr_as" failed at step #1 (src/file.rs:LL:CC) because
                        condition evaluted to false: `ty == ty_expected`
                          ty = u32
                          ty_expected = (u32, u32)"#]])
}
