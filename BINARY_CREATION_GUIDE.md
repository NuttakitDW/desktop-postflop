# 📦 Binary File Creation Guide - Desktop Postflop

คู่มือสร้างไฟล์ Binary (.flop) หลายๆ แบบ พร้อมตัวอย่างโค้ดและวิธีใช้งาน

---

## 📋 สารบัญ

1. [วิธีที่ 1: ใช้ GUI (ง่ายสุด)](#วิธีที่-1-ใช้-gui)
2. [วิธีที่ 2: ใช้ Rust Code](#วิธีที่-2-ใช้-rust-code)
3. [ตัวอย่าง Setup หลายแบบ](#ตัวอย่าง-setup-หลายแบบ)
4. [Board Textures](#board-textures)
5. [Range Examples](#range-examples)
6. [Bet Sizing Patterns](#bet-sizing-patterns)

---

## วิธีที่ 1: ใช้ GUI

### Quick Test (2 นาที)

```bash
npm run tauri dev
```

#### Setup:
```
OOP Range: AA,KK,QQ
IP Range:  AA,KK,QQ,JJ

Board: As Kd 7c

Starting Pot: 60
Effective Stack: 940

Bet Sizes:
  OOP Flop: 50%, 100%
  IP Flop:  50%, 100%

Iterations: 100
Target Exploitability: 2.0
```

คลิก **Run Solver** → **Save** → บันทึกเป็น `test_quick.flop`

---

### Standard Game (10 นาที)

#### Setup:
```
OOP Range: 66+,A8s+,A5s-A4s,AJo+,K9s+,KQo,QTs+,JTs
IP Range:  QQ-22,AQs-A2s,ATo+,K5s+,KJo+,Q8s+,J8s+

Board: Kh Qh 7c

Starting Pot: 60
Effective Stack: 940

Bet Sizes:
  OOP Flop: 33%, 75%, 150%
  IP Flop:  33%, 75%, 150%

  Raises: 2.5x, a (all-in)

Iterations: 1000
Target Exploitability: 0.5
```

บันทึกเป็น `btn_vs_bb_khigh.flop`

---

## วิธีที่ 2: ใช้ Rust Code

สร้างไฟล์ `src-tauri/postflop-solver/examples/create_games.rs`

### ตัวอย่าง 1: Quick Test

```rust
use postflop_solver::*;

fn main() {
    println!("Creating quick test game...");

    // Ranges
    let oop_range = "AA,KK,QQ";
    let ip_range = "AA,KK,QQ,JJ";

    // Cards
    let card_config = CardConfig {
        range: [
            oop_range.parse().unwrap(),
            ip_range.parse().unwrap(),
        ],
        flop: flop_from_str("AsKd7c").unwrap(),
        turn: NOT_DEALT,
        river: NOT_DEALT,
    };

    // Tree configuration
    let bet_sizes = BetSizeOptions::try_from(("50%", "2x")).unwrap();
    let tree_config = TreeConfig {
        initial_state: BoardState::Flop,
        starting_pot: 60,
        effective_stack: 940,
        rake_rate: 0.0,
        rake_cap: 0.0,
        flop_bet_sizes: [bet_sizes.clone(), bet_sizes.clone()],
        turn_bet_sizes: [bet_sizes.clone(), bet_sizes.clone()],
        river_bet_sizes: [bet_sizes.clone(), bet_sizes],
        turn_donk_sizes: None,
        river_donk_sizes: None,
        add_allin_threshold: 1.5,
        force_allin_threshold: 0.15,
        merging_threshold: 0.1,
        max_raises_per_street: 0, // unlimited
    };

    // Create and solve
    let action_tree = ActionTree::new(tree_config).unwrap();
    let mut game = PostFlopGame::with_config(card_config, action_tree).unwrap();

    game.allocate_memory(false);

    let max_iterations = 100;
    let target_exploitability = game.tree_config().starting_pot as f32 * 0.02;
    solve(&mut game, max_iterations, target_exploitability, true);

    // Save
    save_data_to_file(
        &game,
        "Quick test - AA-QQ vs AA-JJ",
        "test_quick.flop",
        None
    ).unwrap();

    println!("✅ Saved: test_quick.flop");
}
```

### ตัวอย่าง 2: BTN vs BB (Standard)

```rust
use postflop_solver::*;

fn main() {
    println!("Creating BTN vs BB standard game...");

    // Standard SRP ranges
    let oop_range = "66+,A8s+,A5s-A4s,AJo+,K9s+,KQo,QTs+,JTs,96s+,85s+,75s+,65s,54s";
    let ip_range = "QQ-22,AQs-A2s,ATo+,K5s+,KJo+,Q8s+,J8s+,T7s+,96s+,86s+,75s+,64s+,53s+";

    // Cards
    let card_config = CardConfig {
        range: [
            oop_range.parse().unwrap(),
            ip_range.parse().unwrap(),
        ],
        flop: flop_from_str("KhQh7c").unwrap(),
        turn: NOT_DEALT,
        river: NOT_DEALT,
    };

    // Tree - multiple bet sizes
    let oop_bet = BetSizeOptions::try_from(("33%, 75%, 150%", "2.5x, a")).unwrap();
    let ip_bet = BetSizeOptions::try_from(("33%, 75%, 150%", "2.5x, a")).unwrap();

    let tree_config = TreeConfig {
        initial_state: BoardState::Flop,
        starting_pot: 60,
        effective_stack: 940,
        rake_rate: 0.0,
        rake_cap: 0.0,
        flop_bet_sizes: [oop_bet.clone(), ip_bet.clone()],
        turn_bet_sizes: [oop_bet.clone(), ip_bet.clone()],
        river_bet_sizes: [oop_bet, ip_bet],
        turn_donk_sizes: None,
        river_donk_sizes: None,
        add_allin_threshold: 1.5,
        force_allin_threshold: 0.15,
        merging_threshold: 0.1,
        max_raises_per_street: 0,
    };

    // Create and solve
    let action_tree = ActionTree::new(tree_config).unwrap();
    let mut game = PostFlopGame::with_config(card_config, action_tree).unwrap();

    game.allocate_memory(false);

    let max_iterations = 1000;
    let target_exploitability = game.tree_config().starting_pot as f32 * 0.005;
    solve(&mut game, max_iterations, target_exploitability, true);

    // Save
    save_data_to_file(
        &game,
        "BTN vs BB - Kh Qh 7c - 100BB",
        "btn_vs_bb_khigh.flop",
        None
    ).unwrap();

    println!("✅ Saved: btn_vs_bb_khigh.flop");
}
```

### ตัวอย่าง 3: Multi-Street (Turn + River)

```rust
use postflop_solver::*;

fn main() {
    println!("Creating multi-street game...");

    let oop_range = "66+,A8s+,AJo+,K9s+,QTs+";
    let ip_range = "QQ-22,AQs-A2s,ATo+,K5s+";

    // Include Turn card
    let card_config = CardConfig {
        range: [
            oop_range.parse().unwrap(),
            ip_range.parse().unwrap(),
        ],
        flop: flop_from_str("9h8c7d").unwrap(),
        turn: card_from_str("2s").unwrap(), // Turn card
        river: NOT_DEALT,
    };

    let bet_sizes = BetSizeOptions::try_from(("50%, 100%", "2.5x")).unwrap();

    let tree_config = TreeConfig {
        initial_state: BoardState::Turn, // Start from Turn
        starting_pot: 200,  // Pot after flop action
        effective_stack: 800,
        rake_rate: 0.0,
        rake_cap: 0.0,
        flop_bet_sizes: [bet_sizes.clone(), bet_sizes.clone()],
        turn_bet_sizes: [bet_sizes.clone(), bet_sizes.clone()],
        river_bet_sizes: [bet_sizes.clone(), bet_sizes],
        turn_donk_sizes: None,
        river_donk_sizes: Some(DonkSizeOptions::try_from("50%, 100%").unwrap()),
        add_allin_threshold: 1.5,
        force_allin_threshold: 0.15,
        merging_threshold: 0.1,
        max_raises_per_street: 0,
    };

    let action_tree = ActionTree::new(tree_config).unwrap();
    let mut game = PostFlopGame::with_config(card_config, action_tree).unwrap();

    game.allocate_memory(false);

    let max_iterations = 500;
    let target_exploitability = game.tree_config().starting_pot as f32 * 0.01;
    solve(&mut game, max_iterations, target_exploitability, true);

    save_data_to_file(
        &game,
        "9h8c7d2s - Turn spot",
        "turn_connected.flop",
        None
    ).unwrap();

    println!("✅ Saved: turn_connected.flop");
}
```

### รัน Rust Examples

```bash
cd src-tauri/postflop-solver

# รันตัวอย่าง
cargo run --release --example create_games

# หรือสร้างหลายไฟล์
cargo run --release --example create_all_games
```

---

## ตัวอย่าง Setup หลายแบบ

### 1. Heads-Up Cash Game (100BB)

```rust
// BTN vs BB - Standard SRP
OOP: "66+,A8s+,A5s-A4s,AJo+,K9s+,KQo,QTs+,JTs"
IP:  "QQ-22,AQs-A2s,ATo+,K5s+,KJo+,Q8s+,J8s+"

Board: "Kh Qh 7c"
Pot: 60
Stack: 940

Bet Sizes: "33%, 75%, 150%"
Raises: "2.5x, a"
```

### 2. Tournament (20BB)

```rust
// Short stack
OOP: "88+,A9s+,AJo+,KQs"
IP:  "77+,A8s+,ATo+,KJs+"

Board: "As 7h 3d"
Pot: 30
Stack: 170

Bet Sizes: "50%, a" // Limited sizing
Raises: "a"         // All-in only
```

### 3. Polarized Board

```rust
// Ace-high dry
OOP: "88+,ATs+,AJo+,KQs"
IP:  "77+,A9s+,ATo+,KJs+"

Board: "Ah Kc 3s"
Pot: 60
Stack: 940

Bet Sizes: "33%, 75%, 150%"
```

### 4. Wet/Connected Board

```rust
// Many draws
OOP: "66+,A8s+,AJo+,K9s+,QTs+,JTs,T9s,98s,87s"
IP:  "66+,A7s+,ATo+,K8s+,Q9s+,J9s+,T8s+,97s+,86s+"

Board: "9h 8c 7d"
Pot: 60
Stack: 940

Bet Sizes: "50%, 100%, 150%"
Raises: "2.5x, a"
```

### 5. Paired Board

```rust
// Static texture
OOP: "77+,A9s+,AJo+,KTs+"
IP:  "66+,A8s+,ATo+,K9s+"

Board: "8h 8c 3d"
Pot: 60
Stack: 940

Bet Sizes: "33%, 75%"  // Smaller sizes
Raises: "2.5x"
```

---

## Board Textures

### High Boards
```rust
// Dry
"Ah Kc 7s"
"As Qd 3h"
"Kh Jc 4d"

// Connected
"Ks Qh Jc"
"Qd Jh 9s"
"Jh Ts 8c"

// Monotone
"Ah Kh 6h"
"Qs Js 4s"
```

### Middle Boards
```rust
// Paired
"8h 8c 3s"
"9s 9d 2h"
"7d 7c 4s"

// Connected
"9h 8c 7d"
"8s 7h 5c"
"7c 6d 4h"

// Two-tone
"9h 8h 3c"
"8s 7s 2d"
```

### Low Boards
```rust
// Monotone
"6h 5h 3h"
"7c 4c 2c"

// Dry
"7s 4d 2h"
"6c 3h 2d"

// Connected
"6h 5c 4d"
"5s 4h 2c"
```

---

## Range Examples

### Cash Game Ranges

#### BTN Open (100BB)
```
22+,A2s+,K2s+,Q2s+,J6s+,T6s+,96s+,86s+,75s+,65s,54s,
A2o+,K9o+,Q9o+,J9o+,T9o
```

#### BB Defense vs BTN
```
22+,A2s+,K2s+,Q2s+,J5s+,T6s+,96s+,86s+,75s+,65s,54s,
A2o+,K5o+,Q8o+,J8o+,T8o+,98o
```

#### SB vs BB 3-bet Defense
```
77+,A9s+,AJo+,KTs+,KQo,QJs
```

### Tournament Ranges (20BB)

#### BTN Push
```
22+,A2s+,K2s+,Q5s+,J8s+,T8s+,98s,
A2o+,K8o+,Q9o+,J9o+,T9o
```

#### BB Call vs BTN Push
```
88+,A9s+,ATo+,KJs+
```

---

## Bet Sizing Patterns

### Standard (3-bet sizing)
```rust
Bet:   "33%, 75%, 150%"
Raise: "2.5x, a"
```

### Polarized (2-bet sizing)
```rust
Bet:   "50%, 150%"
Raise: "2.5x"
```

### Geometric (GTO wizard style)
```rust
Bet:   "50%, e, a"  // e = geometric
Raise: "2.5x"
```

### Short Stack (limited)
```rust
Bet:   "50%, a"
Raise: "a"
```

### Multiple Streets
```rust
Flop:  "33%, 75%, 150%"
Turn:  "50%, 100%, a"
River: "75%, 150%, a"
```

---

## Batch Creation Script

สร้างไฟล์ `src-tauri/postflop-solver/examples/create_all_games.rs`

```rust
use postflop_solver::*;

struct GameSetup {
    name: String,
    oop_range: String,
    ip_range: String,
    board: String,
    pot: i32,
    stack: i32,
    bet_sizes: String,
    iterations: u32,
}

fn main() {
    let games = vec![
        GameSetup {
            name: "test_quick".to_string(),
            oop_range: "AA,KK,QQ".to_string(),
            ip_range: "AA,KK,QQ,JJ".to_string(),
            board: "AsKd7c".to_string(),
            pot: 60,
            stack: 940,
            bet_sizes: "50%".to_string(),
            iterations: 100,
        },
        GameSetup {
            name: "btn_vs_bb_khigh".to_string(),
            oop_range: "66+,A8s+,AJo+".to_string(),
            ip_range: "QQ-22,AQs-A2s,ATo+".to_string(),
            board: "KhQh7c".to_string(),
            pot: 60,
            stack: 940,
            bet_sizes: "33%, 75%, 150%".to_string(),
            iterations: 1000,
        },
        GameSetup {
            name: "btn_vs_bb_ahigh".to_string(),
            oop_range: "88+,ATs+,AJo+".to_string(),
            ip_range: "77+,A9s+,ATo+".to_string(),
            board: "AhKc3s".to_string(),
            pot: 60,
            stack: 940,
            bet_sizes: "33%, 75%".to_string(),
            iterations: 1000,
        },
        GameSetup {
            name: "connected_board".to_string(),
            oop_range: "66+,A8s+,AJo+,K9s+,QTs+".to_string(),
            ip_range: "66+,A7s+,ATo+,K8s+".to_string(),
            board: "9h8c7d".to_string(),
            pot: 60,
            stack: 940,
            bet_sizes: "50%, 100%".to_string(),
            iterations: 800,
        },
        GameSetup {
            name: "paired_board".to_string(),
            oop_range: "77+,A9s+,AJo+".to_string(),
            ip_range: "66+,A8s+,ATo+".to_string(),
            board: "8h8c3d".to_string(),
            pot: 60,
            stack: 940,
            bet_sizes: "33%, 75%".to_string(),
            iterations: 800,
        },
    ];

    for setup in games {
        println!("Creating: {}...", setup.name);

        let card_config = CardConfig {
            range: [
                setup.oop_range.parse().unwrap(),
                setup.ip_range.parse().unwrap(),
            ],
            flop: flop_from_str(&setup.board).unwrap(),
            turn: NOT_DEALT,
            river: NOT_DEALT,
        };

        let bet_sizes = BetSizeOptions::try_from((setup.bet_sizes.as_str(), "2.5x")).unwrap();

        let tree_config = TreeConfig {
            initial_state: BoardState::Flop,
            starting_pot: setup.pot,
            effective_stack: setup.stack,
            rake_rate: 0.0,
            rake_cap: 0.0,
            flop_bet_sizes: [bet_sizes.clone(), bet_sizes.clone()],
            turn_bet_sizes: [bet_sizes.clone(), bet_sizes.clone()],
            river_bet_sizes: [bet_sizes.clone(), bet_sizes],
            turn_donk_sizes: None,
            river_donk_sizes: None,
            add_allin_threshold: 1.5,
            force_allin_threshold: 0.15,
            merging_threshold: 0.1,
            max_raises_per_street: 0,
        };

        let action_tree = ActionTree::new(tree_config).unwrap();
        let mut game = PostFlopGame::with_config(card_config, action_tree).unwrap();

        game.allocate_memory(false);

        let target_exploitability = setup.pot as f32 * 0.01;
        solve(&mut game, setup.iterations, target_exploitability, true);

        let filename = format!("{}.flop", setup.name);
        save_data_to_file(&game, &setup.name, &filename, None).unwrap();

        println!("✅ Saved: {}", filename);
    }

    println!("\n🎉 All games created!");
}
```

รัน:
```bash
cargo run --release --example create_all_games
```

---

## Card Encoding Reference

```rust
// Ranks: 0-12
0=2, 1=3, 2=4, 3=5, 4=6, 5=7, 6=8, 7=9,
8=T, 9=J, 10=Q, 11=K, 12=A

// Suits: 0-3
0=♣ (Club), 1=♦ (Diamond), 2=♥ (Heart), 3=♠ (Spade)

// Formula: card_number = rank * 4 + suit

// Examples:
Ah = 12 * 4 + 2 = 50
Kh = 11 * 4 + 2 = 46
7c = 5 * 4 + 0 = 20
```

---

## Performance Tips

### Quick Testing
```
Iterations: 100-300
Range: แคบ (AA-JJ)
Bet Sizes: 1-2 ตัวเลือก
Time: 1-2 นาที
```

### Standard Solving
```
Iterations: 1000-3000
Range: ปกติ
Bet Sizes: 2-3 ตัวเลือก
Time: 5-15 นาที
```

### High Accuracy
```
Iterations: 10000+
Range: กว้าง
Bet Sizes: 3+ ตัวเลือก
Time: 30+ นาที
```

---

## Troubleshooting

### "Out of memory"
```rust
// ลด range
OOP: "88+,ATs+,AJo+" // แทน 66+,...

// ลด bet sizes
Bet: "50%, 100%" // แทน 33%, 75%, 150%

// Enable compression
game.allocate_memory(true); // true = enable
```

### "Solver too slow"
```rust
// ลด iterations
max_iterations: 500 // แทน 1000

// เพิ่ม target exploitability
target_exp: 1.0 // แทน 0.5
```

### "Invalid board"
```rust
// ต้องใช้ flop_from_str() ที่ถูกต้อง
flop_from_str("KhQh7c").unwrap() // ✅
flop_from_str("Kh Qh 7c").unwrap() // ❌ (มี space)
```

---

**เวอร์ชัน**: 1.0.0
**วันที่**: 2026-02-05
**Last Updated**: Complete with all examples and patterns
