use crate::range::*;
use postflop_solver::*;
use rayon::ThreadPool;
use serde::Serialize;
use std::sync::Mutex;

#[derive(Serialize)]
pub struct AbstractionEnableResponse {
    pub num_buckets: [usize; 2],
    pub memory_usage: (u64, u64),
}

// File loading/saving response types
#[derive(Serialize)]
pub struct GameLoadResponse {
    pub memo: String,
    pub is_solved: bool,
    pub is_abstraction_enabled: bool,
    pub num_buckets: Option<[usize; 2]>,
    pub storage_mode: String,
    pub board: Vec<u8>,
    pub starting_pot: i32,
    pub effective_stack: i32,
    pub private_cards: [Vec<u16>; 2],
}

#[derive(Serialize)]
pub struct GameInfoResponse {
    pub is_solved: bool,
    pub is_abstraction_enabled: bool,
    pub num_buckets: Option<[usize; 2]>,
    pub storage_mode: String,
    pub board: Vec<u8>,
    pub starting_pot: i32,
    pub effective_stack: i32,
}

#[inline]
fn decode_action(action: &str) -> Action {
    match action {
        "F" => Action::Fold,
        "X" => Action::Check,
        "C" => Action::Call,
        _ => {
            let mut chars = action.chars();
            let first_char = chars.next().unwrap();
            let amount = chars.as_str().parse().unwrap();
            match first_char {
                'B' => Action::Bet(amount),
                'R' => Action::Raise(amount),
                'A' => Action::AllIn(amount),
                _ => unreachable!(),
            }
        }
    }
}

#[inline]
fn action_usize(action: isize) -> usize {
    match action {
        -1 => usize::MAX,
        a => a as usize,
    }
}

#[inline]
fn round(value: f64) -> f64 {
    if value < 1.0 {
        (value * 1000000.0).round() / 1000000.0
    } else if value < 10.0 {
        (value * 100000.0).round() / 100000.0
    } else if value < 100.0 {
        (value * 10000.0).round() / 10000.0
    } else if value < 1000.0 {
        (value * 1000.0).round() / 1000.0
    } else if value < 10000.0 {
        (value * 100.0).round() / 100.0
    } else {
        (value * 10.0).round() / 10.0
    }
}

#[inline]
fn round_iter<'a>(iter: impl Iterator<Item = &'a f32> + 'a) -> impl Iterator<Item = f64> + 'a {
    iter.map(|&x| round(x as f64))
}

#[inline]
pub fn weighted_average(slice: &[f32], weights: &[f32]) -> f64 {
    let mut sum = 0.0;
    let mut weight_sum = 0.0;
    for (&value, &weight) in slice.iter().zip(weights.iter()) {
        sum += value as f64 * weight as f64;
        weight_sum += weight as f64;
    }
    sum / weight_sum
}

#[tauri::command(async)]
pub fn game_init(
    range_state: tauri::State<Mutex<RangeManager>>,
    game_state: tauri::State<Mutex<PostFlopGame>>,
    board: Vec<u8>,
    starting_pot: i32,
    effective_stack: i32,
    rake_rate: f64,
    rake_cap: f64,
    donk_option: bool,
    oop_flop_bet: String,
    oop_flop_raise: String,
    oop_turn_bet: String,
    oop_turn_raise: String,
    oop_turn_donk: String,
    oop_river_bet: String,
    oop_river_raise: String,
    oop_river_donk: String,
    ip_flop_bet: String,
    ip_flop_raise: String,
    ip_turn_bet: String,
    ip_turn_raise: String,
    ip_river_bet: String,
    ip_river_raise: String,
    add_allin_threshold: f64,
    force_allin_threshold: f64,
    merging_threshold: f64,
    added_lines: String,
    removed_lines: String,
) -> Option<String> {
    let (turn, river, state) = match board.len() {
        3 => (NOT_DEALT, NOT_DEALT, BoardState::Flop),
        4 => (board[3], NOT_DEALT, BoardState::Turn),
        5 => (board[3], board[4], BoardState::River),
        _ => return Some("Invalid board length".to_string()),
    };

    let ranges = &range_state.lock().unwrap().0;
    let card_config = CardConfig {
        range: ranges[..2].try_into().unwrap(),
        flop: board[..3].try_into().unwrap(),
        turn,
        river,
    };

    let tree_config = TreeConfig {
        initial_state: state,
        starting_pot,
        effective_stack,
        rake_rate,
        rake_cap,
        flop_bet_sizes: [
            BetSizeOptions::try_from((oop_flop_bet.as_str(), oop_flop_raise.as_str())).unwrap(),
            BetSizeOptions::try_from((ip_flop_bet.as_str(), ip_flop_raise.as_str())).unwrap(),
        ],
        turn_bet_sizes: [
            BetSizeOptions::try_from((oop_turn_bet.as_str(), oop_turn_raise.as_str())).unwrap(),
            BetSizeOptions::try_from((ip_turn_bet.as_str(), ip_turn_raise.as_str())).unwrap(),
        ],
        river_bet_sizes: [
            BetSizeOptions::try_from((oop_river_bet.as_str(), oop_river_raise.as_str())).unwrap(),
            BetSizeOptions::try_from((ip_river_bet.as_str(), ip_river_raise.as_str())).unwrap(),
        ],
        turn_donk_sizes: match donk_option {
            false => None,
            true => DonkSizeOptions::try_from(oop_turn_donk.as_str()).ok(),
        },
        river_donk_sizes: match donk_option {
            false => None,
            true => DonkSizeOptions::try_from(oop_river_donk.as_str()).ok(),
        },
        add_allin_threshold,
        force_allin_threshold,
        merging_threshold,
    };

    let mut action_tree = ActionTree::new(tree_config).unwrap();

    if !added_lines.is_empty() {
        for added_line in added_lines.split(',') {
            let line = added_line
                .split(&['-', '|'][..])
                .map(decode_action)
                .collect::<Vec<_>>();
            if action_tree.add_line(&line).is_err() {
                return Some("Failed to add line (loaded broken tree?)".to_string());
            }
        }
    }

    if !removed_lines.is_empty() {
        for removed_line in removed_lines.split(',') {
            let line = removed_line
                .split(&['-', '|'][..])
                .map(decode_action)
                .collect::<Vec<_>>();
            if action_tree.remove_line(&line).is_err() {
                return Some("Failed to remove line (loaded broken tree?)".to_string());
            }
        }
    }

    let mut game = game_state.lock().unwrap();
    game.update_config(card_config, action_tree).err()
}

#[tauri::command]
pub fn game_private_cards(game_state: tauri::State<Mutex<PostFlopGame>>) -> [Vec<u16>; 2] {
    let game = game_state.lock().unwrap();
    let convert = |player: usize| {
        game.private_cards(player)
            .iter()
            .map(|&(c1, c2)| (c1 as u16) | (c2 as u16) << 8)
            .collect()
    };
    [convert(0), convert(1)]
}

#[tauri::command]
pub fn game_memory_usage(game_state: tauri::State<Mutex<PostFlopGame>>) -> (u64, u64) {
    let game = game_state.lock().unwrap();
    game.memory_usage()
}

#[tauri::command]
pub fn game_memory_usage_with_abstraction(
    game_state: tauri::State<Mutex<PostFlopGame>>,
    num_buckets: usize,
) -> (u64, u64) {
    let game = game_state.lock().unwrap();
    game.memory_usage_with_abstraction(num_buckets)
}

#[tauri::command]
pub fn game_memory_usage_bunching(game_state: tauri::State<Mutex<PostFlopGame>>) -> u64 {
    let game = game_state.lock().unwrap();
    game.memory_usage_bunching()
}

#[tauri::command(async)]
pub fn game_allocate_memory(
    game_state: tauri::State<Mutex<PostFlopGame>>,
    enable_compression: bool,
) {
    let mut game = game_state.lock().unwrap();
    game.allocate_memory(enable_compression);
}

#[tauri::command(async)]
pub fn game_enable_abstraction(
    game_state: tauri::State<Mutex<PostFlopGame>>,
    num_buckets: usize,
    max_iterations: usize,
) -> Result<AbstractionEnableResponse, String> {
    let start = std::time::Instant::now();
    let mut game = game_state.lock().unwrap();

    let config = AbstractionConfig {
        num_buckets,
        max_iterations,
    };

    game.enable_abstraction(&config)?;
    println!("enable_abstraction took {:?}", start.elapsed());

    let num_buckets_result = [
        game.effective_hand_count(0),
        game.effective_hand_count(1),
    ];
    let memory_usage = game.memory_usage();

    Ok(AbstractionEnableResponse {
        num_buckets: num_buckets_result,
        memory_usage,
    })
}

#[tauri::command(async)]
pub fn game_set_bunching(
    bunching_state: tauri::State<Mutex<Option<BunchingData>>>,
    game_state: tauri::State<Mutex<PostFlopGame>>,
) -> Option<String> {
    let bunching_data = bunching_state.lock().unwrap();
    let bunching_data = bunching_data.as_ref().unwrap();
    let mut game = game_state.lock().unwrap();
    game.set_bunching_effect(bunching_data).err()
}

#[tauri::command(async)]
pub fn game_solve_step(
    game_state: tauri::State<Mutex<PostFlopGame>>,
    pool_state: tauri::State<Mutex<ThreadPool>>,
    current_iteration: u32,
) {
    let start = std::time::Instant::now();
    let game = game_state.lock().unwrap();
    let pool = pool_state.lock().unwrap();
    pool.install(|| solve_step(&*game, current_iteration));
    let elapsed = start.elapsed();
    if current_iteration < 5 || current_iteration % 100 == 0 {
        println!("solve_step({}) took {:?}", current_iteration, elapsed);
    }
}

#[derive(Serialize)]
pub struct SolveResult {
    pub iterations: u32,
    pub exploitability: f32,
    pub time_ms: u64,
}

#[tauri::command(async)]
pub fn game_solve(
    game_state: tauri::State<Mutex<PostFlopGame>>,
    pool_state: tauri::State<Mutex<ThreadPool>>,
    max_iterations: u32,
    target_exploitability: f32,
) -> SolveResult {
    let start = std::time::Instant::now();
    let pool = pool_state.lock().unwrap();

    // Acquire game lock inside pool.install to avoid Send issue
    let exploitability = pool.install(|| {
        let mut game = game_state.lock().unwrap();
        solve(&mut *game, max_iterations, target_exploitability, true)
    });

    let time_ms = start.elapsed().as_millis() as u64;
    println!("game_solve completed: {} iterations, exploitability={:.4}, time={}ms",
             max_iterations, exploitability, time_ms);

    SolveResult {
        iterations: max_iterations,
        exploitability,
        time_ms,
    }
}

#[tauri::command(async)]
pub fn game_exploitability(
    game_state: tauri::State<Mutex<PostFlopGame>>,
    pool_state: tauri::State<Mutex<ThreadPool>>,
) -> f32 {
    let start = std::time::Instant::now();
    let game = game_state.lock().unwrap();
    let pool = pool_state.lock().unwrap();
    let result = pool.install(|| compute_exploitability(&*game));
    println!("compute_exploitability took {:?}, result = {}", start.elapsed(), result);
    result
}

#[tauri::command(async)]
pub fn game_finalize(
    game_state: tauri::State<Mutex<PostFlopGame>>,
    pool_state: tauri::State<Mutex<ThreadPool>>,
) {
    let pool = pool_state.lock().unwrap();
    pool.install(|| finalize(&mut *game_state.lock().unwrap()));
}

#[tauri::command]
pub fn game_apply_history(game_state: tauri::State<Mutex<PostFlopGame>>, history: Vec<usize>) {
    let mut game = game_state.lock().unwrap();
    game.apply_history(&history);
}

#[tauri::command]
pub fn game_total_bet_amount(
    game_state: tauri::State<Mutex<PostFlopGame>>,
    append: Vec<isize>,
) -> [i32; 2] {
    let mut game = game_state.lock().unwrap();
    if append.is_empty() {
        return game.total_bet_amount();
    }
    let history = game.history().to_vec();
    for &action in &append {
        game.play(action_usize(action));
    }
    let ret = game.total_bet_amount();
    game.apply_history(&history);
    ret
}

fn actions(game: &PostFlopGame) -> Vec<String> {
    if game.is_terminal_node() {
        vec!["terminal".to_string()]
    } else if game.is_chance_node() {
        vec!["chance".to_string()]
    } else {
        game.available_actions()
            .iter()
            .map(|&x| match x {
                Action::Fold => "Fold:0".to_string(),
                Action::Check => "Check:0".to_string(),
                Action::Call => "Call:0".to_string(),
                Action::Bet(amount) => format!("Bet:{amount}"),
                Action::Raise(amount) => format!("Raise:{amount}"),
                Action::AllIn(amount) => format!("Allin:{amount}"),
                _ => unreachable!(),
            })
            .collect()
    }
}

#[tauri::command]
pub fn game_actions_after(
    game_state: tauri::State<Mutex<PostFlopGame>>,
    append: Vec<isize>,
) -> Vec<String> {
    let mut game = game_state.lock().unwrap();
    if append.is_empty() {
        return actions(&game);
    }
    println!("[DEBUG] game_actions_after: append={:?}", append);
    println!("[DEBUG] current history={:?}, is_chance={}, is_terminal={}",
        game.history(), game.is_chance_node(), game.is_terminal_node());
    println!("[DEBUG] available_actions={:?}", game.available_actions());
    let history = game.history().to_vec();
    for &action in &append {
        println!("[DEBUG] playing action {} (usize={})", action, action_usize(action));
        game.play(action_usize(action));
    }
    let ret = actions(&game);
    game.apply_history(&history);
    ret
}

#[tauri::command]
pub fn game_possible_cards(game_state: tauri::State<Mutex<PostFlopGame>>) -> u64 {
    let game = game_state.lock().unwrap();
    game.possible_cards()
}

fn current_player(game: &PostFlopGame) -> String {
    if game.is_terminal_node() {
        "terminal".to_string()
    } else if game.is_chance_node() {
        "chance".to_string()
    } else if game.current_player() == 0 {
        "oop".to_string()
    } else {
        "ip".to_string()
    }
}

pub fn num_actions(game: &PostFlopGame) -> usize {
    match game.is_chance_node() {
        true => 0,
        false => game.available_actions().len(),
    }
}

#[derive(Serialize)]
pub struct GameResultsResponse {
    current_player: String,
    num_actions: usize,
    is_empty: i32,
    eqr_base: [i32; 2],
    weights: [Vec<f64>; 2],
    normalizer: [Vec<f64>; 2],
    equity: [Vec<f64>; 2],
    ev: [Vec<f64>; 2],
    eqr: [Vec<f64>; 2],
    strategy: Vec<f64>,
    action_ev: Vec<f64>,
}

#[tauri::command]
pub fn game_get_results(game_state: tauri::State<Mutex<PostFlopGame>>) -> GameResultsResponse {
    let mut game = game_state.lock().unwrap();

    let total_bet_amount = game.total_bet_amount();
    let pot_base = game.tree_config().starting_pot + total_bet_amount.iter().min().unwrap();
    let eqr_base = [
        pot_base + total_bet_amount[0],
        pot_base + total_bet_amount[1],
    ];

    let trunc = |&w: &f32| if w < 0.0005 { 0.0 } else { round(w as f64) };
    let weights = [
        game.weights(0).iter().map(trunc).collect::<Vec<_>>(),
        game.weights(1).iter().map(trunc).collect::<Vec<_>>(),
    ];

    let is_empty = |player: usize| weights[player].iter().all(|&w| w == 0.0);
    let is_empty_flag = is_empty(0) as i32 + 2 * is_empty(1) as i32;

    let mut normalizer = [Vec::new(), Vec::new()];
    let mut equity = [Vec::new(), Vec::new()];
    let mut ev = [Vec::new(), Vec::new()];
    let mut eqr = [Vec::new(), Vec::new()];

    if is_empty_flag > 0 {
        normalizer[0].extend(weights[0].iter());
        normalizer[1].extend(weights[1].iter());
    } else {
        game.cache_normalized_weights();

        normalizer[0].extend(round_iter(game.normalized_weights(0).iter()));
        normalizer[1].extend(round_iter(game.normalized_weights(1).iter()));

        let equity_raw = [game.equity(0), game.equity(1)];
        let ev_raw = [game.expected_values(0), game.expected_values(1)];

        equity[0].extend(round_iter(equity_raw[0].iter()));
        equity[1].extend(round_iter(equity_raw[1].iter()));
        ev[0].extend(round_iter(ev_raw[0].iter()));
        ev[1].extend(round_iter(ev_raw[1].iter()));

        for player in 0..2 {
            let pot = eqr_base[player] as f64;
            for (&eq, &ev) in equity_raw[player].iter().zip(ev_raw[player].iter()) {
                let (eq, ev) = (eq as f64, ev as f64);
                if eq < 5e-7 {
                    eqr[player].push(ev / 0.0);
                } else {
                    eqr[player].push(round(ev / (pot * eq)));
                }
            }
        }
    }

    let mut strategy = Vec::new();
    let mut action_ev = Vec::new();

    if !game.is_terminal_node() && !game.is_chance_node() {
        strategy.extend(round_iter(game.strategy().iter()));
        if is_empty_flag == 0 {
            action_ev.extend(round_iter(
                game.expected_values_detail(game.current_player()).iter(),
            ));
        }
    }

    GameResultsResponse {
        current_player: current_player(&game),
        num_actions: num_actions(&game),
        is_empty: is_empty_flag,
        eqr_base,
        weights,
        normalizer,
        equity,
        ev,
        eqr,
        strategy,
        action_ev,
    }
}

#[derive(Serialize)]
pub struct GameChanceReportsResponse {
    status: Vec<i32>,
    combos: [Vec<f64>; 2],
    equity: [Vec<f64>; 2],
    ev: [Vec<f64>; 2],
    eqr: [Vec<f64>; 2],
    strategy: Vec<f64>,
}

#[tauri::command]
pub fn game_get_chance_reports(
    game_state: tauri::State<Mutex<PostFlopGame>>,
    append: Vec<isize>,
    num_actions: usize,
) -> GameChanceReportsResponse {
    let mut game = game_state.lock().unwrap();
    let history = game.history().to_vec();

    let mut status = vec![0; 52]; // 0: not possible, 1: empty, 2: not empty
    let mut combos = [vec![0.0; 52], vec![0.0; 52]];
    let mut equity = [vec![0.0; 52], vec![0.0; 52]];
    let mut ev = [vec![0.0; 52], vec![0.0; 52]];
    let mut eqr = [vec![0.0; 52], vec![0.0; 52]];
    let mut strategy = vec![0.0; num_actions * 52];

    let possible_cards = game.possible_cards();
    for chance in 0..52 {
        if possible_cards & (1 << chance) == 0 {
            continue;
        }

        game.play(chance);
        for &action in &append[1..] {
            game.play(action_usize(action));
        }

        let trunc = |&w: &f32| if w < 0.0005 { 0.0 } else { w };
        let weights = [
            game.weights(0).iter().map(trunc).collect::<Vec<_>>(),
            game.weights(1).iter().map(trunc).collect::<Vec<_>>(),
        ];

        combos[0][chance] = round(weights[0].iter().fold(0.0, |acc, &w| acc + w as f64));
        combos[1][chance] = round(weights[1].iter().fold(0.0, |acc, &w| acc + w as f64));

        let is_empty = |player: usize| weights[player].iter().all(|&w| w == 0.0);
        let is_empty_flag = [is_empty(0), is_empty(1)];

        game.cache_normalized_weights();
        let normalizer = [game.normalized_weights(0), game.normalized_weights(1)];

        if !game.is_terminal_node() {
            let current_player = game.current_player();
            if !is_empty_flag[current_player] {
                let strategy_tmp = game.strategy();
                let num_hands = game.private_cards(current_player).len();
                let ws = if is_empty_flag[current_player ^ 1] {
                    &weights[current_player]
                } else {
                    normalizer[current_player]
                };
                for action in 0..num_actions {
                    let slice = &strategy_tmp[action * num_hands..(action + 1) * num_hands];
                    let strategy_summary = weighted_average(slice, ws);
                    strategy[action * 52 + chance] = round(strategy_summary);
                }
            }
        }

        if is_empty_flag[0] || is_empty_flag[1] {
            status[chance] = 1;
            game.apply_history(&history);
            continue;
        }

        status[chance] = 2;

        let total_bet_amount = game.total_bet_amount();
        let pot_base = game.tree_config().starting_pot + total_bet_amount.iter().min().unwrap();

        for player in 0..2 {
            let pot = (pot_base + total_bet_amount[player]) as f32;
            let equity_tmp = weighted_average(&game.equity(player), normalizer[player]);
            let ev_tmp = weighted_average(&game.expected_values(player), normalizer[player]);
            equity[player][chance] = round(equity_tmp);
            ev[player][chance] = round(ev_tmp);
            eqr[player][chance] = round(ev_tmp / (pot as f64 * equity_tmp));
        }

        game.apply_history(&history);
    }

    GameChanceReportsResponse {
        status,
        combos,
        equity,
        ev,
        eqr,
        strategy,
    }
}

fn board_state_to_string(state: BoardState) -> String {
    match state {
        BoardState::Flop => "flop".to_string(),
        BoardState::Turn => "turn".to_string(),
        BoardState::River => "river".to_string(),
    }
}

#[tauri::command(async)]
pub fn game_load_file(
    game_state: tauri::State<Mutex<PostFlopGame>>,
    path: String,
) -> Result<GameLoadResponse, String> {
    let start = std::time::Instant::now();
    println!("[UI] Loading file: {}", path);

    // First, read header only to validate
    match read_file_info(&path) {
        Ok(info) => {
            println!("[UI] File info:");
            println!("  - Version: {}", info.version);
            println!("  - Compression: {}", if info.compression_type == 0 { "none" } else { "zstd" });
            println!("  - Data type: {}", info.data_type);
            println!("  - Estimated memory: {} MB", info.estimated_memory_usage / 1_048_576);
            println!("  - File size: {} MB", info.file_size / 1_048_576);
            println!("  - Memo: {} chars", info.memo.len());
        }
        Err(e) => {
            println!("[UI] Warning: Could not read file info: {}", e);
        }
    }

    println!("[UI] Starting full load...");
    let load_start = std::time::Instant::now();

    let (loaded_game, memo): (PostFlopGame, String) =
        load_data_from_file(&path, None).map_err(|e| e.to_string())?;

    println!("[UI] Load completed in {:?}", load_start.elapsed());

    let is_solved = loaded_game.is_solved();
    let is_abstraction_enabled = loaded_game.is_abstraction_enabled();
    let num_buckets = loaded_game.abstraction_data().map(|d| [d.num_buckets(0), d.num_buckets(1)]);
    let storage_mode = board_state_to_string(loaded_game.storage_mode());

    let card_config = loaded_game.card_config();
    let mut board = card_config.flop.to_vec();
    if card_config.turn != NOT_DEALT {
        board.push(card_config.turn);
    }
    if card_config.river != NOT_DEALT {
        board.push(card_config.river);
    }

    let tree_config = loaded_game.tree_config();
    let starting_pot = tree_config.starting_pot;
    let effective_stack = tree_config.effective_stack;

    let convert = |player: usize| {
        loaded_game
            .private_cards(player)
            .iter()
            .map(|&(c1, c2)| (c1 as u16) | (c2 as u16) << 8)
            .collect()
    };
    let private_cards = [convert(0), convert(1)];

    // Store the loaded game
    *game_state.lock().unwrap() = loaded_game;

    println!("[UI] Total time: {:?}", start.elapsed());

    Ok(GameLoadResponse {
        memo,
        is_solved,
        is_abstraction_enabled,
        num_buckets,
        storage_mode,
        board,
        starting_pot,
        effective_stack,
        private_cards,
    })
}

#[tauri::command(async)]
pub fn game_save_file(
    game_state: tauri::State<Mutex<PostFlopGame>>,
    path: String,
    memo: String,
    compression_level: Option<i32>,
) -> Result<(), String> {
    let game = game_state.lock().unwrap();
    save_data_to_file(&*game, &memo, &path, compression_level).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn game_info(game_state: tauri::State<Mutex<PostFlopGame>>) -> GameInfoResponse {
    let game = game_state.lock().unwrap();

    let is_solved = game.is_solved();
    let is_abstraction_enabled = game.is_abstraction_enabled();
    let num_buckets = game.abstraction_data().map(|d| [d.num_buckets(0), d.num_buckets(1)]);
    let storage_mode = board_state_to_string(game.storage_mode());

    let card_config = game.card_config();
    let mut board = card_config.flop.to_vec();
    if card_config.turn != NOT_DEALT {
        board.push(card_config.turn);
    }
    if card_config.river != NOT_DEALT {
        board.push(card_config.river);
    }

    let tree_config = game.tree_config();
    let starting_pot = tree_config.starting_pot;
    let effective_stack = tree_config.effective_stack;

    GameInfoResponse {
        is_solved,
        is_abstraction_enabled,
        num_buckets,
        storage_mode,
        board,
        starting_pot,
        effective_stack,
    }
}
