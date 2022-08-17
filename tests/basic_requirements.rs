use ntest::timeout;

mod common;
use common::TestCase;

#[test]
#[timeout(3000)]
fn test_01_20_pts_basic_one_game() {
    // one single game that succeeds
    TestCase::read("01_01_single_game").run_and_compare_result();
    // yet another game that fails
    TestCase::read("01_02_single_game").run_and_compare_result();
    // a game with invalid input
    TestCase::read("01_03_single_game_with_invalid_input").run_and_compare_result();
}

#[test]
#[timeout(1000)]
fn test_02_5_pts_specify_answer() {
    TestCase::read("02_01_specify_answer").run_and_compare_result();
}

#[test]
#[timeout(2000)]
fn test_03_10_pts_difficult_mode() {
    // difficult mode enabled, no invalid input
    TestCase::read("03_01_difficult_mode").run_and_compare_result();
    // difficult mode enabled with invalid input
    TestCase::read("03_02_difficult_mode").run_and_compare_result();
}

#[test]
#[timeout(2000)]
fn test_04_5_pts_continue_game_and_statistics() {
    // 2 continuous games in normal mode
    TestCase::read("04_01_statistics_1").run_and_compare_result();
    // 5 continuous games in difficult mode with invalid input
    TestCase::read("04_02_statistics_2").run_and_compare_result();
}

#[test]
#[timeout(3000)]
fn test_05_5_pts_specify_offset_and_seed() {
    // specify offset and seed
    TestCase::read("05_01_specify_seed").run_and_compare_result();
    // specify offset and seed with 3 continuous games
    TestCase::read("05_02_specify_seed").run_and_compare_result();
    // specify answer, offset and seed (conflict args)
    TestCase::read("05_03_conflict_args").run_and_expect_exit();
}

#[test]
#[timeout(2000)]
fn test_06_5_pts_specify_word_list() {
    // specify word list + offset + seed
    TestCase::read("06_01_specify_word_list").run_and_compare_result();
    // specify invalid word list (answer words are not subset of available words)
    TestCase::read("06_02_invalid_word_list").run_and_expect_exit();
}

#[test]
#[timeout(2000)]
fn test_07_5_pts_save_game_state() {
    // save game state after several rounds and compare JSON
    TestCase::read("07_01_save_state").run_and_compare_game_state();
    // load game state, check statistics and JSON output after several rounds
    TestCase::read("07_02_load_state").run_and_compare_game_state();
    // load game state from an invalid JSON
    TestCase::read("07_03_invalid_json_format").run_and_expect_exit();
}

#[test]
#[timeout(2000)]
fn test_08_5_pts_config_file() {
    // use config file to specify word list, offset and seed
    TestCase::read("08_01_config_file").run_and_compare_result();
    // override config in command line options
    TestCase::read("08_02_config_override").run_and_compare_result();
}
