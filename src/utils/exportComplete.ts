import { gameInfo, gameGetResults, gamePrivateCards, rangeToString, gameExportJsonFile } from "../invokes";
import { useStore } from "../store";
import { readTextFile } from '@tauri-apps/api/fs';
import { appCacheDir, join } from '@tauri-apps/api/path';

/**
 * Export complete JSON with all game data including strategy, equity, EV
 *
 * Uses optimized Rust implementation with parallel processing for best performance
 * Writes to a temp file first to avoid IPC timeout issues with large data
 *
 * @returns Complete game data as JSON object
 */
export async function exportCompleteJSON() {
  const store = useStore();

  console.time("[Export] Total Export Time");
  console.log("[Export] Starting optimized complete JSON export...");

  try {
    // For large files (>50MB), use file-based export to avoid IPC timeout
    // This writes directly to a temp file, then reads it back
    console.time("[Export] Rust Export to File");

    // Create temp file path in app cache directory
    const cacheDirPath = await appCacheDir();
    const tempFilePath = await join(cacheDirPath, `postflop-export-${Date.now()}.json`);

    console.log(`[Export] Writing to temp file: ${tempFilePath}`);
    await gameExportJsonFile(tempFilePath);
    console.timeEnd("[Export] Rust Export to File");

    console.time("[Export] Read & Parse");
    const jsonString = await readTextFile(tempFilePath);
    const completeData = JSON.parse(jsonString);
    console.timeEnd("[Export] Read & Parse");

    console.log("[Export] Complete JSON created successfully");
    console.log(`[Export] Data size: ${(jsonString.length / 1024 / 1024).toFixed(2)} MB`);
    console.log(`[Export] Temp file: ${tempFilePath} (can be deleted)`);
    console.timeEnd("[Export] Total Export Time");

    return completeData;

  } catch (error) {
    console.error("[Export] Failed to export complete JSON:", error);
    throw error;
  }
}

/**
 * Export current node results only (fast, for current state visualization)
 *
 * @returns Current node results
 */
export async function exportCurrentNodeJSON() {
  const store = useStore();

  console.log("[Export] Starting current node export...");

  try {
    // 1. Metadata (ข้อมูลพื้นฐาน)
    console.log("[Export] Fetching game info...");
    const info = await gameInfo();

    // 2. Ranges (OOP & IP)
    console.log("[Export] Fetching ranges...");
    const oopRange = await rangeToString(0);
    const ipRange = await rangeToString(1);

    // 3. Private Cards (Card Distribution)
    console.log("[Export] Fetching private cards...");
    const privateCards = await gamePrivateCards();

    // 4. Strategy & Results (ข้อมูลหลัก!)
    console.log("[Export] Fetching results...");
    const results = await gameGetResults();

    // 5. Decode Board Cards
    const decodedBoard = decodeBoard(info.board);

    // 6. Create Complete JSON
    console.log("[Export] Building complete JSON structure...");
    const completeData = {
      // === Metadata ===
      version: 1,
      memo: store.loadedFileMemo,
      exported_at: new Date().toISOString(),

      // === Game Configuration ===
      game_config: {
        is_solved: info.is_solved,
        storage_mode: info.storage_mode,
        board: info.board,
        board_readable: decodedBoard,
        starting_pot: info.starting_pot,
        effective_stack: info.effective_stack,
      },

      // === Ranges ===
      ranges: {
        oop: oopRange,
        ip: ipRange,
      },

      // === Private Cards ===
      private_cards: {
        oop: privateCards[0],
        ip: privateCards[1],
      },

      // === Current State Results ===
      results: {
        current_player: results.currentPlayer,
        num_actions: results.numActions,
        is_empty: results.isEmpty,
        eqr_base: results.eqrBase,

        // Weights (น้ำหนักของแต่ละมือ)
        weights: {
          oop: results.weights[0],
          ip: results.weights[1],
        },

        // Normalizer (น้ำหนักปรับแล้ว)
        normalizer: {
          oop: results.normalizer[0],
          ip: results.normalizer[1],
        },

        // Equity (โอกาสชนะ)
        equity: {
          oop: results.equity[0],
          ip: results.equity[1],
        },

        // Expected Value (มูลค่าที่คาดหวัง)
        ev: {
          oop: results.ev[0],
          ip: results.ev[1],
        },

        // EQR (Equity Realization)
        eqr: {
          oop: results.eqr[0],
          ip: results.eqr[1],
        },

        // Strategy (กลยุทธ์การเล่น - % ของแต่ละ action)
        strategy: results.strategy,

        // Action EV (มูลค่าของแต่ละ action)
        action_ev: results.actionEv,
      },

      // === Decoding Reference ===
      _reference: {
        card_encoding: "rank * 4 + suit",
        ranks: ["2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K", "A"],
        suits: ["♣ (Club)", "♦ (Diamond)", "♥ (Heart)", "♠ (Spade)"],
        suits_short: ["c", "d", "h", "s"],
        note: "This JSON contains complete game data including strategy, equity, and EV for the current node."
      },
    };

    console.log("[Export] Current node JSON created successfully");
    return completeData;

  } catch (error) {
    console.error("[Export] Failed to export current node JSON:", error);
    throw error;
  }
}

/**
 * Decode board cards from numbers to readable format
 *
 * @param board Array of card numbers (0-51)
 * @returns Readable board string (e.g., "Kh Qh 7c")
 */
function decodeBoard(board: number[]): string {
  const ranks = ["2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K", "A"];
  const suits = ["c", "d", "h", "s"];

  return board
    .map((card) => {
      const rank = ranks[Math.floor(card / 4)];
      const suit = suits[card % 4];
      return rank + suit;
    })
    .join(" ");
}

/**
 * Decode a single card number to readable format
 *
 * @param cardNum Card number (0-51)
 * @returns Readable card string (e.g., "Kh")
 */
export function decodeCard(cardNum: number): string {
  const ranks = ["2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K", "A"];
  const suits = ["c", "d", "h", "s"];

  const rank = ranks[Math.floor(cardNum / 4)];
  const suit = suits[cardNum % 4];
  return rank + suit;
}

/**
 * Get summary statistics from complete JSON
 *
 * @param data Complete JSON data
 * @returns Summary statistics
 */
export function getJSONSummary(data: any) {
  return {
    memo: data.memo,
    board: data.game_config.board_readable,
    is_solved: data.game_config.is_solved,
    storage_mode: data.game_config.storage_mode,
    starting_pot: data.game_config.starting_pot,
    effective_stack: data.game_config.effective_stack,
    current_player: data.results.current_player,
    num_actions: data.results.num_actions,
    oop_range: data.ranges.oop,
    ip_range: data.ranges.ip,
    num_oop_hands: data.private_cards.oop.length,
    num_ip_hands: data.private_cards.ip.length,
  };
}
