# Helix Trainer

**Learn Helix keybindings by doing — 49 hands-on exercises you complete in your real editor.**

Helix Trainer generates a structured set of practice files that you open directly in [Zed](https://zed.dev) (or any editor with Helix mode). Each exercise teaches specific commands, gives you text to transform, and shows the expected result. You practice with real keybindings on real text — no simulations, no quizzes, just deliberate practice.

```
helix-trainer init
cd helix-exercises
# Open in Zed. Start editing.
```

---

## Why

Reading a keybinding reference is like reading a phrasebook — you recognize words but can't speak the language. Muscle memory comes from repetition in context.

Helix Trainer gives you that context: structured exercises that progressively build your fluency, from basic motion (`h`/`j`/`k`/`l`) to multi-selection workflows that feel like a superpower.

## How It Works

1. **Install** the CLI tool
2. **Generate** an exercise project with `helix-trainer init`
3. **Open** the project in Zed (or any Helix-mode editor) as a workspace
4. **Edit** each `.hxt` file — the PRACTICE section contains text to transform, the EXPECTED section shows the goal
5. **Verify** your work with `helix-trainer verify <file>` or just compare visually
6. **Track** your progress with `helix-trainer progress`

The editor is the trainer. No separate app, no context-switching.

## Install

```sh
# From crates.io
cargo install helix-trainer

# Or build from source
git clone https://github.com/yourusername/helix-trainer
cd helix-trainer
cargo install --path .
```

Pre-built binaries for macOS (Intel + Apple Silicon), Linux, and Windows are available on the [Releases](../../releases) page.

## Quick Start

```sh
# Generate the exercise project
helix-trainer init

# Or specify a custom directory
helix-trainer init ~/code/helix-practice

# Enter the project
cd helix-exercises

# Open in Zed
zed .

# Start with the first exercise
# exercises/01-movement/01-basic-motion.hxt
```

Open the file, read the instructions at the top, edit the PRACTICE section to match EXPECTED. That's it.

## Exercise Format

Every exercise is a self-contained `.hxt` file with a clear structure:

```
╔══════════════════════════════════════════════════════════════════╗
║  HELIX TRAINER — Exercise 6.1: Regex Select                     ║
║  Category: Multi-Selection         Difficulty: ★★☆ Intermediate  ║
╚══════════════════════════════════════════════════════════════════╝

COMMANDS TO LEARN
─────────────────
  s     Select all regex matches within the current selection
  c     Change (delete selection and enter insert mode)

INSTRUCTIONS
────────────
  The paragraph below uses "color" five times. Change every
  instance to "colour" — all at once.

  The Helix way:
  1. Select the entire practice block
  2. Press  s , type  color , press Enter
  3. Press  c , type  colour , press Esc
  Done. Five replacements, zero repetition.

────────────────────────── PRACTICE ──────────────────────────────

The color of the sky changes throughout the day. At dawn, a warm
color spreads across the horizon. By noon the color shifts to a
brilliant blue. Artists know that color theory is essential for
painting realistic scenes. The right color can set the entire
mood of a composition.

────────────────────────── EXPECTED ──────────────────────────────

The colour of the sky changes throughout the day. At dawn, a warm
colour spreads across the horizon. By noon the colour shifts to a
brilliant blue. Artists know that colour theory is essential for
painting realistic scenes. The right colour can set the entire
mood of a composition.

──────────────────────────────────────────────────────────────────
HINTS (read only if stuck):
  ...
```

- **COMMANDS TO LEARN** — the keybindings the exercise teaches
- **INSTRUCTIONS** — what to do and how
- **PRACTICE** — the text you edit with real Helix commands
- **EXPECTED** — what PRACTICE should look like when you're done
- **HINTS** — placed below EXPECTED so you scroll past the answer first

## Curriculum

49 exercises across 12 modules, organized in 4 progressive tiers.

### Tier 1 — Fundamentals

| Module | Exercises | What You'll Learn |
|--------|-----------|-------------------|
| **01 Movement** | 5 | `h`/`j`/`k`/`l`, `w`/`b`/`e`, `f`/`t` (multiline!), line navigation, scrolling |
| **02 Selection** | 4 | `x` (line select), `v` (extend mode), `;` (collapse), `%` (select all) |
| **03 Changes** | 5 | `d`/`c` (delete/change), `y`/`p` (yank/paste), `u`/`U` (undo/redo), indent, case |

### Tier 2 — Intermediate

| Module | Exercises | What You'll Learn |
|--------|-----------|-------------------|
| **04 Text Objects** | 4 | `m i`/`m a` + delimiters, words, functions, arguments (tree-sitter) |
| **05 Surround** | 4 | `m s` (add), `m r` (replace), `m d` (delete) surround characters |
| **06 Multi-Selection** | 5 | `s` (regex select), `S` (split), `C` (cursors), `K`/`Alt-K` (keep/remove) |
| **07 Search** | 3 | `/` search, `*` (use selection as pattern), global find-and-replace workflow |

### Tier 3 — Advanced

| Module | Exercises | What You'll Learn |
|--------|-----------|-------------------|
| **08 Goto Mode** | 4 | `g d`/`g r` (LSP), `g n`/`g p` (buffers), `g o`/`g u` (Zed git ops) |
| **09 Space Mode** | 3 | `Space f` (files), `Space s` (symbols), `Space r` (rename), clipboard |
| **10 Unimpaired** | 3 | `]d`/`[d` (diagnostics), `]f`/`[f` (functions), indent navigation |

### Tier 4 — Mastery

| Module | Exercises | What You'll Learn |
|--------|-----------|-------------------|
| **11 Advanced Workflows** | 5 | Rename variable, extract function, reformat data, bulk transform, Vim-to-Helix |
| **12 Challenges** | 4 | Speed edit, minimal keystrokes, real-world refactor, code golf |

## CLI Reference

All commands except `init` run from inside the generated project directory.

### `helix-trainer init [dir]`

Generate a new exercise project.

```sh
helix-trainer init                    # Creates ./helix-exercises/
helix-trainer init ~/my-training      # Custom location
```

### `helix-trainer progress`

Show completion stats with progress bars for each module.

```
  HELIX TRAINER — Progress

  Module                       Progress
  ─────────────────────────────────────────
  ✓ 01-movement                  ███████████████   5/5  100%
  ✓ 02-selection                 ███████████████   4/4  100%
    03-changes                   ██████░░░░░░░░░   2/5  40%
    04-text-objects              ░░░░░░░░░░░░░░░   0/4  0%
    ...
  ─────────────────────────────────────────
  Total: 11/49 exercises completed (22%)
```

### `helix-trainer verify [file]`

Check exercises by diffing PRACTICE against EXPECTED. With no argument, checks all exercises.

```sh
# Check all exercises
helix-trainer verify

# Check a specific exercise
helix-trainer verify exercises/04-text-objects/01-delimiter-objects.hxt
# ✓ exercises/04-text-objects/01-delimiter-objects.hxt
```

### `helix-trainer next`

Print the path of the next incomplete exercise.

```sh
helix-trainer next
# exercises/03-changes/03-undo-redo-repeat.hxt
```

### `helix-trainer reset [file]`

Restore exercises to their original state from the installed package templates. With no argument, resets all exercises.

```sh
# Reset a specific exercise
helix-trainer reset exercises/01-movement/01-basic-motion.hxt
# ✓ Reset 01-movement/01-basic-motion.hxt

# Reset everything — start fresh
helix-trainer reset

## The Helix Mental Model

Helix reverses Vim's editing grammar:

| | Vim | Helix |
|---|---|---|
| **Model** | Verb → Object (`dw`) | Selection → Action (`wd`) |
| **Delete word** | `dw` | `wd` |
| **Change inside quotes** | `ci"` | `mi"c` |
| **Delete line** | `dd` | `xd` |
| **Yank paragraph** | `yap` | `mapy` |

The core insight: **you always see what will be affected before you act**. Every motion creates a visible selection. You refine it, then commit. No more "oops, I deleted the wrong thing."

Multi-selection takes this further. Instead of `:%s/old/new/g`, you:

```
%           Select entire file
s old       Split into selections on "old"
c new       Change all selections simultaneously
```

This is the workflow you'll master in Module 06.

## Tips for Getting the Most Out of It

**Follow the progression.** The exercises build on each other. Module 04 (text objects) assumes you know Module 03 (changes). Module 06 (multi-selection) assumes you know Module 04.

**One module per session.** Don't grind through all 49 in a day. Do a module, then use those commands in your real work. Come back tomorrow.

**Use the which-key popup.** In Zed with Helix mode, press any prefix key (`g`, `m`, `Space`, `z`, `]`, `[`) and pause — a popup shows all available sub-commands. This is your cheat sheet.

**`;` is your reset button.** If a selection goes wrong, press `;` to collapse it back to a cursor and try again. Build this habit early.

**Read the hints last.** They're placed below EXPECTED deliberately. Try each exercise without hints first. Struggle is where learning happens.

**Repeat the hard ones.** Use `helix-trainer reset <file>` to redo any exercise. The challenges in Module 12 are designed for repeated practice.

## Compatibility

Helix Trainer is built for [Zed](https://zed.dev) with `helix_mode: true`, but the exercises work in any editor that supports Helix keybindings:

- **Zed** — native Helix mode (`"helix_mode": true` in settings.json)
- **Helix** — the exercises use standard Helix keybindings
- **Neovim** — with a Helix emulation plugin

The CLI is a single static binary with no runtime dependencies. The exercises themselves are plain text files embedded in the binary.

## Contributing

Contributions are welcome! Here's how you can help:

### Adding exercises

1. Follow the `.hxt` format (see any existing exercise for reference)
2. Place the file in the appropriate module directory
3. Ensure PRACTICE and EXPECTED sections are bounded by the marker lines
4. Test with `helix-trainer verify` to confirm the parser handles your file
5. Include progressive hints

### Exercise quality checklist

- [ ] Instructions are clear enough to follow without prior Helix experience for that tier
- [ ] PRACTICE text is realistic (code, prose, or data — not lorem ipsum)
- [ ] EXPECTED result is achievable with the listed commands
- [ ] Hints are ordered from gentle nudge to explicit keystroke sequence
- [ ] The exercise teaches something that builds on previous modules

### Reporting issues

If an exercise has incorrect expected output, unclear instructions, or a keybinding that doesn't work in Zed's Helix mode, please [open an issue](../../issues).

## How It's Built

A single-binary Rust CLI with minimal dependencies:

- **`src/hxt.rs`** — Pure parser for `.hxt` files: extracts PRACTICE/EXPECTED sections, diffs them
- **`src/commands/verify.rs`** — Verifies exercises against expected output
- **`src/commands/progress.rs`** — Scans exercise files and renders a progress dashboard
- **`src/commands/reset.rs`** — Restores exercises from embedded templates
- **`src/commands/init.rs`** — Extracts embedded exercises to a new directory
- **`src/main.rs`** — CLI entry point with clap subcommands

The 49 exercises are compiled into the binary via `include_dir!`. `helix-trainer init` extracts them to disk. That's the entire architecture.

## License

MIT

## Acknowledgments

- [Helix Editor](https://helix-editor.com) — for the selection-first editing model
- [Zed](https://zed.dev) — for bringing Helix mode to a modern editor
- [Kakoune](https://kakoune.org) — the original inspiration for selection-first editing
- Vim's `vimtutor` — the original "learn by editing" concept that inspired this project
