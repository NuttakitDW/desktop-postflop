# Complete JSON Export with Combos, Equity, EV, and EQR

## Implementation Summary

The JSON export has been enhanced to include detailed hand statistics for both OOP and IP players:
- **Combos**: Hand combinations (private cards)
- **Equity**: Equity value for each hand
- **EV**: Expected value for each hand
- **EQR**: Equity Realization (calculated as EV / Equity)

## Changes Made

### 1. Modified Files

#### [src-tauri/postflop-solver/src/game/json_export.rs](src-tauri/postflop-solver/src/game/json_export.rs)
- Added equity, EV, and EQR calculation in `to_json_value_internal()` function
- Calculations are done in parallel using rayon when available
- Only computed if the game is solved and normalized weights are cached
- EQR is calculated as `EV / Equity` with division by zero protection

#### [src-tauri/src/solver.rs](src-tauri/src/solver.rs)
- Updated `game_export_json()` to cache normalized weights before export
- Updated `game_export_json_file()` to cache normalized weights before export
- Ensured weights are only cached if the game is solved

### 2. JSON Structure

The exported JSON now includes the following structure in the `hand_data` section:

```json
{
  "hand_data": {
    "oop_private_cards": [[card1, card2], ...],
    "ip_private_cards": [[card1, card2], ...],
    "oop_initial_weights": [1.0, ...],
    "ip_initial_weights": [1.0, ...],
    "oop_equity": [0.85065, ...],
    "ip_equity": [0.08531, ...],
    "oop_ev": [55.56188, ...],
    "ip_ev": [0.75773, ...],
    "oop_eqr": [65.31684, ...],
    "ip_eqr": [8.88195, ...]
  }
}
```

## Example Hand Data

### OOP (Out of Position) Hand Example
```json
{
  "hand_index": 0,
  "combo": [40, 41],        // Card IDs
  "weight": 1.0,
  "equity": 0.8506516814231873,  // 85.07%
  "ev": 55.561885833740234,      // Expected Value in chips
  "eqr": 65.31684875488281       // Equity Realization %
}
```

### IP (In Position) Hand Example
```json
{
  "hand_index": 0,
  "combo": [32, 33],        // Card IDs
  "weight": 1.0,
  "equity": 0.08531144261360168, // 8.53%
  "ev": 0.7577323913574219,      // Expected Value in chips
  "eqr": 8.88195514678955        // Equity Realization %
}
```

## How to Use

### From Rust Code

```rust
use postflop_solver::*;

// Load a solved game file
let (mut game, memo): (PostFlopGame, String) =
    load_data_from_file("game.flop", None)?;

// Cache normalized weights (required for equity/EV calculations)
if game.is_solved() {
    game.cache_normalized_weights();
}

// Export to JSON
let json_value = game.to_json_value()?;

// Write to file
let file = std::fs::File::create("output.json")?;
serde_json::to_writer_pretty(file, &json_value)?;
```

### From Tauri Frontend

```typescript
import { invoke } from '@tauri-apps/api/tauri'

// Export to JSON file
await invoke('game_export_json_file', { path: 'output.json' })

// Or get JSON as string
const jsonString = await invoke('game_export_json')
const data = JSON.parse(jsonString)

// Access hand data
const oopHands = data.hand_data.oop_private_cards
const oopEquity = data.hand_data.oop_equity
const oopEV = data.hand_data.oop_ev
const oopEQR = data.hand_data.oop_eqr
```

## Interpreting the Data

### Equity
- Range: 0.0 to 1.0
- Represents the probability of winning at showdown
- Example: 0.85 = 85% equity

### Expected Value (EV)
- Measured in chips
- Positive EV = profitable
- Negative EV = unprofitable
- Example: 55.56 = Expected to win 55.56 chips

### Equity Realization (EQR)
- Calculated as: (EV / Equity) * 100
- Represents how well a hand realizes its equity
- 100% = perfect equity realization
- < 100% = hand is not realizing full equity (common for OOP)
- > 100% = hand realizes more than raw equity (rare)

## Performance Considerations

- The JSON export uses parallel processing when the `rayon` feature is enabled
- For large game trees (>100MB), use `game_export_json_file()` to write directly to disk
- Normalized weights caching is required but only done once per export

## Testing

Run the test example:
```bash
cd src-tauri/postflop-solver
cargo run --example test_json_export --features json-export
```

This will:
1. Load `game.flop`
2. Cache normalized weights
3. Export to `game-complete-test.json`
4. Display sample hand data

## File Size

The complete JSON export includes:
- All node data (strategy, cfvalues)
- All hand combinations
- Equity, EV, and EQR for each hand
- Configuration and metadata

Expected file sizes:
- Flop game: 50-500 MB
- Turn game: 200-2000 MB
- River game: 100-1000 MB

The exact size depends on:
- Number of hands in ranges
- Game tree complexity
- Number of streets solved
