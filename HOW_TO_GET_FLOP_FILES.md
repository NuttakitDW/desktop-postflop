# How to Get .flop or .bin Files

This guide explains how to obtain `.flop` or `.bin` solver files for use with the JSON Viewer feature in Desktop Postflop.

## Overview

`.flop` and `.bin` files are binary files containing:
- Game configuration (board, ranges, bet sizes)
- Solver state (strategies, equity, expected values)
- Metadata (memo, solved status, storage mode)

## Methods to Get These Files

### Method 1: Use the Desktop Postflop Application (Easiest)

This is the **recommended method** for most users.

#### Step 1: Configure Your Game
1. Launch Desktop Postflop application
2. Set up your game configuration:
   - **OOP Range**: Define out-of-position player's range
   - **IP Range**: Define in-position player's range
   - **Board**: Select flop cards (3 cards minimum)
   - **Tree Configuration**: Set bet sizes, pot size, stack size

#### Step 2: Run the Solver
1. Click "**Run Solver**" in the sidebar
2. Configure solver settings:
   - Max iterations (e.g., 1000)
   - Target exploitability (e.g., 0.5% of pot)
   - Memory options
3. Click "**Start**" and wait for solving to complete

#### Step 3: Save the File
1. Click "**Save**" button in the navbar
2. Choose location and filename
3. Add a memo (optional, e.g., "BTN vs BB SRP Kh Qh 7c")
4. File is saved as `.flop` format

**Result**: You now have a `.flop` file ready to load and view!

---

### Method 2: Use Existing Sample Files

The project includes sample files for testing.

#### Check for Sample Files
```bash
# Look in the postflop-solver directory
ls -lh src-tauri/postflop-solver/*.flop

# Example output:
# -rw-r--r-- 1 user staff 349M Feb 4 00:36 solution.flop
```

#### Available Sample File
- **Location**: `src-tauri/postflop-solver/solution.flop`
- **Size**: ~349MB (solved game with full data)
- **Usage**: Load this file to test the JSON viewer

#### How to Use Sample File
1. Run Desktop Postflop: `npm run tauri dev`
2. Click "Load" → Navigate to `src-tauri/postflop-solver/`
3. Select `solution.flop`
4. JSON Template will automatically appear

---

### Method 3: Create Files Programmatically (Advanced)

For developers who want to create files via code or scripts.

#### Using Rust (postflop-solver library)

**Prerequisites**:
- Rust installed (`rustup`)
- Clone the repository

**Example Code**:
```rust
use postflop_solver::*;

fn main() {
    // Define ranges
    let oop_range = "66+,A8s+,A5s-A4s,AJo+,K9s+,KQo,QTs+,JTs,96s+,85s+,75s+,65s,54s";
    let ip_range = "QQ-22,AQs-A2s,ATo+,K5s+,KJo+,Q8s+,J8s+,T7s+,96s+,86s+,75s+,64s+,53s+";

    // Configure cards
    let card_config = CardConfig {
        range: [oop_range.parse().unwrap(), ip_range.parse().unwrap()],
        flop: flop_from_str("Td9d6h").unwrap(),
        turn: NOT_DEALT,
        river: NOT_DEALT,
    };

    // Configure tree
    let bet_sizes = BetSizeOptions::try_from(("60%, e, a", "2.5x")).unwrap();
    let tree_config = TreeConfig {
        initial_state: BoardState::Flop,
        starting_pot: 200,
        effective_stack: 900,
        rake_rate: 0.0,
        rake_cap: 0.0,
        flop_bet_sizes: [bet_sizes.clone(), bet_sizes.clone()],
        turn_bet_sizes: [bet_sizes.clone(), bet_sizes.clone()],
        river_bet_sizes: [bet_sizes.clone(), bet_sizes],
        turn_donk_sizes: None,
        river_donk_sizes: Some(DonkSizeOptions::try_from("50%").unwrap()),
        add_allin_threshold: 1.5,
        force_allin_threshold: 0.15,
        merging_threshold: 0.1,
        max_raises_per_street: 0, // unlimited
    };

    // Create game
    let action_tree = ActionTree::new(tree_config).unwrap();
    let mut game = PostFlopGame::with_config(card_config, action_tree).unwrap();
    game.allocate_memory(false);

    // Solve
    let max_iterations = 1000;
    let target_exploitability = game.tree_config().starting_pot as f32 * 0.005;
    solve(&mut game, max_iterations, target_exploitability, true);

    // Save to file
    save_data_to_file(
        &game,
        "My custom game",  // memo
        "my_game.flop",    // filename
        None               // compression level (None = no compression)
    ).unwrap();

    println!("Saved to my_game.flop");
}
```

**Run the Example**:
```bash
cd src-tauri/postflop-solver
cargo run --release --example file_io
```

**With Compression** (requires `zstd` feature):
```rust
// Add to Cargo.toml:
// postflop-solver = { path = "...", features = ["zstd"] }

save_data_to_file(&game, "memo", "game.flop", Some(10)).unwrap();
// Compression level: 1-22 (10 is a good balance)
```

---

### Method 4: Download from Other Users

If others have shared their solver files:

#### File Sharing Platforms
- GitHub repositories with solver files
- Poker forums (2+2, Discord communities)
- Personal websites of poker players/coaches

#### Verification
Always verify files from untrusted sources:
```bash
# Check file size (should be reasonable)
ls -lh downloaded_file.flop

# Try loading in Desktop Postflop
# If it loads successfully, it's valid
```

---

## File Format Comparison

### .flop Files
- **Primary format** for Desktop Postflop
- Contains full game state
- Can be compressed (optional)
- Typical size: 1MB - 500MB depending on game complexity

### .bin Files
- **Alternative format** (same structure as .flop)
- Fully compatible with Desktop Postflop
- Just a different extension convention
- Used in some Rust examples

**Note**: Both formats are identical in structure. The file extension is just a naming convention.

---

## Storage Modes

Files can be saved with different storage modes:

### Flop Storage Mode
- Stores only up to flop actions
- Smallest file size
- Cannot view turn/river results

### Turn Storage Mode
- Stores up to turn actions
- Medium file size
- Cannot view river results

### River Storage Mode
- Stores complete game tree
- Largest file size
- Full access to all streets

**Example**: Truncating Storage Mode
```rust
// After solving, truncate to turn storage
game.set_target_storage_mode(BoardState::Turn).unwrap();
save_data_to_file(&game, "memo", "game_turn.flop", None).unwrap();
```

---

## Quick Start Workflow

### For Testing JSON Viewer (2 minutes)

1. **Use the existing sample file**:
   ```bash
   npm run tauri dev
   ```
2. Click "**Load**"
3. Navigate to: `src-tauri/postflop-solver/solution.flop`
4. Select and open
5. JSON Template appears automatically!

### For Creating Your Own Files (15+ minutes)

1. **Run Desktop Postflop**:
   ```bash
   npm run tauri dev
   ```

2. **Configure Game** (in sidebar):
   - OOP Range: `AA-22,AKs-A2s,AKo-A2o` (example)
   - IP Range: `AA-22,AKs-A2s,AKo-A7o` (example)
   - Board: `Kh Qh 7c` (click cards to select)

3. **Configure Tree** (in sidebar):
   - Starting Pot: `60`
   - Effective Stack: `940`
   - Bet Sizes: `33%, 75%, 150%` (example)

4. **Run Solver**:
   - Max Iterations: `1000` (quick test) or `10000` (accurate)
   - Target Exploitability: `0.5`
   - Click "**Start**"
   - Wait for completion (progress bar shows status)

5. **Save File**:
   - Click "**Save**" in navbar
   - Enter memo: "My first solver run"
   - Choose location: Desktop or Documents
   - Filename: `my_game.flop`
   - Click Save

6. **View JSON**:
   - JSON Template already open (automatic)
   - Or click "**JSON Template**" in sidebar

---

## Troubleshooting

### "File not found" Error
- Verify file path is correct
- Check file has `.flop` or `.bin` extension
- Make sure file wasn't moved or deleted

### "Failed to load file" Error
- File may be corrupted
- Incompatible version (old solver version)
- Try loading a known-good sample file first

### "No sample files available"
- Build the postflop-solver examples:
  ```bash
  cd src-tauri/postflop-solver
  cargo run --release --example file_io
  ```

### Solver takes too long
- Reduce max iterations (e.g., 100 instead of 1000)
- Use simpler board texture
- Tighten ranges (fewer combos)
- Reduce bet sizing options

### File too large
- Use storage mode truncation:
  - Flop mode: Smallest
  - Turn mode: Medium
  - River mode: Largest
- Enable compression (requires rebuild with `zstd` feature)
- Simplify game tree (fewer bet sizes)

---

## File Location Recommendations

### During Development
```
project_root/
├── test_files/
│   ├── sample_flop.flop
│   ├── sample_turn.flop
│   └── sample_river.flop
└── src-tauri/postflop-solver/
    └── solution.flop  (existing sample)
```

### For Production Use
```
User Documents/
└── PokerSolver/
    ├── Cash_Games/
    │   ├── BTN_vs_BB_Kh7h2c.flop
    │   └── SB_vs_BB_AsKd3s.flop
    └── Tournaments/
        └── 20bb_BTN_vs_BB.flop
```

---

## Example Ranges

### Preflop Standard Ranges

**BTN (Button) Open Range**:
```
22+,A2s+,K2s+,Q2s+,J6s+,T6s+,96s+,86s+,75s+,65s,54s,
A2o+,K9o+,Q9o+,J9o+,T9o
```

**BB (Big Blind) vs BTN Defense**:
```
22+,A2s+,K2s+,Q2s+,J5s+,T6s+,96s+,86s+,75s+,65s,54s,
A2o+,K5o+,Q8o+,J8o+,T8o+,98o
```

**Single Raised Pot (SRP)**:
```
OOP: 66+,A8s+,A5s-A4s,AJo+,K9s+,KQo,QTs+,JTs,96s+,85s+,75s+,65s,54s
IP:  QQ-22,AQs-A2s,ATo+,K5s+,KJo+,Q8s+,J8s+,T7s+,96s+,86s+,75s+,64s+,53s+
```

### Common Board Textures

**High Dry**:
- `AhKc7s`, `AsKd3h`, `KhQc4d`

**High Connected**:
- `KsQhJc`, `QdJh9s`, `JhTs8c`

**Middle Paired**:
- `8h8c3s`, `9s9d2h`, `7d7c4s`

**Low Monotone**:
- `6h5h3h`, `7c4c2c`, `8s5s2s`

---

## Resources

### Documentation
- [Desktop Postflop README](./README.md)
- [postflop-solver Documentation](https://b-inary.github.io/postflop_solver/postflop_solver/)
- [JSON Viewer Guide](./JSON_VIEWER_GUIDE.md)

### Examples
- [Basic Example](./src-tauri/postflop-solver/examples/basic.rs)
- [File I/O Example](./src-tauri/postflop-solver/examples/file_io.rs)
- [Backend Solver Example](./src-tauri/postflop-solver/examples/backend_solver.rs)

### Community
- [GitHub Issues](https://github.com/b-inary/desktop-postflop/issues)
- [postflop-solver Repository](https://github.com/b-inary/postflop-solver)
- [WASM Postflop (Web Version)](https://github.com/b-inary/wasm-postflop)

---

## Summary

### Quickest Path to Testing
1. ✅ Use existing sample: `src-tauri/postflop-solver/solution.flop`
2. ✅ Load it in the app
3. ✅ View JSON Template automatically

### Best Practice for Regular Use
1. 🎮 Use Desktop Postflop GUI to configure games
2. ⚡ Run solver with appropriate iterations
3. 💾 Save with descriptive memo
4. 📊 View JSON to verify configuration
5. 🔄 Iterate and refine

### For Advanced Users
1. 🦀 Write Rust code using postflop-solver library
2. 🔧 Automate game generation
3. 📦 Batch process multiple scenarios
4. 🔍 Analyze results programmatically

---

**Last Updated**: 2026-02-04
**Version**: 1.0
