use std::env::consts::EXE_EXTENSION;
use std::fs::File;
use std::io::prelude::*;
use std::io::*;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::Once;

use assert_json_diff::assert_json_eq;
use lazy_static::lazy_static;
use pretty_assertions::assert_eq;
use serde_json;

// The code was originally written by Jack O'Connor (@oconnor663)
// Taken from https://github.com/oconnor663/os_pipe.rs/blob/f41c58e503e1efc5e4d0edfcd2e756b3a81b4232/src/lib.rs#L281-L314
// Downloaded at Aug 13, 2022
// Licensed under:
// The MIT License (MIT)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.
fn build_and_find_path(name: &str) -> PathBuf {
    // This project defines some associated binaries for testing, and we shell out to them in
    // these tests. `cargo test` doesn't automatically build associated binaries, so this
    // function takes care of building them explicitly, with the right debug/release flavor.
    static CARGO_BUILD_ONCE: Once = Once::new();
    CARGO_BUILD_ONCE.call_once(|| {
        let mut build_command = Command::new("cargo");
        build_command.args(&["build", "--quiet"]);
        if !cfg!(debug_assertions) {
            build_command.arg("--release");
        }
        let build_status = build_command.status().unwrap();
        assert!(
            build_status.success(),
            "Cargo failed to build associated binaries."
        );
    });
    let flavor = if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    };
    Path::new("target")
        .join(flavor)
        .join(name)
        .with_extension(EXE_EXTENSION)
}

lazy_static! {
    static ref EXE_PATH: PathBuf = build_and_find_path("wordle");
}

pub struct TestCase {
    name: String,
    arguments: Vec<String>,
    input: String,
    answer: String,
}

impl TestCase {
    pub fn read(name: &str) -> Self {
        let case_dir = Path::new("tests").join("cases");
        let in_file = case_dir.join(format!("{}.in", name));
        let ans_file = case_dir.join(format!("{}.ans", name));
        let args_file = case_dir.join(format!("{}.args", name));

        let in_content = std::fs::read_to_string(in_file).unwrap();
        let ans_content = std::fs::read_to_string(ans_file).unwrap();
        let args_content = std::fs::read_to_string(args_file).unwrap();

        Self {
            name: name.to_string(),
            arguments: args_content
                .trim()
                .split("\n")
                .filter(|s| s != &"") // remove empty lines
                .map(|s| s.to_string())
                .collect(),
            input: in_content,
            answer: ans_content,
        }
    }

    fn execute_program_and_feed_input(&self) -> Child {
        let mut command = Command::new(EXE_PATH.as_os_str())
            .args(&self.arguments)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to execute process");

        // feed stdin
        command
            .stdin
            .take()
            .unwrap()
            .write_all(self.input.as_bytes())
            .unwrap();
        command
    }

    pub fn run_and_compare_result(&self) {
        let mut command = self.execute_program_and_feed_input();
        // read stdout from user program
        let mut output = Vec::new();
        command
            .stdout
            .take()
            .unwrap()
            .read_to_end(&mut output)
            .unwrap();
        let output = String::from_utf8(output).unwrap();

        // command.try_wait();

        // wait for the program to exit normally
        assert!(
            command.wait().expect("failed to wait on process").success(),
            "case {} should exit normally",
            self.name
        );

        // compare result
        assert_eq!(
            output.trim(),
            self.answer.trim(),
            "case {} incorrect",
            self.name
        );
    }

    pub fn run_and_compare_game_state(&mut self) {
        // read state before & end
        let case_dir = Path::new("tests").join("cases");
        let before_state_file = case_dir.join(format!("{}.before.json", self.name));
        let run_state_file = case_dir.join(format!("{}.run.json", self.name));
        let after_state_file = case_dir.join(format!("{}.after.json", self.name));

        // run with temporary state file
        std::fs::copy(&before_state_file, &run_state_file).unwrap();
        self.arguments.append(&mut vec![
            "--state".to_string(),
            run_state_file.to_str().unwrap().to_string(),
        ]);
        self.run_and_compare_result();

        // load state and compare with answer
        let run_state: serde_json::Value =
            serde_json::from_reader(BufReader::new(File::open(&run_state_file).unwrap())).unwrap();
        let answer_state: serde_json::Value =
            serde_json::from_reader(BufReader::new(File::open(&after_state_file).unwrap()))
                .unwrap();
        assert_json_eq!(run_state, answer_state);
    }

    pub fn run_and_expect_exit(&self) {
        let command = self.execute_program_and_feed_input();
        assert!(
            !command
                .wait_with_output()
                .expect("failed to wait on process")
                .status
                .success(),
            "case {} should exit with error",
            self.name
        );
    }
}
