# Makefile JSON Export Guide

This guide shows how to use the Makefile commands to export complete JSON data from .flop binary files without using the UI.

## Available Commands

### 1. Export game2.flop (Quick Command)

```bash
cd src-tauri/postflop-solver
make export-json-game2
```

**Output:**
- Creates: `game2-complete.json` (330 MB)
- Displays: Sample hand data with Combos, Equity, EV, and EQR

**Example Output:**
```
=== Sample Hand Data ===
{
  "oop_hands": 27,
  "ip_hands": 81,
  "sample_oop": {
    "combo": [40, 41],
    "equity": 0.6089447140693665,    // 60.89%
    "ev": 36.666114807128906,        // +36.67 chips
    "eqr": 60.21255111694336         // 60.21%
  },
  "sample_ip": {
    "combo": [0, 1],
    "equity": 0.8416385650634766,    // 84.16%
    "ev": 46.870628356933594,        // +46.87 chips
    "eqr": 55.689735412597656        // 55.69%
  }
}
```

---

### 2. Export Any .flop File

```bash
cd src-tauri/postflop-solver
make export-json FLOP_FILE=game.flop OUTPUT_JSON=output.json
```

**Parameters:**
- `FLOP_FILE`: Input .flop binary file (default: `game.flop`)
- `OUTPUT_JSON`: Output JSON file (default: `game-export.json`)

**Examples:**
```bash
# Export game.flop to game-export.json
make export-json

# Export custom file
make export-json FLOP_FILE=my-game.flop OUTPUT_JSON=my-output.json

# Export to specific location
make export-json FLOP_FILE=game2.flop OUTPUT_JSON=/tmp/export.json
```

---

### 3. Export to Console (Preview)

```bash
cd src-tauri/postflop-solver
make export-json-console FLOP_FILE=game2.flop
```

**Output:** Displays JSON structure and sample data in the console without creating a large file.

**Example Output:**
```json
{
  "metadata": {
    "version": 1,
    "is_solved": true,
    "storage_mode": "river"
  },
  "hand_counts": {
    "oop": 27,
    "ip": 81
  },
  "fields": [
    "ip_eqr", "ip_equity", "ip_ev",
    "ip_initial_weights", "ip_private_cards",
    "oop_eqr", "oop_equity", "oop_ev",
    "oop_initial_weights", "oop_private_cards"
  ],
  "sample_hands": {
    "oop": [
      {
        "combo": [40, 41],
        "equity": 0.6089,
        "ev": 36.67,
        "eqr": 60.21
      }
    ]
  }
}
```

---

## Direct Command Line Usage

You can also run the export tool directly without Make:

```bash
cd src-tauri/postflop-solver

# Build the export tool
cargo build --release --features "bincode rayon zstd json-export" --example export_json

# Run it
./target/release/examples/export_json game2.flop output.json
```

**Full Output Example:**
```
╔══════════════════════════════════════════════════════════════╗
║           PostFlop Solver - Complete JSON Export            ║
╚══════════════════════════════════════════════════════════════╝

Input file:  game2.flop
Output file: output.json

⏳ Loading game file... ✓ Done (0.04s)

Game Information:
  Memo:         (empty)
  Is solved:    ✓ Yes
  Storage mode: River
  Board:        47 17 2
  OOP hands:    27
  IP hands:     81
  Total nodes:  121489

⏳ Caching normalized weights... ✓ Done (0.00s)

Sample Hand Data:
  OOP Sample:
    Hand 0: [40, 41] | Equity: 60.89% | EV: 36.67 | EQR: 60.21%
    Hand 1: [40, 42] | Equity: 60.89% | EV: 36.67 | EQR: 60.21%
    Hand 2: [40, 43] | Equity: 60.88% | EV: 36.66 | EQR: 60.22%
  IP Sample:
    Hand 0: [0, 1] | Equity: 84.16% | EV: 46.87 | EQR: 55.69%
    Hand 1: [0, 3] | Equity: 84.16% | EV: 46.85 | EQR: 55.67%
    Hand 2: [1, 3] | Equity: 84.16% | EV: 46.87 | EQR: 55.69%

⏳ Exporting to JSON... ✓ Done (0.82s)
⏳ Writing to file... ✓ Done (113.96s)

✓ Export completed successfully!
  Output file: output.json
  File size:   330.20 MB
```

---

## JSON Structure

The exported JSON includes:

### Metadata
```json
{
  "version": 1,
  "exported_at": "2026-02-30T16:34:47Z",
  "is_solved": true,
  "storage_mode": "river",
  "compression_enabled": false
}
```

### Configuration
```json
{
  "configuration": {
    "card_config": {
      "flop": [47, 17, 2],
      "turn": null,
      "river": null,
      "oop_range": [...],
      "ip_range": [...]
    },
    "tree_config": {
      "starting_pot": 60,
      "effective_stack": 940,
      ...
    }
  }
}
```

### Hand Data (NEW!)
```json
{
  "hand_data": {
    "oop_private_cards": [[40, 41], [40, 42], ...],
    "ip_private_cards": [[0, 1], [0, 3], ...],
    "oop_initial_weights": [1.0, 1.0, ...],
    "ip_initial_weights": [1.0, 1.0, ...],
    "oop_equity": [0.6089, 0.6089, ...],
    "ip_equity": [0.8416, 0.8416, ...],
    "oop_ev": [36.67, 36.67, ...],
    "ip_ev": [46.87, 46.85, ...],
    "oop_eqr": [60.21, 60.21, ...],
    "ip_eqr": [55.69, 55.67, ...]
  }
}
```

### Nodes
```json
{
  "nodes": [
    {
      "index": 0,
      "prev_action": "None",
      "player": 0,
      "turn": null,
      "river": null,
      "amount": 0,
      "children": [1, 2, 3],
      "is_terminal": false,
      "is_chance": false,
      "strategy": [...],
      "cfvalues": [...],
      "cfvalues_ip": [...]
    }
  ]
}
```

---

## Analyzing the JSON Output

### Using jq (JSON processor)

**1. Show hand data structure:**
```bash
jq '.hand_data | keys' game2-complete.json
```

**2. Count hands:**
```bash
jq '{oop: (.hand_data.oop_private_cards | length), ip: (.hand_data.ip_private_cards | length)}' game2-complete.json
```

**3. Show first 5 OOP hands with all stats:**
```bash
jq '[range(0;5)] | map({
  hand: .,
  combo: .hand_data.oop_private_cards[.],
  equity: .hand_data.oop_equity[.],
  ev: .hand_data.oop_ev[.],
  eqr: .hand_data.oop_eqr[.]
})' game2-complete.json
```

**4. Calculate average equity:**
```bash
jq '{
  oop_avg_equity: (.hand_data.oop_equity | add / length),
  ip_avg_equity: (.hand_data.ip_equity | add / length)
}' game2-complete.json
```

**5. Find best EV hands:**
```bash
jq '.hand_data | {
  best_oop_ev: (.oop_ev | max),
  best_ip_ev: (.ip_ev | max)
}' game2-complete.json
```

---

## Performance Notes

- **Loading time**: 0.04s for game2.flop (26 MB)
- **Export time**: 0.82s for JSON generation
- **Write time**: ~114s for 330 MB file
- **Total time**: ~2 minutes for complete export

### File Sizes

| Game Type | Binary (.flop) | JSON Export |
|-----------|----------------|-------------|
| game.flop | 9 MB | 114 MB |
| game2.flop | 26 MB | 330 MB |

**Tip:** For very large files (>500 MB JSON), consider using the console preview first.

---

## Troubleshooting

### Error: "Failed to load game file"
- Check that the .flop file exists
- Ensure it's a valid binary format
- Try with a different file

### Error: "Failed to create output file"
- Check disk space (JSON files are 10-15x larger than binary)
- Ensure you have write permissions
- Try writing to /tmp first

### Large file taking too long
- Use `export-json-console` for quick preview
- Export only specific data using jq after creation
- Consider compressing the JSON: `gzip game2-complete.json`

---

## Additional Commands

### Compress JSON output
```bash
make export-json-game2
gzip game2-complete.json
# Creates game2-complete.json.gz (~20-30x smaller)
```

### Extract specific data
```bash
# Extract only hand data
jq '.hand_data' game2-complete.json > hand-data-only.json

# Extract only OOP data
jq '{oop_cards: .hand_data.oop_private_cards, oop_equity: .hand_data.oop_equity, oop_ev: .hand_data.oop_ev, oop_eqr: .hand_data.oop_eqr}' game2-complete.json > oop-only.json
```

---

## Summary

The Makefile provides three simple commands:

1. **`make export-json-game2`** - Quick export of game2.flop
2. **`make export-json FLOP_FILE=X OUTPUT_JSON=Y`** - Export any file
3. **`make export-json-console FLOP_FILE=X`** - Preview in console

All exports include complete data with **Combos, Equity, EV, and EQR** for every hand! 🎉
