# JSON Viewer Workflow Guide

## Overview

The JSON Viewer feature provides an intuitive interface to view, copy, and export metadata from `.flop` and `.bin` solver files directly within the Desktop Postflop application.

## Features

### 1. **Automatic Loading**
- When you load a `.flop` or `.bin` file, the application automatically:
  - Switches to the Solver view
  - Opens the JSON Template sidebar
  - Displays the file metadata in JSON format

### 2. **File Information Summary**
The viewer displays a quick summary card showing:
- **Storage Mode**: flop/turn/river
- **Solved Status**: Whether the game has been solved
- **Starting Pot**: Initial pot size
- **Effective Stack**: Stack size
- **Board**: Decoded board cards (e.g., "Kh Qh 7c")
- **Memo**: Game description

### 3. **JSON Display**
- Full JSON structure with proper formatting
- Includes metadata and decoded reference information
- Dark-themed code display for easy reading
- Scrollable container for large JSON objects

### 4. **Actions**
- **Copy JSON**: Copy the entire JSON to clipboard (shows success notification)
- **Export JSON**: Save the JSON to a file

## Workflow

### Step 1: Load a Solver File
1. Click the "**Load**" button in the navbar
2. Select a `.flop` or `.bin` file from your filesystem
3. Wait for the loading indicator to complete

### Step 2: View JSON Template
The application automatically:
- Switches to the "**JSON Template**" view
- Displays the file information summary
- Shows the complete JSON structure

### Step 3: Work with JSON Data

#### Copy to Clipboard
1. Click "**Copy JSON**" button
2. A success message appears: "JSON copied to clipboard!"
3. Paste the JSON anywhere you need

#### Export to File
1. Click "**Export JSON**" button
2. Choose save location and filename
3. JSON file is saved with proper formatting

### Step 4: Access JSON Later
- The "**JSON Template**" button in the sidebar shows a green "**Loaded**" badge when a file is loaded
- Click "**JSON Template**" anytime to view the JSON again

## JSON Structure

```json
{
  "version": 1,
  "memo": "Game description",
  "is_solved": true,
  "storage_mode": "flop",
  "board": [48, 44, 20],
  "starting_pot": 60,
  "effective_stack": 940,
  "exported_at": "2026-02-04T10:00:00.000Z",
  "note": "This is a JSON preview of the binary game file. It contains metadata only, not the full solver state.",
  "_decoded_info_for_reference": {
    "board_readable": "Kh Qh 7c",
    "encoding_formula": "rank * 4 + suit",
    "ranks": "0=2, 1=3, ..., 9=T, 10=J, 11=Q, 12=A",
    "suits": "0=♣ (Club), 1=♦ (Diamond), 2=♥ (Heart), 3=♠ (Spade)"
  }
}
```

## Field Descriptions

| Field | Type | Description |
|-------|------|-------------|
| `version` | number | JSON template version |
| `memo` | string | Game description/notes |
| `is_solved` | boolean | Whether the game has been solved |
| `storage_mode` | string | Storage mode: "flop", "turn", or "river" |
| `board` | number[] | Board cards as encoded numbers (0-51) |
| `starting_pot` | number | Initial pot size |
| `effective_stack` | number | Effective stack size |
| `exported_at` | string | ISO 8601 timestamp of export |
| `note` | string | Important notice about JSON limitations |
| `_decoded_info_for_reference` | object | Helper information for decoding cards |

## Card Encoding

Cards are encoded as numbers from 0-51 using the formula:
```
card_number = rank * 4 + suit
```

### Ranks (0-12)
- 0=2, 1=3, 2=4, 3=5, 4=6, 5=7, 6=8, 7=9, 8=T, 9=J, 10=Q, 11=K, 12=A

### Suits (0-3)
- 0=♣ (Clubs), 1=♦ (Diamonds), 2=♥ (Hearts), 3=♠ (Spades)

### Examples
- Kh (King of Hearts) = 11 * 4 + 2 = 46
- Qh (Queen of Hearts) = 10 * 4 + 2 = 42
- 7c (Seven of Clubs) = 5 * 4 + 0 = 20

## Use Cases

### 1. Quick File Inspection
View metadata without fully loading the solver interface

### 2. Documentation
Export JSON for documenting game configurations

### 3. Debugging
Verify file contents and configurations

### 4. Data Analysis
Extract metadata for batch processing or analysis scripts

### 5. Sharing Configuration
Share game settings with other users via JSON

## UI Components

### Location
**Sidebar Navigation** → "**JSON Template**"

### Visual Indicators
- **Green "Loaded" badge**: Appears when a file is loaded
- **Blue summary card**: Shows key information at a glance
- **Dark code display**: JSON with syntax highlighting
- **Action buttons**: Blue (Copy) and Green (Export)

## Important Notes

⚠️ **Metadata Only**
- This JSON contains metadata only
- Does NOT include:
  - Strategy data
  - Equity/EV calculations
  - Game tree
  - Hand ranges
  - Solver results

💾 **Full Game State**
- To save complete game data, use the "**Save**" button in the navbar
- This creates a `.flop` file with full solver state

## Technical Details

### Components
- **JsonViewer.vue**: Main viewer component
- **NavBar.vue**: File loading and navigation
- **SideBar.vue**: Sidebar button with status indicator
- **App.vue**: Component integration

### Store Updates
- Added `json-viewer` to `SideView` type
- Added "JSON Template" header
- Automatic view switching on file load

### API Calls
- `gameInfo()`: Fetches metadata from loaded game
- Reads from `store.loadedFileMemo` for memo field

## Troubleshooting

### JSON Template button not visible
- Check if you're in the Solver view (not Results)
- Button is always visible, but shows "Loaded" badge when file is loaded

### JSON shows "No file loaded"
- Load a `.flop` or `.bin` file first
- Click "Load" button in navbar

### Copy/Export not working
- Check browser permissions for clipboard access
- Verify file system write permissions for export
- Try refreshing the application

### Decoded board not showing correctly
- Verify the loaded file is valid
- Check that board array contains valid card numbers (0-51)

## Future Enhancements

Potential improvements:
- [ ] Add tree configuration to JSON output
- [ ] Include range information
- [ ] Add filtering/search in JSON viewer
- [ ] Syntax highlighting for JSON
- [ ] Collapsible JSON sections
- [ ] Diff comparison between two JSON files
- [ ] CLI tool for batch JSON export

## Related Documentation

- [JSON Export Guide](./JSON_EXPORT_GUIDE.md) - Original export functionality
- [Changes Summary](./CHANGES_SUMMARY.md) - Recent project changes
- [README](./README.md) - Main project documentation

---

**Version**: 1.0.0
**Last Updated**: 2026-02-04
**Author**: Desktop Postflop Development Team
