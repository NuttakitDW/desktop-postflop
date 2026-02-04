# JSON Viewer Workflow Diagram

## System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     Desktop Postflop UI                      │
│                                                               │
│  ┌────────────┐  ┌────────────┐  ┌────────────┐            │
│  │  NavBar    │  │  SideBar   │  │  App.vue   │            │
│  │            │  │            │  │            │            │
│  │ [Load Btn] │  │ [JSON Btn] │  │ <JsonView> │            │
│  └────────────┘  └────────────┘  └────────────┘            │
│         │               │                │                   │
│         └───────────────┴────────────────┘                   │
│                         │                                    │
│                    ┌────▼────┐                              │
│                    │  Store  │                              │
│                    │ (Pinia) │                              │
│                    └────┬────┘                              │
└─────────────────────────┼───────────────────────────────────┘
                          │
                    ┌─────▼──────┐
                    │   Tauri    │
                    │  Commands  │
                    └─────┬──────┘
                          │
                    ┌─────▼──────┐
                    │   Rust     │
                    │  Backend   │
                    │ (solver.rs)│
                    └────────────┘
```

## User Flow

```
┌─────────────────────────────────────────────────────────────┐
│                     USER INTERACTION                         │
└─────────────────────────────────────────────────────────────┘
                          │
                          ▼
              ┌───────────────────────┐
              │ Click "Load" Button   │
              └───────────┬───────────┘
                          │
                          ▼
              ┌───────────────────────┐
              │ Select .flop/.bin file│
              └───────────┬───────────┘
                          │
                          ▼
              ┌───────────────────────┐
              │ Loading Indicator     │
              └───────────┬───────────┘
                          │
                          ▼
        ┌─────────────────────────────────────┐
        │ gameLoadFile(path) → Rust Backend   │
        └─────────────────┬───────────────────┘
                          │
                          ▼
        ┌─────────────────────────────────────┐
        │ Update Store:                       │
        │  - isFileLoaded = true              │
        │  - loadedFileMemo = memo            │
        │  - storageMode = mode               │
        │  - config.board = board             │
        └─────────────────┬───────────────────┘
                          │
                          ▼
        ┌─────────────────────────────────────┐
        │ Auto-Navigate:                      │
        │  - navView = "solver"               │
        │  - sideView = "json-viewer"         │
        └─────────────────┬───────────────────┘
                          │
                          ▼
        ┌─────────────────────────────────────┐
        │ JsonViewer Component:               │
        │  1. Watch store.isFileLoaded        │
        │  2. Call gameInfo()                 │
        │  3. Format JSON data                │
        │  4. Decode board cards              │
        │  5. Display in UI                   │
        └─────────────────┬───────────────────┘
                          │
                          ▼
        ┌─────────────────────────────────────┐
        │ User sees:                          │
        │  ✓ File info summary                │
        │  ✓ Formatted JSON                   │
        │  ✓ Copy/Export buttons              │
        └─────────────────────────────────────┘
```

## Component Interaction Flow

```
┌──────────────────────────────────────────────────────────────┐
│                      NavBar.vue                              │
│                                                              │
│  loadFile() {                                               │
│    path = await open()                                      │
│    result = await gameLoadFile(path)  ────────┐            │
│    store.isFileLoaded = true                   │            │
│    store.navView = "solver"                    │            │
│    store.sideView = "json-viewer" ────┐       │            │
│  }                                     │       │            │
└────────────────────────────────────────┼───────┼────────────┘
                                         │       │
                                         │       │
┌────────────────────────────────────────┼───────┼────────────┐
│                    SideBar.vue         │       │            │
│                                        │       │            │
│  <button @click="store.sideView =     │       │            │
│           'json-viewer'">              │       │            │
│    JSON Template                       │       │            │
│    <span v-if="store.isFileLoaded">◄──┘       │            │
│      Loaded                                    │            │
│    </span>                                     │            │
│  </button>                                     │            │
└────────────────────────────────────────────────┼────────────┘
                                                 │
                                                 │
┌────────────────────────────────────────────────┼────────────┐
│                     App.vue                    │            │
│                                                │            │
│  <div v-show="store.sideView ===               │            │
│               'json-viewer'">  ◄───────────────┘            │
│    <JsonViewer />                                           │
│  </div>                                                     │
└────────────────────────────────────────────────┬────────────┘
                                                 │
                                                 │
┌────────────────────────────────────────────────▼────────────┐
│                   JsonViewer.vue                            │
│                                                             │
│  watch(store.isFileLoaded) {  ◄───────────┐                │
│    if (isFileLoaded) {                    │                │
│      info = await gameInfo()  ────────┐   │                │
│      jsonData = {                      │   │                │
│        memo: store.loadedFileMemo,     │   │                │
│        is_solved: info.is_solved,      │   │                │
│        board: info.board,              │   │                │
│        ...                             │   │                │
│      }                                 │   │                │
│    }                                   │   │                │
│  }                                     │   │                │
│                                        │   │                │
│  copyToClipboard() {                   │   │                │
│    navigator.clipboard.writeText(...)  │   │                │
│  }                                     │   │                │
│                                        │   │                │
│  exportToFile() {                      │   │                │
│    path = await save()                 │   │                │
│    await writeTextFile(path, json)     │   │                │
│  }                                     │   │                │
└────────────────────────────────────────┼───┼────────────────┘
                                         │   │
                                         │   │
                        ┌────────────────┘   │
                        │  Tauri API         │
                        │                    │
                        ▼                    ▼
              ┌──────────────────┐  ┌──────────────────┐
              │  gameInfo()      │  │ gameLoadFile()   │
              │  (solver.rs)     │  │ (solver.rs)      │
              └────────┬─────────┘  └────────┬─────────┘
                       │                     │
                       ▼                     ▼
              ┌────────────────────────────────────────┐
              │         Rust Backend                   │
              │  - PostFlopGame state                  │
              │  - Card config                         │
              │  - Tree config                         │
              └────────────────────────────────────────┘
```

## Data Flow

```
┌─────────────────────────────────────────────────────────────┐
│                      .flop / .bin File                       │
│  ┌────────────────────────────────────────────────────┐     │
│  │ Binary data containing:                            │     │
│  │  - Card config (board, ranges)                     │     │
│  │  - Tree config (bet sizes, structure)              │     │
│  │  - Solver state (strategies, EVs)                  │     │
│  │  - Metadata (memo, solved status)                  │     │
│  └────────────────────────────────────────────────────┘     │
└───────────────────────────┬─────────────────────────────────┘
                            │
                            ▼ gameLoadFile()
┌───────────────────────────────────────────────────────────────┐
│                   Rust Backend (solver.rs)                    │
│  ┌──────────────────────────────────────────────────────┐    │
│  │ GameLoadResponse {                                   │    │
│  │   memo: String,                                      │    │
│  │   is_solved: bool,                                   │    │
│  │   storage_mode: String,                              │    │
│  │   board: Vec<u8>,                                    │    │
│  │   starting_pot: i32,                                 │    │
│  │   effective_stack: i32,                              │    │
│  │   private_cards: [Vec<u16>; 2]                       │    │
│  │ }                                                    │    │
│  └──────────────────────────────────────────────────────┘    │
└───────────────────────────┬───────────────────────────────────┘
                            │
                            ▼
┌───────────────────────────────────────────────────────────────┐
│                      Pinia Store (store.ts)                   │
│  ┌──────────────────────────────────────────────────────┐    │
│  │ Store State:                                         │    │
│  │   isFileLoaded: true                                 │    │
│  │   loadedFileMemo: "BTN vs BB..."                     │    │
│  │   storageMode: "flop"                                │    │
│  │   navView: "solver"                                  │    │
│  │   sideView: "json-viewer"                            │    │
│  │                                                      │    │
│  │ Config State:                                        │    │
│  │   board: [48, 44, 20]                                │    │
│  │   startingPot: 60                                    │    │
│  │   effectiveStack: 940                                │    │
│  └──────────────────────────────────────────────────────┘    │
└───────────────────────────┬───────────────────────────────────┘
                            │
                            ▼ gameInfo()
┌───────────────────────────────────────────────────────────────┐
│                   Rust Backend (solver.rs)                    │
│  ┌──────────────────────────────────────────────────────┐    │
│  │ GameInfoResponse {                                   │    │
│  │   is_solved: bool,                                   │    │
│  │   storage_mode: String,                              │    │
│  │   board: Vec<u8>,                                    │    │
│  │   starting_pot: i32,                                 │    │
│  │   effective_stack: i32                               │    │
│  │ }                                                    │    │
│  └──────────────────────────────────────────────────────┘    │
└───────────────────────────┬───────────────────────────────────┘
                            │
                            ▼
┌───────────────────────────────────────────────────────────────┐
│                   JsonViewer Component                        │
│  ┌──────────────────────────────────────────────────────┐    │
│  │ jsonData = {                                         │    │
│  │   version: 1,                                        │    │
│  │   memo: store.loadedFileMemo,                        │    │
│  │   is_solved: info.is_solved,                         │    │
│  │   storage_mode: info.storage_mode,                   │    │
│  │   board: info.board,                                 │    │
│  │   starting_pot: info.starting_pot,                   │    │
│  │   effective_stack: info.effective_stack,             │    │
│  │   exported_at: "2026-02-04T...",                     │    │
│  │   note: "Metadata only...",                          │    │
│  │   _decoded_info_for_reference: {                     │    │
│  │     board_readable: "Kh Qh 7c",                      │    │
│  │     encoding_formula: "rank * 4 + suit",             │    │
│  │     ranks: "0=2, 1=3, ..., 12=A",                    │    │
│  │     suits: "0=♣, 1=♦, 2=♥, 3=♠"                      │    │
│  │   }                                                  │    │
│  │ }                                                    │    │
│  └──────────────────────────────────────────────────────┘    │
└───────────────────────────┬───────────────────────────────────┘
                            │
                            ▼
┌───────────────────────────────────────────────────────────────┐
│                         UI Display                            │
│  ┌──────────────────────────────────────────────────────┐    │
│  │ ╔════════════════════════════════════════════════╗  │    │
│  │ ║          JSON Template                         ║  │    │
│  │ ║  [Copy JSON]  [Export JSON]                    ║  │    │
│  │ ╠════════════════════════════════════════════════╣  │    │
│  │ ║  File Information                              ║  │    │
│  │ ║  Storage Mode: flop      Solved: Yes          ║  │    │
│  │ ║  Starting Pot: 60        Effective Stack: 940  ║  │    │
│  │ ║  Board: Kh Qh 7c                               ║  │    │
│  │ ╠════════════════════════════════════════════════╣  │    │
│  │ ║  {                                             ║  │    │
│  │ ║    "version": 1,                               ║  │    │
│  │ ║    "memo": "BTN vs BB...",                     ║  │    │
│  │ ║    "is_solved": true,                          ║  │    │
│  │ ║    "storage_mode": "flop",                     ║  │    │
│  │ ║    "board": [48, 44, 20],                      ║  │    │
│  │ ║    ...                                         ║  │    │
│  │ ║  }                                             ║  │    │
│  │ ╚════════════════════════════════════════════════╝  │    │
│  └──────────────────────────────────────────────────────┘    │
└───────────────────────────────────────────────────────────────┘
```

## State Management

```
┌─────────────────────────────────────────────────────────────┐
│                    Pinia Store States                        │
└─────────────────────────────────────────────────────────────┘

┌──────────────────┐        ┌──────────────────┐
│   useStore()     │        │ useConfigStore() │
│   (app state)    │        │  (config state)  │
├──────────────────┤        ├──────────────────┤
│ navView          │        │ board            │
│ sideView         │        │ startingPot      │
│ isFileLoaded     │        │ effectiveStack   │
│ isFileLoading    │        │ rakePercent      │
│ loadedFileMemo   │        │ rakeCap          │
│ storageMode      │        │ donkOption       │
│ isSolverFinished │        │ bet sizes...     │
└──────────────────┘        └──────────────────┘

                    ▲
                    │
                    │ Watch
                    │
┌───────────────────┴────────────────┐
│      JsonViewer Component          │
│                                    │
│  watch(() => store.isFileLoaded)   │
│  {                                 │
│    if (isFileLoaded) {             │
│      loadJsonData()                │
│    }                               │
│  }                                 │
└────────────────────────────────────┘
```

## Action Flow

```
USER ACTIONS ──────> COMPONENT METHODS ──────> TAURI APIS

[Load File]  ────>  loadFile()        ────>  gameLoadFile(path)
                    │                         │
                    └─> Update Store          └─> Returns GameLoadResponse
                        │
                        └─> Navigate to JSON view

[View JSON]  ────>  JsonViewer.vue
                    │
                    └─> gameInfo()     ────>  Returns GameInfoResponse
                        │
                        └─> Format & Display

[Copy JSON]  ────>  copyToClipboard() ────>  navigator.clipboard.writeText()
                    │
                    └─> Show success notification

[Export JSON]────>  exportToFile()    ────>  save() → writeTextFile()
                    │
                    └─> Show success alert
```

---

**Diagram Version**: 1.0
**Last Updated**: 2026-02-04
