

# Final Project: Wordle

Final Project (1) for the Rust course "Programming Practice" in the Summer 2022 semester.

## Program Structure

```c
src
├── app
│   ├── colors.rs         // Color constants
│   ├── definition.rs     // Word definition panel
│   ├── grid.rs           // Letter matrix
│   ├── keyboard.rs       // Keyboard component and input monitoring
│   ├── letter.rs         // Letter component
│   ├── metrics.rs        // Size and layout-related constants
│   ├── settings.rs       // Settings panel
│   ├── stats.rs          // Statistics panel
│   ├── utils.rs          // Utility functions
│   └── visuals.rs        // Visual style (Light / Dark mode)
├── app.rs                // GUI
├── args.rs               // Argument parsing and validation
├── builtin_words.rs      // Built-in word list
├── dict.rs               // Built-in dictionary
├── game.rs               // Game logic
├── main.rs               // CLI
└── stats.rs              // Statistics recording and storage
```

Modules at the crate level (except `app`) are shared by both CLI and GUI, while `app` and its submodules are exclusive to the GUI.

## Main Game Features

### CLI

Launching the program directly will enter the CLI interactive mode. In this mode, the player will first be asked to specify the game's answer, and then a Wordle game will begin.

![CLI](images/cli.png)

In interactive mode, after each guess, the results of all guesses and the status of each letter will be displayed. If `HINT` is entered, a hint will be provided:

![Hint](images/hint.png)

After the game ends, the word's definition will be shown, and the player will be asked whether to play another round.

![Definition](images/definition.png)

In CLI mode, certain parameters can be specified to customize the gaming experience.

![CLI Options](images/options.png)

| Parameter                     | Sub-parameter      | Function                                                                 | Notes                                                                            |
| ----------------------------- | ------------------ | ------------------------------------------------------------------------ | -------------------------------------------------------------------------------- |
| `--acceptable-set` / `-a`     | Path `<FILE>`      | Specifies the allowed guess word list, with one 5-letter word per line   |                                                                                  |
| `--config` / `-c`             | Path `<FILE>`      | Specifies the default configuration file, in JSON format                 | CLI arguments take higher priority than the configuration file                   |
| `--day` / `-d`                | Integer `<DAY>`    | Specifies the game day, i.e., the seed and day determine the answer      | Depends on `--random`; range is 1 to the size of the answer word list (inclusive) |
| `--difficult` / `-D`          |                    | Enables difficult mode, where each guess must use the hints from the previous guess | |
| `--final-set` / `-f`          | Path `<FILE>`      | Specifies the answer word list, with one 5-letter word per line          | The answer word list must be a subset of the guess word list                     |
| `--gui` / `-g`                |                    | Launches the GUI                                                         | No other parameters will be parsed in this case                                  |
| `--help` / `-h`               |                    | Displays help information                                                |                                                                                  |
| `--random` / `-r`             |                    | Randomly selects the answer                                              | Conflicts with `--word`                                                          |
| `--seed` / `-s`               | Integer `<SEED>`   | Specifies the random number seed                                         | Depends on `--random`                                                            |
| `--state` / `-S`              | Path `<FILE>`      | Enables game state storage and sets the storage path                     |                                                                                  |
| `--stats` / `-t`              |                    | Displays statistics after the game ends                                  |                                                                                  |
| `--word` / `-w`               | Word `<WORD>`      | Specifies the answer                                                     | Conflicts with `--random`; the answer should be in the answer word list          |

The following will demonstrate some command-line parameter functionalities as well as error input detection.

![Difficult Mode](images/difficult.png)

![Statistics](images/stats.png)

### GUI

The GUI supports both local execution and web deployment via WebAssembly compilation. The project is currently deployed [here](https://abmfy.github.io/wordle/).

![GUI](images/gui.png)

The GUI makes full use of the Enter and Backspace keys for information display and interaction. For example, in normal mode, the Enter key is disabled:

![Enter Disabled](images/enter_disabled.png)

The Enter key only becomes active when 5 letters are entered and the word exists in the word list:

![Enter Enabled](images/enter_enabled.png)

When the game ends, the Backspace key transforms into the "Next Game" button, and if the word was not guessed correctly, the Enter key will display the correct answer:

![Enter With Answer](images/answer.png)

The settings panel provides toggles for difficult mode, as well as selection for random seed and game day. Notably, in difficult mode, the GUI switches to a deep dark mode:

![Dark Mode](images/dark.png)

Typing `HINT` in the keyboard area will transform the Enter key into a hint button; clicking it provides a hint:

![Hint Button](images/hint_button.png)

![Hint Shown](images/hint_got.png)

The statistics panel will display statistics:

![Statistics Panel](images/stat_panel.png)

The definition panel will display the word's definition after the game ends:

![Definition Panel](images/def_panel.png)

Additionally, the GUI has been adapted for mobile devices, allowing normal gameplay on phones. When running on mobile, the three panel areas are collapsed under a toggle button to avoid obstructing the letter matrix:

<img src="images/mobile.png" alt="Mobile Mode" style="width: 50%;" />

<img src="images/mobile_panel.png" alt="Mobile Mode Panel" style="width: 50%;" />

## Implementation of Advanced Requirements

### GUI

Implemented using the [egui](https://crates.io/crates/egui) library and the [eframe](https://crates.io/crates/eframe) framework. The letter matrix and keyboard are implemented by directly drawing graphics and listening for click and keyboard input events. Animation effects are achieved via `egui::Contenxt::animate_value_with_time`. The dark mode effect is implemented by switching the global `Visuals` and adding transition animations. Mobile adaptation is achieved by detecting screen dimensions.

After packaging the compiled WebAssembly using [trunk](https://trunkrs.dev), it can be deployed to a server.

### Hints

The hint feature is implemented in a straightforward manner: it selects a word from the word list that matches all previous hints. Since players rarely use hints early in the game (first or second guess), this approach works well.

### Word Definitions

The implementation of word definitions primarily focuses on dictionary processing.

First, data from the New Oxford American Dictionary is extracted from macOS's Dictionary.app, and parsed using the [JadedTuna/apple-dictionary](https://github.com/JadedTuna/apple-dictionary) project to obtain [`dictionary.xml`](https://cloud.tsinghua.edu.cn/f/f165e853e90441a78f13/) (due to its size, this raw dictionary data is not included in the repository). Subsequently, `dict_gen.py` is used to preprocess the dictionary, including extracting corresponding entries for the answer word list, mapping derived words to their root forms, supplementing a few missing words, and generating the `assets/dict.json` file.

## Reflections on Completing the Assignment

The week spent completing this assignment was busy and fulfilling. While implementing the basic features, I was deeply impressed by how, despite Rust being a relatively young language, it boasts a thriving ecosystem. Third-party libraries such as `console`, `clap`, `rand`, and `serde_json` provided immense help in implementing core functionalities. Libraries like `clap` and `serde_json` fully leverage Rust's macros to enable convenient and rapid data structuring, offering a quick and easy development experience while maintaining static safety. Rust's powerful compile-time checks also allowed most memory-related bugs to be caught by the compiler, significantly improving development efficiency.

When starting to implement the advanced features, I noticed that egui can compile to both native and WebAssembly, so I decided to use egui directly for the GUI. During the first two or three days, GUI development progress was actually quite slow because egui differs from most common UI frameworks: it uses immediate mode layout and lacks an event system. While this brings development speed, it also introduces a major issue: relatively weak layout capabilities. Therefore, when implementing the letter matrix and keyboard, I directly drew them on the screen at pre-calculated coordinates to solve the layout problem. After a day of exploration, I finally drew the first version of the interface:

![First Edition of GUI](images/first_edition.png)

If the goal was merely to create a "playable" GUI, it probably wouldn't have taken this long. Most of the development time was actually spent pursuing details:

+ When letter states change, I wanted some animation effects. The flip effect from the original Wordle is not easy to implement in the egui framework, so I used simple color gradients. This gradient effect is applied not only to the colors in the letter matrix but is also visible when keyboard colors change.
+ To add a sense of "sophistication," the entire UI switches to dark mode in difficult mode: not just a black background, but a complete color scheme change. However, egui's built-in visual style changes lack transition animations, which felt very abrupt to me. So, I spent an afternoon adding transition animations for switching between light/dark modes.
+ Where should game results be prompted? I pondered this for a long time and eventually turned to the Enter and Backspace keys. Thus, they gradually took on more and more unconventional functions. In this project, the Enter key handles: input validation prompts, submission, answer display, and hint acquisition, while the Backspace key handles: backspacing and starting the next game.
+ When the window is scaled down significantly, the letter matrix covers the keyboard, and on mobile devices, the keyboard becomes very large... So, I implemented some responsive sizing so that mobile users can also enjoy Wordle pleasantly.
+ On mobile, if the three top-left panels use `Frame`, they get obscured by the letter matrix (since it's drawn directly); if they use `Window`, they obscure the letter matrix. Ultimately, I separated the display for mobile and desktop: mobile uses a collapsed `Window` containing three `CollapsingHeader` components, while desktop directly uses three `Frame` components.
+ After the answer is revealed, you often wonder: "Wow, what is this word?" So, I added a feature to display word definitions after the game ends. This feature isn't particularly difficult, but dictionary preprocessing is quite tedious. For example, some words are derived from others or have oddly formatted root forms in the dictionary (éclat, ’twixt, quasi-), requiring mapping to their root forms to find definitions.
+ Where to place the hint feature? Add a button? But that might encourage overuse. Noticing that the word HINT has only four letters, I thought of making the Enter button transform into a hint button when the input matches HINT. This doesn't interfere with normal guessing (except in the extremely rare case where the guess is HINTS), and makes the hint feature instantly accessible. This raises another question: how do players know that typing HINT provides a hint? The answer is the definition panel. Definitions only appear after the game ends, while hints are only needed during gameplay. So, if the definition panel is opened during gameplay, it shows that typing HINT will yield a hint.

After development was complete, Rust's ability to compile directly to WebAssembly amazed me once again. Deploying the compiled web page was also very smooth, and the final product overall met my own expectations. Throughout the development process, I also learned how to organize and manage Rust projects. To me, Rust is a language that reshapes the entire way one thinks about programming. I look forward to learning more new knowledge in my continued Rust studies!
