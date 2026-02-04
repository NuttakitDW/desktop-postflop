# Desktop Postflop - Quick Start Guide

## 🚀 Test JSON Viewer in 2 Minutes

### 1. Run the Application
```bash
npm run tauri dev
```

### 2. Load Sample File
1. Click **"Load"** button (top navbar)
2. Navigate to: `src-tauri/postflop-solver/solution.flop`
3. Click **"Open"**

### 3. View JSON Template
- ✅ Automatically opens JSON Template view
- ✅ See file information summary
- ✅ See formatted JSON data
- ✅ Click **"Copy JSON"** to copy to clipboard
- ✅ Click **"Export JSON"** to save to file

**Done!** You've successfully tested the JSON viewer.

---

## 🎮 Create Your Own Solver File

### Method 1: Using the GUI (Recommended)

#### Step 1: Configure Ranges
In the **Solver** view sidebar:

1. **OOP Range**
   - Click "OOP Range"
   - Enter range: `AA,KK,QQ,AKs` (example)
   - Or use grid to select hands

2. **IP Range**
   - Click "IP Range"
   - Enter range: `AA,KK,QQ,JJ,AKs,AKo` (example)

#### Step 2: Configure Board
1. Click **"Board"**
2. Select 3 cards for flop: `Kh Qh 7c` (example)
3. Optional: Add turn/river cards

#### Step 3: Configure Tree
1. Click **"Tree Configuration"**
2. Set values:
   - Starting Pot: `60`
   - Effective Stack: `940`
   - OOP Flop Bet: `33%, 75%, 150%`
   - IP Flop Bet: `33%, 75%, 150%`

#### Step 4: Run Solver
1. Click **"Run Solver"**
2. Set iterations: `1000` (for quick test)
3. Click **"Start"**
4. Wait for completion (~1-5 minutes)

#### Step 5: Save & View
1. Click **"Save"** button (top navbar)
2. Enter memo: "My first solver run"
3. Save as: `my_game.flop`
4. JSON Template automatically appears!

---

## 📋 Sample Ranges

### Conservative
```
OOP: AA,KK,QQ,AKs
IP:  AA,KK,QQ,JJ,AKs,AKo
```

### Standard SRP (Single Raised Pot)
```
OOP: 66+,A8s+,A5s-A4s,AJo+,K9s+,KQo,QTs+,JTs
IP:  QQ-22,AQs-A2s,ATo+,K5s+,KJo+,Q8s+,J8s+
```

### Wide Ranges
```
OOP: 22+,A2s+,K2s+,Q8s+,J8s+,T8s+,98s,ATo+,KJo+
IP:  22+,A2s+,K2s+,Q2s+,J6s+,T7s+,97s+,A2o+,K9o+
```

---

## 📊 Common Board Textures

### High Boards
- **Dry**: `AhKc7s`, `AsQd3h`, `KhJc4d`
- **Connected**: `KsQhJc`, `QdJh9s`, `JhTs8c`

### Middle Boards
- **Paired**: `8h8c3s`, `9s9d2h`, `7d7c4s`
- **Connected**: `9h8c7d`, `8s7h5c`, `7c6d4h`

### Low Boards
- **Monotone**: `6h5h3h`, `7c4c2c`, `8s5s2s`
- **Dry**: `7s4d2h`, `6c3h2d`, `5h4c2s`

---

## 🎯 Bet Sizing Notation

### Percentage of Pot
- `33%` - One-third pot
- `50%` - Half pot
- `75%` - Three-quarter pot
- `100%` - Pot-sized bet

### Special Notations
- `e` - Geometric sizing (calculated)
- `a` - All-in

### Examples
- `33%, 75%, 150%` - Three bet sizes
- `50%, e, a` - Half pot, geometric, and all-in
- `2x, 3x` - Multiple of previous bet (for raises)

---

## ⚡ Performance Tips

### Fast Testing (1-2 minutes)
- **Iterations**: `100-500`
- **Ranges**: Tight ranges (AA, KK only)
- **Bet Sizes**: 1-2 options only

### Standard Solving (5-15 minutes)
- **Iterations**: `1000-3000`
- **Ranges**: Normal ranges
- **Bet Sizes**: 2-3 options

### High Accuracy (30+ minutes)
- **Iterations**: `10000+`
- **Ranges**: Wide ranges
- **Bet Sizes**: 3+ options
- **Multiple streets**: Turn and river

---

## 🗂️ File Organization

### Recommended Structure
```
Documents/
└── PokerSolver/
    ├── Testing/
    │   └── quick_test.flop
    ├── Cash/
    │   ├── BTN_vs_BB/
    │   │   ├── Khigh_boards.flop
    │   │   └── Qhigh_boards.flop
    │   └── SB_vs_BB/
    └── Tournament/
        └── 20bb_spots.flop
```

---

## 🔍 JSON Template Usage

### After Loading a File

**View in UI**:
1. File info summary appears at top
2. Full JSON below

**Copy to Clipboard**:
1. Click **"Copy JSON"** button
2. Paste into text editor, documentation, etc.

**Export to File**:
1. Click **"Export JSON"** button
2. Choose location
3. Use for documentation, sharing config

### What's in the JSON?
- ✅ Game memo/description
- ✅ Solved status (true/false)
- ✅ Storage mode (flop/turn/river)
- ✅ Board cards (encoded + decoded)
- ✅ Starting pot size
- ✅ Effective stack size
- ✅ Export timestamp
- ❌ Strategy data (not included)
- ❌ Equity/EV results (not included)

---

## 🐛 Common Issues

### "File Loading Failed"
- **Solution**: Verify file is `.flop` or `.bin` format
- Try loading sample file first: `src-tauri/postflop-solver/solution.flop`

### "Solver Taking Forever"
- **Solution**: Reduce iterations (try 100 first)
- Use tighter ranges (fewer hands)
- Reduce bet sizing options

### "JSON Template Shows 'No file loaded'"
- **Solution**: Load a file first using "Load" button
- Make sure loading completed (check for errors)

### "Cannot Find Sample File"
- **Solution**: Check path: `src-tauri/postflop-solver/solution.flop`
- If missing, create one using the GUI workflow above

---

## 📚 Next Steps

### Learn More
- [Full Documentation](./README.md)
- [JSON Viewer Guide](./JSON_VIEWER_GUIDE.md)
- [How to Get .flop Files](./HOW_TO_GET_FLOP_FILES.md)
- [Implementation Details](./JSON_VIEWER_IMPLEMENTATION.md)

### Advanced Topics
- [Rust Examples](./src-tauri/postflop-solver/examples/)
- [API Documentation](https://b-inary.github.io/postflop_solver/postflop_solver/)
- [Custom Scripts](./src-tauri/postflop-solver/README.md)

---

## 💡 Pro Tips

1. **Start Small**: Test with tight ranges and low iterations first
2. **Save Often**: Save configurations you plan to reuse
3. **Descriptive Memos**: Use clear names like "BTN_vs_BB_Khigh_dry"
4. **JSON Export**: Export JSON for record-keeping before making changes
5. **Multiple Files**: Create separate files for different scenarios

---

## ⌨️ Keyboard Shortcuts

### Navigation
- **Solver View**: Click "Solver" button or press sidebar buttons
- **Results View**: Click "Results" button (after solving)

### File Operations
- **Load**: Click "Load" button in navbar
- **Save**: Click "Save" button in navbar (after solving)
- **Export JSON**: Click "Export JSON" button (when file loaded)

---

## 📞 Getting Help

### Resources
- [GitHub Issues](https://github.com/b-inary/desktop-postflop/issues)
- [postflop-solver Repository](https://github.com/b-inary/postflop-solver)
- Project Documentation (all .md files in root)

### Before Asking
1. ✅ Check this Quick Start guide
2. ✅ Try the sample file
3. ✅ Read error messages carefully
4. ✅ Check if build succeeded (`npm run build`)

---

**Version**: 1.0
**Last Updated**: 2026-02-04

Happy Solving! 🎲♠️♥️♣️♦️
