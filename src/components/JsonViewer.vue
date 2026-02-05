<template>
  <div class="json-viewer-container">
    <div class="flex justify-between items-center mb-4">
      <div class="flex items-center gap-4">
        <h2 class="text-xl font-bold text-gray-800">JSON Viewer</h2>
        <!-- Mode Toggle -->
        <div class="flex bg-gray-200 rounded-lg p-1">
          <button
            @click="viewMode = 'metadata'"
            :class="[
              'px-3 py-1 rounded text-sm font-semibold transition-colors',
              viewMode === 'metadata'
                ? 'bg-white text-blue-600 shadow'
                : 'text-gray-600 hover:text-gray-800'
            ]"
          >
            Metadata
          </button>
          <button
            @click="switchToComplete"
            :disabled="!canViewComplete"
            :class="[
              'px-3 py-1 rounded text-sm font-semibold transition-colors',
              viewMode === 'complete'
                ? 'bg-white text-blue-600 shadow'
                : canViewComplete
                  ? 'text-gray-600 hover:text-gray-800'
                  : 'text-gray-400 cursor-not-allowed'
            ]"
          >
            Complete
            <span v-if="!canViewComplete" class="ml-1 text-xs">(Load file first)</span>
          </button>
        </div>
      </div>
      <div class="flex gap-2">
        <button
          @click="copyToClipboard"
          class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors flex items-center gap-2"
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
          </svg>
          Copy JSON
        </button>
        <button
          @click="exportToFile"
          class="px-4 py-2 bg-green-600 text-white rounded hover:bg-green-700 transition-colors flex items-center gap-2"
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
          </svg>
          Export
        </button>
      </div>
    </div>

    <!-- File Info Summary -->
    <div v-if="jsonData" class="bg-blue-50 border border-blue-200 rounded-lg p-4 mb-4">
      <div class="flex justify-between items-start mb-2">
        <h3 class="font-semibold text-blue-900">
          {{ viewMode === 'metadata' ? 'Metadata Only' : 'Complete Game Data' }}
        </h3>
        <span class="text-xs bg-blue-200 text-blue-800 px-2 py-1 rounded">
          {{ viewMode === 'metadata' ? '~500 bytes' : '~150 MB' }}
        </span>
      </div>
      <div class="grid grid-cols-2 gap-2 text-sm">
        <div><span class="font-medium">Storage Mode:</span> <span class="capitalize text-blue-700">{{ getConfigValue('storage_mode') }}</span></div>
        <div><span class="font-medium">Solved:</span> <span :class="getConfigValue('is_solved') ? 'text-green-600' : 'text-red-600'">{{ getConfigValue('is_solved') ? 'Yes' : 'No' }}</span></div>
        <div><span class="font-medium">Starting Pot:</span> {{ getConfigValue('starting_pot') }}</div>
        <div><span class="font-medium">Effective Stack:</span> {{ getConfigValue('effective_stack') }}</div>
        <div><span class="font-medium">Board:</span> {{ decodedBoard }}</div>
        <div v-if="viewMode === 'complete' && completeData"><span class="font-medium">OOP Range:</span> <span class="text-xs">{{ completeData.ranges.oop }}</span></div>
        <div v-if="viewMode === 'complete' && completeData"><span class="font-medium">IP Range:</span> <span class="text-xs">{{ completeData.ranges.ip }}</span></div>
        <div v-if="jsonData.memo || store.loadedFileMemo" class="col-span-2"><span class="font-medium">Memo:</span> {{ jsonData.memo || store.loadedFileMemo }}</div>
      </div>
      <!-- Warning for metadata mode -->
      <div v-if="viewMode === 'metadata'" class="mt-3 text-xs text-yellow-700 bg-yellow-50 p-2 rounded">
        ⚠️ Metadata only - does not include strategy, equity, or EV data. Switch to "Complete" mode for full data.
      </div>
      <!-- Info for complete mode -->
      <div v-if="viewMode === 'complete'" class="mt-3 text-xs p-2 rounded" :class="getConfigValue('is_solved') ? 'text-green-700 bg-green-50' : 'text-blue-700 bg-blue-50'">
        <span v-if="getConfigValue('is_solved')">✅ Complete data includes: Ranges, Strategy, Equity, EV, EQR, and all game state information.</span>
        <span v-else>ℹ️ Complete data includes: Ranges, Game Configuration, and Tree Structure. Strategy/Equity/EV will be available after solving.</span>
      </div>
    </div>

    <!-- JSON Display -->
    <div class="bg-gray-900 text-gray-100 rounded-lg p-4 overflow-auto" style="max-height: 600px;">
      <pre class="text-sm font-mono">{{ formattedJson }}</pre>
    </div>

    <!-- Copy Success Message -->
    <Transition name="fade">
      <div v-if="showCopySuccess" class="fixed bottom-4 right-4 bg-green-500 text-white px-6 py-3 rounded-lg shadow-lg">
        JSON copied to clipboard!
      </div>
    </Transition>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { useStore } from '../store';
import { gameInfo } from '../invokes';
import { save } from '@tauri-apps/api/dialog';
import { writeTextFile } from '@tauri-apps/api/fs';
import { exportCompleteJSON } from '../utils/exportComplete';

const store = useStore();
const showCopySuccess = ref(false);
const jsonData = ref<any>(null);
const completeData = ref<any>(null);
const viewMode = ref<'metadata' | 'complete'>('metadata');
const isLoadingComplete = ref(false);

// Computed property for button state
const canViewComplete = computed(() => {
  const canView = store.isFileLoaded;
  console.log('[JsonViewer] canViewComplete:', canView, 'isFileLoaded:', store.isFileLoaded);
  return canView;
});

// Card decoding
const ranks = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];
const suits = ['♣', '♦', '♥', '♠'];

const decodeCard = (cardNum: number): string => {
  const rank = ranks[Math.floor(cardNum / 4)];
  const suit = suits[cardNum % 4];
  return rank + suit;
};

const decodedBoard = computed(() => {
  if (viewMode.value === 'complete' && completeData.value) {
    return completeData.value.game_config.board_readable;
  }
  if (!jsonData.value || !jsonData.value.board) return '';
  return jsonData.value.board.map(decodeCard).join(' ');
});

const getConfigValue = (key: string) => {
  if (viewMode.value === 'complete' && completeData.value) {
    return completeData.value.game_config[key];
  }
  return jsonData.value ? jsonData.value[key] : '';
};

// Load JSON data
const loadJsonData = async () => {
  try {
    if (!store.isFileLoaded) {
      jsonData.value = null;
      return;
    }

    const info = await gameInfo();

    jsonData.value = {
      version: 1,
      memo: store.loadedFileMemo,
      is_solved: info.is_solved,
      storage_mode: info.storage_mode,
      board: info.board,
      starting_pot: info.starting_pot,
      effective_stack: info.effective_stack,
      exported_at: new Date().toISOString(),
      note: "This is a JSON preview of the binary game file. It contains metadata only, not the full solver state.",
      _decoded_info_for_reference: {
        board_readable: decodedBoard.value,
        encoding_formula: "rank * 4 + suit",
        ranks: "0=2, 1=3, ..., 9=T, 10=J, 11=Q, 12=A",
        suits: "0=♣ (Club), 1=♦ (Diamond), 2=♥ (Heart), 3=♠ (Spade)"
      }
    };
  } catch (e) {
    console.error('Failed to load JSON data:', e);
    alert('Failed to load JSON data: ' + e);
  }
};

const loadingStartTime = ref<number>(0);
const loadingElapsedTime = ref<number>(0);
let loadingInterval: number | null = null;

const formattedJson = computed(() => {
  if (viewMode.value === 'complete') {
    if (!completeData.value) {
      if (isLoadingComplete.value) {
        const elapsed = loadingElapsedTime.value;
        return `Loading complete data...\n\nElapsed time: ${elapsed}s\n\nThis may take 30-60 seconds for large games.\nPlease be patient - the export is processing in the background.`;
      }
      return 'No complete data loaded. Click "Complete" to load.';
    }
    return JSON.stringify(completeData.value, null, 2);
  }

  if (!jsonData.value) {
    return 'No file loaded. Please load a .flop or .bin file first.';
  }
  return JSON.stringify(jsonData.value, null, 2);
});

const switchToComplete = async () => {
  console.log('[JsonViewer] switchToComplete called');
  console.log('[JsonViewer] isFileLoaded:', store.isFileLoaded);
  console.log('[JsonViewer] isSolverFinished:', store.isSolverFinished);

  if (!store.isFileLoaded) {
    alert('Please load a game file first to view complete data.');
    return;
  }

  viewMode.value = 'complete';

  // Load complete data if not already loaded
  if (!completeData.value) {
    try {
      isLoadingComplete.value = true;
      loadingStartTime.value = Date.now();
      loadingElapsedTime.value = 0;

      // Start timer to show elapsed time
      loadingInterval = window.setInterval(() => {
        loadingElapsedTime.value = Math.floor((Date.now() - loadingStartTime.value) / 1000);
      }, 1000);

      completeData.value = await exportCompleteJSON();

      // Stop timer
      if (loadingInterval !== null) {
        clearInterval(loadingInterval);
        loadingInterval = null;
      }
      isLoadingComplete.value = false;
    } catch (e) {
      // Stop timer on error
      if (loadingInterval !== null) {
        clearInterval(loadingInterval);
        loadingInterval = null;
      }
      isLoadingComplete.value = false;
      console.error('Failed to load complete data:', e);
      alert('Failed to load complete data: ' + e);
      viewMode.value = 'metadata';
    }
  }
};

const copyToClipboard = async () => {
  try {
    await navigator.clipboard.writeText(formattedJson.value);
    showCopySuccess.value = true;
    setTimeout(() => {
      showCopySuccess.value = false;
    }, 2000);
  } catch (e) {
    console.error('Failed to copy:', e);
    alert('Failed to copy to clipboard: ' + e);
  }
};

const exportToFile = async () => {
  try {
    const defaultFilename = viewMode.value === 'complete'
      ? 'game-complete.json'
      : 'game-metadata.json';

    const path = await save({
      filters: [{ name: 'JSON', extensions: ['json'] }],
      defaultPath: defaultFilename,
    });

    if (typeof path !== 'string') return;

    // If exporting complete but not loaded yet, load it first
    if (viewMode.value === 'complete' && !completeData.value) {
      isLoadingComplete.value = true;
      loadingStartTime.value = Date.now();
      loadingElapsedTime.value = 0;

      // Start timer to show elapsed time
      loadingInterval = window.setInterval(() => {
        loadingElapsedTime.value = Math.floor((Date.now() - loadingStartTime.value) / 1000);
      }, 1000);

      completeData.value = await exportCompleteJSON();

      // Stop timer
      if (loadingInterval !== null) {
        clearInterval(loadingInterval);
        loadingInterval = null;
      }
      isLoadingComplete.value = false;
    }

    await writeTextFile(path, formattedJson.value);

    const fileSize = viewMode.value === 'complete' ? '~150 MB' : '~500 bytes';
    alert(`JSON exported successfully!\n\nFile: ${path}\nSize: ${fileSize}\nMode: ${viewMode.value === 'complete' ? 'Complete' : 'Metadata'}`);
  } catch (e) {
    // Stop timer on error
    if (loadingInterval !== null) {
      clearInterval(loadingInterval);
      loadingInterval = null;
    }
    isLoadingComplete.value = false;
    console.error('Failed to export JSON:', e);
    alert('Failed to export JSON: ' + e);
  }
};

// Watch for file loading changes
import { watch } from 'vue';
watch(() => store.isFileLoaded, (newVal) => {
  if (newVal) {
    loadJsonData();
  } else {
    jsonData.value = null;
    completeData.value = null;
    viewMode.value = 'metadata';
  }
}, { immediate: true });

// Reset complete data when switching files
watch(() => store.loadedFileMemo, () => {
  completeData.value = null;
  viewMode.value = 'metadata';
});
</script>

<style scoped>
.json-viewer-container {
  @apply p-6;
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
