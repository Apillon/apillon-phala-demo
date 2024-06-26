<template>
  <div :class="$style.dropzone">
    <div
      :class="[
        $style.dropzoneContainer,
        {
          'pointer-events-none':
            state === EncryptionState.IDLE || state === EncryptionState.VERIFYING_OWNER,
        },
        { 'cursor-pointer': state === EncryptionState.WALLET_CONNECTED },
        {
          'cursor-pointer border border-dashed border-green': state === EncryptionState.DECRYPTED,
        },
        {
          'pointer-events-none border border-dashed border-pink': state === EncryptionState.ERROR,
        },
      ]"
      @click="emit('verify')"
    >
      <input
        ref="fileRef"
        type="file"
        name="file"
        id="fileInput"
        accept=".json"
        :class="$style.hiddenInput"
      />

      <label for="fileInput" class="pb-10 pointer-events-none" :class="$style.fileLabel">
        <h4 v-if="state === EncryptionState.VERIFYING_OWNER" class="mt-12 mb-4">
          <Spinner />
        </h4>
        <h4 v-else-if="state === EncryptionState.DECRYPTED" class="mt-12 mb-4 text-green">
          Correct NFT key.
        </h4>
        <h4 v-else-if="state === EncryptionState.ERROR" class="mt-12 mb-4 text-pink">
          Wrong NFT key.
        </h4>
        <h4 v-else class="mt-12 mb-4">Unlock encrypted files in seconds</h4>

        <div class="h-60 w-80 mx-auto my-4">
          <animated-image
            v-if="state === EncryptionState.DECRYPTED"
            image-url-base="/images/animations/unlocked/schrod-cat"
            :frame-count="9"
            :extra-stopped-frames="{ 8: 9000000 }"
          />
          <animated-image
            v-else-if="state === EncryptionState.ERROR"
            image-url-base="/images/animations/locked/schrod-cat"
            :frame-count="11"
            :extra-stopped-frames="{ 10: 9000000 }"
          />
          <animated-image
            v-else
            image-url-base="/images/animations/upload/schrod-cat"
            :frame-count="7"
          />
        </div>

        <p v-if="state === EncryptionState.ERROR" class="mb-0">
          Your curiosity didn't unlock the files. <br />
          (But it didn't kill the cat, either.)
        </p>
        <p v-else class="mb-0">Click to decrypt file</p>
      </label>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { EncryptionState } from '~/lib/types/general.types';

const props = defineProps({
  state: { type: Number, default: EncryptionState.IDLE },
});

const emit = defineEmits(['verify']);
</script>

<style lang="postcss" module>
.dropzone {
  @apply flex justify-center items-center w-full text-center;
}

.dropzoneContainer {
  @apply p-8 w-full min-h-[168px] flex flex-col justify-center rounded-[20px] hover:bg-bg-dark;
  transition: all 0.3s;

  &.borderSvg {
    background-image: url("data:image/svg+xml,%3csvg width='100%25' height='100%25' xmlns='http://www.w3.org/2000/svg'%3e%3crect width='100%25' height='100%25' fill='none' stroke='%23F0F2DA' stroke-width='1' stroke-dasharray='4' stroke-dashoffset='0' rx='20' stroke-linecap='square'/%3e%3c/svg%3e");
    background-size: 100% 100%;
  }
}

.hiddenInput {
  opacity: 0;
  overflow: hidden;
  position: absolute;
  width: 1px;
  height: 1px;
}

.fileLabel {
  @apply cursor-pointer mb-1;
}

.previewContainer {
  display: flex;
  margin-top: 1rem;
}
</style>
