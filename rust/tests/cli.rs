// This is free and unencumbered software released into the public domain.

#![cfg(feature = "cli")]

use std::process::{Command, Output};

const KEY: &str = "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a";

fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_edky"))
        .args(args)
        .env_remove("NO_COLOR")
        .env_remove("CLICOLOR")
        .env_remove("CLICOLOR_FORCE")
        .env_remove("FORCE_COLOR")
        .output()
        .expect("CLI should start")
}

fn assert_usage_error(args: &[&str]) -> String {
    let output = run(args);
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert_eq!(output.status.code(), Some(2), "{args:?}: {stderr}");
    assert!(output.stdout.is_empty(), "{args:?}");
    assert!(
        stderr.contains("Usage:") || stderr.contains("For more information, try '--help'."),
        "{args:?}: {stderr}"
    );
    assert!(!stderr.contains("panicked"), "{args:?}: {stderr}");
    stderr
}

#[test]
fn missing_commands_are_usage_errors() {
    assert_usage_error(&[]);
    for args in [
        vec!["--debug"],
        vec!["-d"],
        vec!["-v"],
        vec!["-vv"],
        vec!["--color", "never"],
        vec!["--color=always"],
    ] {
        assert!(assert_usage_error(&args).contains("subcommand"));
    }
}

#[test]
fn conversion_and_parsing_require_inputs() {
    for args in [
        vec!["convert"],
        vec!["parse"],
        vec!["--debug", "convert"],
        vec!["convert", "--debug"],
        vec!["convert", "--from", "hex", "--to", "hex"],
        vec!["parse", "--from", "hex"],
        vec!["parse", "--verbose"],
        vec!["convert", "--"],
    ] {
        assert!(assert_usage_error(&args).contains("<INPUTS>"));
    }
}

#[test]
fn informational_options_succeed_without_commands_or_inputs() {
    for args in [
        vec!["--help"],
        vec!["-h"],
        vec!["--version"],
        vec!["-V"],
        vec!["--license"],
        vec!["--debug", "--version"],
        vec!["-v", "--license"],
        vec!["convert", "--help"],
        vec!["parse", "--help"],
        vec!["list", "--help"],
    ] {
        let output = run(&args);
        assert!(output.status.success(), "{args:?}: {output:?}");
        assert!(!output.stdout.is_empty(), "{args:?}");
        assert!(output.stderr.is_empty(), "{args:?}: {output:?}");
    }
    assert_eq!(
        run(&["--version"]).stdout,
        format!("edky {}\n", env!("CARGO_PKG_VERSION")).as_bytes()
    );
    assert_eq!(run(&["--license"]).stdout, include_bytes!("../UNLICENSE"));
}

#[test]
fn valid_commands_accept_multiple_inputs() {
    for (args, expected) in [
        (vec!["convert", KEY, KEY], format!("{KEY}\n{KEY}\n")),
        (vec!["parse", KEY, KEY], "OK\n".to_owned()),
        (
            vec!["list"],
            edky::PUBLIC_KEY_FORMATS
                .iter()
                .map(|format| format!("{}\n", format.name()))
                .collect(),
        ),
    ] {
        let output = run(&args);
        assert!(output.status.success(), "{args:?}: {output:?}");
        assert_eq!(output.stdout, expected.as_bytes());
        assert!(output.stderr.is_empty(), "{args:?}: {output:?}");
    }
}

#[test]
fn invalid_inputs_and_options_do_not_report_success() {
    for args in [
        vec!["parse", KEY, "bad"],
        vec!["convert", "bad"],
        vec!["parse", ""],
    ] {
        let output = run(&args);
        assert!(!output.status.success(), "{args:?}");
        assert!(output.stdout.is_empty(), "{args:?}");
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert!(stderr.contains("invalid public key"), "{args:?}: {stderr}");
        assert!(!stderr.contains("panicked"), "{args:?}: {stderr}");
    }
    assert_usage_error(&["convert", "--from", "unknown", KEY]);
    assert_usage_error(&["convert", "--unknown", KEY]);
    assert_usage_error(&["--color", "rainbow", "list"]);
}

#[test]
fn verbosity_and_debug_add_stderr_details_without_changing_stdout() {
    for command in ["list", "convert", "parse"] {
        let inputs = if command == "list" {
            vec![]
        } else {
            vec![KEY, KEY]
        };
        let mut plain_args = vec![command];
        plain_args.extend_from_slice(&inputs);
        let plain = run(&plain_args);
        assert!(plain.status.success());
        assert!(plain.stderr.is_empty());

        let mut previous_len = 0;
        for flag in ["-v", "-vv", "-vvv", "--debug"] {
            for global_before_command in [true, false] {
                let mut args = if global_before_command {
                    vec![flag, command]
                } else {
                    vec![command, flag]
                };
                args.extend_from_slice(&inputs);
                let output = run(&args);
                assert!(output.status.success(), "{args:?}: {output:?}");
                assert_eq!(output.stdout, plain.stdout, "{args:?}");
                assert!(!output.stderr.is_empty(), "{args:?}");
                if global_before_command && flag != "--debug" {
                    assert!(output.stderr.len() > previous_len, "{args:?}: {output:?}");
                    previous_len = output.stderr.len();
                }
                if flag == "--debug" || flag == "-vvv" {
                    assert!(String::from_utf8(output.stderr).unwrap().contains("debug:"));
                }
            }
        }
    }
}

#[test]
fn color_controls_help_usage_errors_and_diagnostics() {
    for (color, colored) in [("always", true), ("never", false), ("auto", false)] {
        let combined = format!("--color={color}");
        for args in [
            vec!["--color", color, "--help"],
            vec!["--help", &combined],
            vec!["convert", "--help", "--color", color],
        ] {
            let output = run(&args);
            assert!(output.status.success(), "{args:?}: {output:?}");
            let stdout = String::from_utf8(output.stdout).unwrap();
            assert_eq!(stdout.contains("\x1b["), colored, "{args:?}: {stdout:?}");
        }
        for args in [vec![&combined, "--debug"], vec![&combined, "parse"]] {
            let stderr = assert_usage_error(&args);
            assert_eq!(stderr.contains("\x1b["), colored, "{args:?}: {stderr:?}");
        }
        let output = run(&["convert", "--debug", &combined, KEY]);
        assert!(output.status.success(), "{output:?}");
        assert_eq!(output.stdout, format!("{KEY}\n").as_bytes());
        assert_eq!(
            String::from_utf8(output.stderr).unwrap().contains("\x1b["),
            colored
        );
    }
}

#[test]
fn color_like_input_after_the_separator_is_not_an_option() {
    let output = run(&[
        "--color=never",
        "--debug",
        "convert",
        "--",
        "--color=always",
    ]);
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    assert!(!String::from_utf8(output.stderr).unwrap().contains("\x1b["));
}
