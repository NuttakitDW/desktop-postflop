---
description: Load .flop or .bin solver file and display JSON template on UI
---

# Workflow: Load Solver File and Display JSON

This workflow describes how to load a `.flop` or `.bin` solver file, extract metadata, and display it as JSON on the UI.

## Prerequisites

- Desktop Postflop app running (`npm run tauri dev`)
- A valid `.flop` or `.bin` solver file

## Test File Location

There is a test file available at:
```
/Users/sumbenz/workspace/desktop-postflop/src-tauri/postflop-solver/solution.flop
```

## Steps

### Step 1: Start the Application
// turbo
```bash
cd /Users/sumbenz/workspace/desktop-postflop && npm run tauri dev
```

### Step 2: Load the Solver File

1. Click the **"Load"** button in the NavBar (top left area)
2. In the file dialog, navigate to and select a `.flop` or `.bin` file
3. Wait for the file to load (progress will be shown)

### Step 3: View Loaded Data

After successful loading:
- The app will navigate to the **Results** view
- The **storage mode** (flop/turn/river) will be displayed in the NavBar
- The board cards, pot size, and stack size will be updated

### Step 4: Export JSON Preview

1. After loading the file, click the **"Export JSON"** button (blue button in NavBar)
2. Choose a location to save the JSON file
3. The JSON file will contain metadata like:

```json
{
  "memo": "Game description",
  "is_solved": true,
  "storage_mode": "flop",
  "board": [48, 44, 20],
  "starting_pot": 60,
  "effective_stack": 940,
  "exported_at": "2026-02-04T14:00:00.000Z",
  "note": "Metadata only, not full game state"
}
```

## Data Structure

### GameLoadResponse (from Rust backend)

| Field | Type | Description |
|-------|------|-------------|
| `memo` | String | User-defined memo for the game |
| `is_solved` | Boolean | Whether the game has been solved |
| `storage_mode` | String | "flop", "turn", or "river" |
| `board` | Array<u8> | Card IDs on the board (0-51) |
| `starting_pot` | i32 | Starting pot size |
| `effective_stack` | i32 | Effective stack size |
| `private_cards` | Array<Array<u16>> | Private cards for each player |

### Card Encoding

Cards are encoded as numbers 0-51:
- **Formula**: `rank * 4 + suit`
- **Ranks**: 0=2, 1=3, ..., 9=T, 10=J, 11=Q, 12=K, 13=A
- **Suits**: 0=♣ (Club), 1=♦ (Diamond), 2=♥ (Heart), 3=♠ (Spade)

**Examples**:
- 48 = K♥ (King of Hearts)
- 44 = Q♥ (Queen of Hearts)  
- 20 = 7♣ (7 of Clubs)

## Code Flow

### 1. User clicks "Load" button
**File**: `src/components/NavBar.vue`
```typescript
const loadFile = async () => {
  const path = await open({
    filters: [{ name: "Postflop", extensions: ["flop", "bin"] }],
  });
  const result = await gameLoadFile(path);
  // Update store with loaded data
}
```

### 2. Frontend calls Rust backend
**File**: `src/invokes.ts`
```typescript
export const gameLoadFile = async (path: string): Promise<GameLoadResponse> => {
  return await invoke("game_load_file", { path });
};
```

### 3. Rust loads and deserializes file
**File**: `src-tauri/src/solver.rs`
```rust
pub fn game_load_file(...) -> Result<GameLoadResponse, String> {
  let (loaded_game, memo) = load_data_from_file(&path, None)?;
  // Extract metadata and return
}
```

### 4. UI updates with loaded data
- Board cards displayed
- Pot/stack sizes updated
- Navigation to Results view
- "Export JSON" button becomes visible

## Troubleshooting

### Error: "Failed to load file"
- Check if the file exists and is a valid `.flop` or `.bin` file
- Ensure the file was created with a compatible version of postflop-solver

### Error: "Version mismatch"
- The file format version doesn't match the solver version
- Try updating the solver or using a compatible file

### Export JSON button not showing
- Make sure a file is loaded successfully first
- Check that `store.isFileLoaded` is `true`

## Related Files

- `src/components/NavBar.vue` - UI for Load/Export buttons
- `src/invokes.ts` - TypeScript wrappers for Rust commands
- `src-tauri/src/solver.rs` - Rust file loading implementation
- `src-tauri/postflop-solver/src/file.rs` - Core file format handling
