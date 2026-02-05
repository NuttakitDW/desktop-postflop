# Complete JSON Export - Desktop Postflop

## 📋 สารบัญ

1. [ภาพรวม](#ภาพรวม)
2. [โครงสร้างไฟล์ Binary](#โครงสร้างไฟล์-binary)
3. [ฟังก์ชันหลัก](#ฟังก์ชันหลัก)
4. [JSON Schema](#json-schema)
5. [การ Export JSON แบบสมบูรณ์](#การ-export-json-แบบสมบูรณ์)
6. [ตัวอย่างการใช้งาน](#ตัวอย่างการใช้งาน)

---

## ภาพรวม

Desktop Postflop ใช้ไฟล์ Binary (.flop/.bin) ในการเก็บข้อมูล Poker Solver ทั้งหมด โดยปัจจุบัน JSON Export แสดงเฉพาะ Metadata (~0.0003% ของข้อมูล) เท่านั้น

เอกสารนี้อธิบายวิธีการ Export JSON แบบสมบูรณ์ ที่มีข้อมูล Strategy, Equity, EV ครบถ้วน

---

## โครงสร้างไฟล์ Binary

### Binary File Format

ไฟล์แบ่งเป็น 2 ส่วนหลัก:

#### **1. Header (Metadata)**
```rust
// ข้อมูลพื้นฐานของไฟล์ (~500 bytes)
Header {
    magic_number: u32,           // 0x09f15790 (magic ตรวจสอบไฟล์)
    version: u8,                 // 1 (เวอร์ชันไฟล์)
    compression_type: u8,        // 0=none, 1=zstd
    data_type: u8,               // 0=game, 1=bunching
    estimated_memory: VarInt,    // RAM ที่ต้องใช้ (bytes)
    memo: String,                // คำอธิบายเกม
}
```

#### **2. Body (Game Data)**
```rust
PostFlopGame {
    // Card Configuration (~10 KB)
    card_config: CardConfig {
        range: [Range; 2],       // OOP & IP ranges
        flop: [u8; 3],           // Flop cards (3 ใบ)
        turn: u8,                // Turn card (255 = NOT_DEALT)
        river: u8,               // River card
    },

    // Action Tree Configuration (~5 KB)
    action_tree: ActionTree {
        tree_config: TreeConfig {
            initial_state: BoardState,
            starting_pot: i32,
            effective_stack: i32,
            rake_rate: f64,
            rake_cap: f64,
            flop_bet_sizes: [BetSizeOptions; 2],
            turn_bet_sizes: [BetSizeOptions; 2],
            river_bet_sizes: [BetSizeOptions; 2],
            turn_donk_sizes: Option<DonkSizeOptions>,
            river_donk_sizes: Option<DonkSizeOptions>,
            add_allin_threshold: f64,
            force_allin_threshold: f64,
            merging_threshold: f64,
            max_raises_per_street: i32,
        },
        nodes: Vec<ActionNode>,
        added_lines: Vec<Vec<Action>>,
        removed_lines: Vec<Vec<Action>>,
    },

    // Solver Results (~150 MB - ข้อมูลหลัก!)
    num_nodes: Vec<usize>,
    num_hands: [Vec<usize>; 2],
    strategy: Vec<Vec<f32>>,         // กลยุทธ์ (Fold/Check/Bet/Raise %)
    weights: Vec<Vec<f32>>,          // น้ำหนักของแต่ละมือ
    equity: Vec<Vec<f32>>,           // Equity ของแต่ละมือ
    expected_values: Vec<Vec<f32>>,  // EV ของแต่ละมือ

    // State
    is_solved: bool,
    storage_mode: BoardState,
    target_memory_usage: u64,
}
```

### ขนาดข้อมูลเปรียบเทียบ

| Component | Binary Size | JSON Export (ปัจจุบัน) | JSON Export (สมบูรณ์) |
|-----------|-------------|----------------------|---------------------|
| Metadata | ~500 bytes | ✅ 500 bytes | ✅ 500 bytes |
| Card Config | ~10 KB | ❌ ไม่มี | ✅ 10 KB |
| Tree Config | ~5 KB | ❌ ไม่มี | ✅ 5 KB |
| Ranges | ~50 KB | ❌ ไม่มี | ✅ 50 KB |
| Game Tree | ~500 KB | ❌ ไม่มี | ⚠️ ไม่รวม (ใหญ่เกิน) |
| Strategy | ~50 MB | ❌ ไม่มี | ✅ 50 MB |
| Equity/EV | ~100 MB | ❌ ไม่มี | ✅ 100 MB |
| **Total** | **~150 MB** | **~500 bytes** | **~150 MB** |

---

## ฟังก์ชันหลัก

### 1. Load File

#### Frontend (TypeScript)
```typescript
// src/invokes.ts:385-387
export const gameLoadFile = async (path: string): Promise<GameLoadResponse> => {
  return await invoke("game_load_file", { path });
};

// ตัวอย่างการใช้
const result = await gameLoadFile("/path/to/game.flop");
console.log(result.memo);              // "BTN vs BB - Kh Qh 7c"
console.log(result.is_solved);         // true
console.log(result.storage_mode);      // "flop"
console.log(result.board);             // [46, 42, 20]
console.log(result.starting_pot);      // 60
console.log(result.effective_stack);   // 940
```

#### Backend (Rust)
```rust
// src-tauri/src/solver.rs:613-684
#[tauri::command(async)]
pub fn game_load_file(
    game_state: tauri::State<Mutex<PostFlopGame>>,
    path: String,
) -> Result<GameLoadResponse, String> {
    // 1. โหลด Binary ทั้งหมด
    let (loaded_game, memo): (PostFlopGame, String) =
        load_data_from_file(&path, None)?;

    // 2. Extract metadata
    let is_solved = loaded_game.is_solved();
    let storage_mode = board_state_to_string(loaded_game.storage_mode());

    // 3. Extract board cards
    let card_config = loaded_game.card_config();
    let mut board = card_config.flop.to_vec();
    if card_config.turn != NOT_DEALT {
        board.push(card_config.turn);
    }
    if card_config.river != NOT_DEALT {
        board.push(card_config.river);
    }

    // 4. Extract tree config
    let tree_config = loaded_game.tree_config();
    let starting_pot = tree_config.starting_pot;
    let effective_stack = tree_config.effective_stack;

    // 5. เก็บ Game State ใน Memory
    *game_state.lock().unwrap() = loaded_game;

    // 6. Return response
    Ok(GameLoadResponse {
        memo,
        is_solved,
        storage_mode,
        board,
        starting_pot,
        effective_stack,
        private_cards,
    })
}
```

### 2. Run Solver

#### Frontend (TypeScript)
```typescript
// src/invokes.ts:270-275 - Solve ทีเดียว
export const gameSolve = async (
  maxIterations: number,
  targetExploitability: number
): Promise<SolveResult> => {
  return await invoke("game_solve", { maxIterations, targetExploitability });
};

// ตัวอย่างการใช้
const result = await gameSolve(1000, 0.5);
console.log(`Iterations: ${result.iterations}`);
console.log(`Exploitability: ${result.exploitability}`);
console.log(`Time: ${result.time_ms}ms`);
```

```typescript
// src/invokes.ts:260-262 - Solve ทีละ Step (สำหรับ Progress Bar)
export const gameSolveStep = async (currentIteration: number) => {
  await invoke("game_solve_step", { currentIteration });
};

// ตัวอย่างการใช้
for (let i = 0; i < maxIterations; i++) {
  await gameSolveStep(i);
  updateProgressBar((i + 1) / maxIterations);
}
await gameFinalize();
```

#### Backend (Rust)
```rust
// src-tauri/src/solver.rs:263-288
#[tauri::command(async)]
pub fn game_solve(
    game_state: tauri::State<Mutex<PostFlopGame>>,
    pool_state: tauri::State<Mutex<ThreadPool>>,
    max_iterations: u32,
    target_exploitability: f32,
) -> SolveResult {
    let start = std::time::Instant::now();
    let pool = pool_state.lock().unwrap();

    // Run solver ใน thread pool
    let exploitability = pool.install(|| {
        let mut game = game_state.lock().unwrap();
        solve(&mut *game, max_iterations, target_exploitability, true)
    });

    let time_ms = start.elapsed().as_millis() as u64;

    SolveResult {
        iterations: max_iterations,
        exploitability,
        time_ms,
    }
}
```

### 3. Get Results

#### Frontend (TypeScript)
```typescript
// src/invokes.ts:317-332
export const gameGetResults = async (): Promise<Results> => {
  const results: ResultsResponse = await invoke("game_get_results");
  return {
    currentPlayer: results.current_player,
    numActions: results.num_actions,
    isEmpty: results.is_empty,
    eqrBase: results.eqr_base,
    weights: results.weights,
    normalizer: results.normalizer,
    equity: results.equity,
    ev: results.ev,
    eqr: results.eqr,
    strategy: results.strategy,
    actionEv: results.action_ev,
  };
};

// ตัวอย่างการใช้
const results = await gameGetResults();
console.log("Current Player:", results.currentPlayer);
console.log("OOP Equity:", results.equity[0]);
console.log("Strategy:", results.strategy);
```

#### Backend (Rust)
```rust
// src-tauri/src/solver.rs:420-500
#[tauri::command]
pub fn game_get_results(game_state: tauri::State<Mutex<PostFlopGame>>) -> GameResultsResponse {
    let mut game = game_state.lock().unwrap();

    // Get weights
    let weights = [
        game.weights(0).iter().map(trunc).collect(),
        game.weights(1).iter().map(trunc).collect(),
    ];

    // Cache normalized weights
    game.cache_normalized_weights();

    // Get equity, EV, EQR
    let equity_raw = [game.equity(0), game.equity(1)];
    let ev_raw = [game.expected_values(0), game.expected_values(1)];

    // Get strategy (if not terminal/chance)
    let strategy = if !game.is_terminal_node() && !game.is_chance_node() {
        game.strategy().iter().map(round).collect()
    } else {
        Vec::new()
    };

    GameResultsResponse {
        current_player: current_player(&game),
        num_actions: num_actions(&game),
        weights,
        normalizer,
        equity,
        ev,
        eqr,
        strategy,
        action_ev,
        // ...
    }
}
```

---

## JSON Schema

### Complete JSON Structure

```typescript
interface CompleteGameJSON {
  // === Metadata ===
  version: number;
  memo: string;
  exported_at: string;

  // === Game Configuration ===
  game_config: {
    is_solved: boolean;
    storage_mode: "flop" | "turn" | "river";
    board: number[];
    board_readable: string;
    starting_pot: number;
    effective_stack: number;
  };

  // === Ranges ===
  ranges: {
    oop: string;
    ip: string;
  };

  // === Private Cards ===
  private_cards: {
    oop: number[];
    ip: number[];
  };

  // === Current State Results ===
  results: {
    current_player: "oop" | "ip" | "chance" | "terminal";
    num_actions: number;
    is_empty: number;
    eqr_base: number[];

    weights: {
      oop: number[];
      ip: number[];
    };

    normalizer: {
      oop: number[];
      ip: number[];
    };

    equity: {
      oop: number[];
      ip: number[];
    };

    ev: {
      oop: number[];
      ip: number[];
    };

    eqr: {
      oop: number[];
      ip: number[];
    };

    strategy: number[];
    action_ev: number[];
  };

  // === Reference ===
  _reference: {
    card_encoding: string;
    ranks: string[];
    suits: string[];
  };
}
```

### ตัวอย่าง JSON Output

```json
{
  "version": 1,
  "memo": "BTN vs BB - Kh Qh 7c - 100BB",
  "exported_at": "2026-02-04T12:00:00.000Z",

  "game_config": {
    "is_solved": true,
    "storage_mode": "flop",
    "board": [46, 42, 20],
    "board_readable": "Kh Qh 7c",
    "starting_pot": 60,
    "effective_stack": 940
  },

  "ranges": {
    "oop": "66+,A8s+,A5s-A4s,AJo+,K9s+,KQo,QTs+,JTs",
    "ip": "QQ-22,AQs-A2s,ATo+,K5s+,KJo+,Q8s+,J8s+"
  },

  "private_cards": {
    "oop": [258, 257, 256, 255, ...],
    "ip": [770, 769, 768, 767, ...]
  },

  "results": {
    "current_player": "oop",
    "num_actions": 4,
    "is_empty": 0,
    "eqr_base": [1000, 1000],

    "weights": {
      "oop": [1.0, 1.0, 1.0, 0.5, ...],
      "ip": [1.0, 1.0, 0.8, 0.7, ...]
    },

    "normalizer": {
      "oop": [0.001, 0.001, 0.001, 0.0005, ...],
      "ip": [0.0012, 0.0012, 0.001, 0.0009, ...]
    },

    "equity": {
      "oop": [0.85, 0.78, 0.65, 0.52, ...],
      "ip": [0.15, 0.22, 0.35, 0.48, ...]
    },

    "ev": {
      "oop": [45.2, 38.5, 18.3, 5.2, ...],
      "ip": [-45.2, -38.5, -18.3, -5.2, ...]
    },

    "eqr": {
      "oop": [1.15, 1.10, 0.92, 0.85, ...],
      "ip": [0.85, 0.90, 1.08, 1.15, ...]
    },

    "strategy": [0.15, 0.60, 0.25, 0.0, ...],

    "action_ev": [42.5, 48.3, 51.2, 39.8]
  },

  "_reference": {
    "card_encoding": "rank * 4 + suit",
    "ranks": ["2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K", "A"],
    "suits": ["♣", "♦", "♥", "♠"]
  }
}
```

---

## การ Export JSON แบบสมบูรณ์

### Implementation

ดูโค้ดเต็มใน `src/utils/exportComplete.ts`

### Key Points

1. **ดึงข้อมูลจากหลาย API**:
   - `gameInfo()` - Metadata
   - `rangeToString()` - Ranges
   - `gamePrivateCards()` - Card distribution
   - `gameGetResults()` - Strategy, Equity, EV

2. **Decode Board Cards**:
   ```typescript
   function decodeBoard(board: number[]): string {
     const ranks = ["2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K", "A"];
     const suits = ["c", "d", "h", "s"];

     return board.map(card => {
       const rank = ranks[Math.floor(card / 4)];
       const suit = suits[card % 4];
       return rank + suit;
     }).join(" ");
   }
   ```

3. **Card Encoding Formula**:
   ```
   card_number = rank * 4 + suit

   Ranks: 0=2, 1=3, ..., 12=A
   Suits: 0=♣, 1=♦, 2=♥, 3=♠

   Examples:
   - Kh = 11 * 4 + 2 = 46
   - Qh = 10 * 4 + 2 = 42
   - 7c = 5 * 4 + 0 = 20
   ```

---

## ตัวอย่างการใช้งาน

### 1. Load → Solve → Export

```typescript
// 1. Load file
const result = await gameLoadFile("/path/to/game.flop");
console.log("Loaded:", result.memo);

// 2. Run solver (ถ้ายังไม่ได้ solve)
if (!result.is_solved) {
  await gameSolve(1000, 0.5);
}

// 3. Export complete JSON
const completeData = await exportCompleteJSON();
await writeTextFile("output.json", JSON.stringify(completeData, null, 2));

console.log("Export complete!");
```

### 2. Progress Bar Solving

```typescript
const maxIterations = 1000;

for (let i = 0; i < maxIterations; i++) {
  await gameSolveStep(i);

  // Update UI
  const progress = ((i + 1) / maxIterations * 100).toFixed(1);
  console.log(`Progress: ${progress}%`);
}

await gameFinalize();
const completeData = await exportCompleteJSON();
```

### 3. Analyze JSON Data

```typescript
const data = await exportCompleteJSON();

// Get OOP equity for AA
const aaIndex = 0; // AA is usually first
const aaEquity = data.results.equity.oop[aaIndex];
console.log(`AA Equity: ${(aaEquity * 100).toFixed(1)}%`);

// Get strategy distribution
const strategy = data.results.strategy;
const actions = ["Fold", "Check", "Bet 33%", "Bet 75%"];
strategy.forEach((prob, i) => {
  console.log(`${actions[i]}: ${(prob * 100).toFixed(1)}%`);
});
```

---

## API Reference

### Frontend Functions

| Function | Parameters | Returns | Description |
|----------|-----------|---------|-------------|
| `gameLoadFile` | `path: string` | `GameLoadResponse` | โหลดไฟล์ .flop/.bin |
| `gameSolve` | `maxIterations: number, targetExploitability: number` | `SolveResult` | รัน solver ทีเดียว |
| `gameSolveStep` | `currentIteration: number` | `void` | รัน solver ทีละ step |
| `gameGetResults` | - | `Results` | ดึงผลลัพธ์ current node |
| `gameInfo` | - | `GameInfoResponse` | ดึง metadata |
| `rangeToString` | `player: number` | `string` | ดึง range string |
| `gamePrivateCards` | - | `number[][]` | ดึง private cards |
| `exportCompleteJSON` | - | `CompleteGameJSON` | Export JSON แบบสมบูรณ์ |

### Backend Commands

| Command | Function | File |
|---------|----------|------|
| `game_load_file` | `game_load_file` | `src-tauri/src/solver.rs:613` |
| `game_solve` | `game_solve` | `src-tauri/src/solver.rs:263` |
| `game_solve_step` | `game_solve_step` | `src-tauri/src/solver.rs:240` |
| `game_get_results` | `game_get_results` | `src-tauri/src/solver.rs:420` |
| `game_info` | `game_info` | `src-tauri/src/solver.rs:697` |
| `range_to_string` | `range_to_string` | (range module) |
| `game_private_cards` | `game_private_cards` | `src-tauri/src/solver.rs:196` |

---

## Performance Considerations

### File Size

| Component | Size | Export Time |
|-----------|------|------------|
| Metadata | ~500 bytes | <1ms |
| Ranges | ~50 KB | ~10ms |
| Strategy | ~50 MB | ~5s |
| Equity/EV | ~100 MB | ~10s |
| **Total** | **~150 MB** | **~15s** |

### Optimization Tips

1. **Lazy Loading**: โหลดเฉพาะ current node แทนทั้ง tree
2. **Compression**: ใช้ gzip ลดขนาด ~70%
3. **Streaming**: Export ทีละส่วนแทนทั้งหมดพร้อมกัน
4. **Worker Thread**: รัน export ใน background

---

## Troubleshooting

### ❌ "Memory exceeded"
- ลดขนาดข้อมูลที่ export
- Export เฉพาะ current node แทนทั้ง tree
- เพิ่ม RAM หรือใช้ compression

### ❌ "Export taking too long"
- ใช้ streaming แทน export ทั้งหมด
- Export ใน background worker
- ลดความละเอียดของข้อมูล (sampling)

### ❌ "JSON parse error"
- ตรวจสอบ special characters ใน memo
- Escape strings ให้ถูกต้อง
- Validate JSON structure ก่อน export

---

## Future Enhancements

- [ ] Export เฉพาะ specific nodes
- [ ] Compression options (gzip, brotli)
- [ ] Streaming export for large files
- [ ] Export to other formats (CSV, Parquet)
- [ ] CLI tool for batch export
- [ ] Import JSON back to Binary
- [ ] Partial export (metadata only, strategy only, etc.)

---

**เวอร์ชัน**: 1.0.0
**วันที่อัพเดท**: 2026-02-05
**ผู้เขียน**: Desktop Postflop Development Team
