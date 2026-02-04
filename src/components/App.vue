<template>
  <div class="min-w-[1080px]" :style="{ height: clientHeight + 'px' }">
    <!-- Loading Overlay -->
    <Transition name="fade">
      <div
        v-if="store.isFileLoading"
        class="fixed inset-0 z-50 flex flex-col items-center justify-center bg-slate-900/80 backdrop-blur-sm"
      >
        <div class="flex flex-col items-center gap-4">
          <!-- Spinner -->
          <svg
            class="animate-spin h-12 w-12 text-blue-500"
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
          >
            <circle
              class="opacity-25"
              cx="12"
              cy="12"
              r="10"
              stroke="currentColor"
              stroke-width="4"
            ></circle>
            <path
              class="opacity-75"
              fill="currentColor"
              d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
            ></path>
          </svg>
          <!-- Loading Message -->
          <div class="text-white text-lg font-medium">
            {{ store.fileLoadingMessage || 'Loading...' }}
          </div>
          <div class="text-gray-400 text-sm">
            Please wait while the solution file is being loaded
          </div>
        </div>
      </div>
    </Transition>

    <NavBar />

    <div
      v-show="store.navView === 'solver'"
      class="flex w-full mx-auto max-w-screen-xl"
      style="height: calc(100% - 2.5rem)"
    >
      <SideBar style="height: calc(100% - 2rem)" />

      <div
        class="flex-grow my-4 px-6 pt-2 overflow-y-auto"
        style="height: calc(100% - 2rem)"
      >
        <div class="flex">
          <div
            :class="
              'mb-5 pl-2 pr-3 pb-0.5 text-lg font-bold border-l-8 border-b-2 ' +
              'border-blue-600 rounded rounded-br-none'
            "
          >
            {{ header }}
          </div>
        </div>

        <div v-show="store.sideView === 'oop-range'">
          <RangeEditor :player="0" />
        </div>
        <div v-show="store.sideView === 'ip-range'">
          <RangeEditor :player="1" />
        </div>
        <div v-show="store.sideView === 'board'">
          <BoardSelector />
        </div>
        <div v-show="store.sideView === 'tree-config'">
          <TreeConfig />
        </div>
        <div v-show="store.sideView === 'bunching'">
          <BunchingEffect />
        </div>
        <div v-show="store.sideView === 'run-solver'">
          <RunSolver />
        </div>
        <div v-show="store.sideView === 'json-viewer'">
          <JsonViewer />
        </div>
        <div v-if="store.sideView === 'about'">
          <AboutPage />
        </div>
      </div>
    </div>

    <div
      v-show="store.navView === 'results'"
      style="height: calc(max(100%, 720px) - 2.5rem)"
    >
      <ResultViewer />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from "vue";
import { useStore } from "../store";

import NavBar from "./NavBar.vue";
import SideBar from "./SideBar.vue";
import RangeEditor from "./RangeEditor.vue";
import BoardSelector from "./BoardSelector.vue";
import TreeConfig from "./TreeConfig.vue";
import BunchingEffect from "./BunchingEffect.vue";
import RunSolver from "./RunSolver.vue";
import JsonViewer from "./JsonViewer.vue";
import AboutPage from "./AboutPage.vue";
import ResultViewer from "./ResultViewer.vue";

const store = useStore();
const header = computed(() => store.headers[store.sideView].join(" > "));

const clientHeight = ref(0);
const updateClientHeight = () => {
  clientHeight.value = document.documentElement.clientHeight - 0.01;
};

updateClientHeight();
window.addEventListener("resize", updateClientHeight);
</script>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
