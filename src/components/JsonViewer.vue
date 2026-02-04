<template>
  <div class="json-viewer-container">
    <div class="flex justify-between items-center mb-4">
      <h2 class="text-xl font-bold text-gray-800">JSON Template</h2>
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
          Export JSON
        </button>
      </div>
    </div>

    <!-- File Info Summary -->
    <div v-if="jsonData" class="bg-blue-50 border border-blue-200 rounded-lg p-4 mb-4">
      <h3 class="font-semibold text-blue-900 mb-2">File Information</h3>
      <div class="grid grid-cols-2 gap-2 text-sm">
        <div><span class="font-medium">Storage Mode:</span> <span class="capitalize text-blue-700">{{ jsonData.storage_mode }}</span></div>
        <div><span class="font-medium">Solved:</span> <span :class="jsonData.is_solved ? 'text-green-600' : 'text-red-600'">{{ jsonData.is_solved ? 'Yes' : 'No' }}</span></div>
        <div><span class="font-medium">Starting Pot:</span> {{ jsonData.starting_pot }}</div>
        <div><span class="font-medium">Effective Stack:</span> {{ jsonData.effective_stack }}</div>
        <div><span class="font-medium">Board:</span> {{ decodedBoard }}</div>
        <div v-if="jsonData.memo"><span class="font-medium">Memo:</span> {{ jsonData.memo }}</div>
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

const store = useStore();
const showCopySuccess = ref(false);
const jsonData = ref<any>(null);

// Card decoding
const ranks = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];
const suits = ['♣', '♦', '♥', '♠'];

const decodeCard = (cardNum: number): string => {
  const rank = ranks[Math.floor(cardNum / 4)];
  const suit = suits[cardNum % 4];
  return rank + suit;
};

const decodedBoard = computed(() => {
  if (!jsonData.value || !jsonData.value.board) return '';
  return jsonData.value.board.map(decodeCard).join(' ');
});

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

const formattedJson = computed(() => {
  if (!jsonData.value) {
    return 'No file loaded. Please load a .flop or .bin file first.';
  }
  return JSON.stringify(jsonData.value, null, 2);
});

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
    const path = await save({
      filters: [{ name: 'JSON', extensions: ['json'] }],
      defaultPath: 'game-template.json',
    });

    if (typeof path !== 'string') return;

    await writeTextFile(path, formattedJson.value);
    alert('JSON exported successfully!');
  } catch (e) {
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
  }
}, { immediate: true });
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
