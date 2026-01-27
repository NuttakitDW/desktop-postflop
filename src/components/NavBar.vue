<template>
  <nav
    class="sticky flex top-0 z-30 w-full h-10 shadow-lg px-4 justify-center bg-slate-800 text-gray-50"
  >
    <div class="flex relative w-full">
      <div class="flex items-center z-10">
        <div class="px-4">
          <span class="text-lg font-semibold">Desktop Postflop</span>
          <span class="pl-2 font-semibold text-gray-400">v0.2.7</span>
        </div>

        <!-- File Menu -->
        <div class="flex items-center gap-1 ml-2">
          <button
            class="flex px-3 h-8 items-center text-sm font-semibold rounded hover:bg-slate-700"
            @click="loadFile"
            title="Load saved game file"
          >
            <FolderOpenIcon class="w-4 h-4 mr-1.5" />
            Load
          </button>
          <button
            v-if="store.hasSolverRun"
            class="flex px-3 h-8 items-center text-sm font-semibold rounded hover:bg-slate-700"
            @click="saveFile"
            title="Save game to file"
          >
            <ArrowDownTrayIcon class="w-4 h-4 mr-1.5" />
            Save
          </button>
        </div>

        <!-- Abstraction indicator -->
        <div v-if="store.isAbstractionEnabled" class="flex items-center ml-4 px-2 py-1 bg-amber-600 rounded text-sm">
          <BeakerIcon class="w-4 h-4 mr-1" />
          Abstraction ({{ store.numBuckets?.[0] ?? 0 }}/{{ store.numBuckets?.[1] ?? 0 }} buckets)
        </div>

        <!-- Storage mode indicator -->
        <div v-if="store.isFileLoaded" class="flex items-center ml-2 px-2 py-1 bg-slate-600 rounded text-sm capitalize">
          {{ store.storageMode }}
        </div>
      </div>

      <div class="flex ml-auto h-full items-center z-10">
        <a
          href="https://github.com/b-inary/desktop-postflop"
          class="flex px-4 h-full items-center font-semibold hover:bg-slate-700"
          target="_blank"
        >
          <img
            src="../assets/GitHub-Mark-Light-32px.png"
            class="inline-block w-5 h-5 mr-2.5"
          />
          GitHub
        </a>
      </div>

      <div
        class="flex absolute w-full h-full left-0 top-0 gap-3 justify-center"
      >
        <button
          :class="
            'flex relative w-32 items-center justify-center font-semibold ' +
            'transition-colors hover:bg-slate-700 hover:text-blue-200 ' +
            (store.navView === 'solver' ? 'bg-slate-700 text-blue-200' : '')
          "
          @click="store.navView = 'solver'"
        >
          <ComputerDesktopIcon class="w-6 h-6" />
          <span class="pl-3">Solver</span>
        </button>
        <button
          :class="
            'flex relative w-32 items-center justify-center font-semibold ' +
            'transition-colors hover:bg-slate-700 hover:text-blue-200 ' +
            (store.navView === 'results' ? 'bg-slate-700 text-blue-200' : '')
          "
          @click="store.navView = 'results'"
        >
          <ChartBarIcon class="w-6 h-6" />
          <span class="pl-3">Results</span>
        </button>
      </div>
    </div>
  </nav>
</template>

<script setup lang="ts">
import { useStore, useConfigStore, useSavedConfigStore } from "../store";
import { ComputerDesktopIcon, ChartBarIcon, FolderOpenIcon, ArrowDownTrayIcon, BeakerIcon } from "@heroicons/vue/24/solid";
import { open, save } from "@tauri-apps/api/dialog";
import { gameLoadFile, gameSaveFile } from "../invokes";

const store = useStore();
const config = useConfigStore();
const savedConfig = useSavedConfigStore();

const loadFile = async () => {
  try {
    const path = await open({
      filters: [{ name: "Postflop", extensions: ["flop", "bin"] }],
    });

    if (typeof path !== "string") return;

    const result = await gameLoadFile(path);

    // Update store state
    store.isFileLoaded = true;
    store.loadedFileMemo = result.memo;
    store.isAbstractionEnabled = result.is_abstraction_enabled;
    store.numBuckets = result.num_buckets;
    store.storageMode = result.storage_mode;
    store.isSolverFinished = result.is_solved;

    // Update config with loaded board
    config.$patch({
      board: result.board,
      startingPot: result.starting_pot,
      effectiveStack: result.effective_stack,
    });

    savedConfig.$patch({
      board: result.board,
      startingPot: result.starting_pot,
      effectiveStack: result.effective_stack,
    });

    // Navigate to results
    store.navView = "results";

  } catch (e) {
    console.error("Failed to load file:", e);
    alert("Failed to load file: " + e);
  }
};

const saveFile = async () => {
  try {
    const path = await save({
      filters: [{ name: "Postflop", extensions: ["flop"] }],
      defaultPath: "game.flop",
    });

    if (typeof path !== "string") return;

    const memo = prompt("Enter memo (optional):", store.loadedFileMemo || "") || "";
    await gameSaveFile(path, memo, null);

    alert("File saved successfully!");

  } catch (e) {
    console.error("Failed to save file:", e);
    alert("Failed to save file: " + e);
  }
};
</script>
