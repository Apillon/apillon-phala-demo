<template>
  <div :class="$style.dropzone">
    <div
      :class="[$style.dropzoneContainer, { '!bg-bg-light': isDragging || file }]"
      @dragover="dragover"
      @dragleave="dragleave"
      @drop="drop"
    >
      <input
        ref="fileRef"
        type="file"
        name="file"
        id="fileInput"
        accept=".json"
        :class="$style.hiddenInput"
        @change="onChange"
      />

      <label for="fileInput" class="pb-10 pointer-events-none" :class="$style.fileLabel">
        <div class="mt-8 mb-4 w-80 mx-auto">
          <animated-image image-url-base="/images/animations/upload/schrod-cat" :frame-count="7" />
          <!-- Animation locked 
          <animated-image
            image-url-base="/images/animations/locked/schrod-cat"
            :frame-count="11"
            :extra-stopped-frames="{ 10: 9000000 }"
          />
          -->
          <!-- Animation unlocked 
          <animated-image
            image-url-base="/images/animations/unlocked/schrod-cat"
            :frame-count="9"
            :extra-stopped-frames="{ 8: 9000000 }"
          />
          --></div>

        <p v-if="file" class="mb-0">{{ file.name }}</p>
        <p v-else-if="isDragging" class="mb-0">Release to drop files here.</p>
        <p v-else class="mb-0">Drag & drop your NFT key here.</p>
      </label>
    </div>
  </div>
</template>

<script lang="ts" setup>
const emit = defineEmits(['uploaded']);
const isDragging = ref<boolean>(false);
const file = ref<File | null>();
const fileRef = ref<HTMLInputElement>();
const $style = useCssModule();

function onChange() {
  const files = fileRef.value?.files
    ? Array.from(fileRef.value.files).filter(item => item.type.includes('json'))
    : [];

  if (files && files.length > 0 && files[0]) {
    file.value = files[0];
    parseUploadedFile(file.value);
  }
}
function dragover(e: DragEvent) {
  e.preventDefault();
  isDragging.value = true;
}
function dragleave() {
  isDragging.value = false;
}
function drop(e: DragEvent) {
  e.preventDefault();
  if (fileRef.value) {
    fileRef.value.files = e.dataTransfer?.files || null;
    onChange();
  }
  isDragging.value = false;
}

function parseUploadedFile(file: File) {
  let reader = new FileReader();
  reader.onload = (ev: ProgressEvent<FileReader>) => {
    if (!!ev?.target?.result) {
      emit('uploaded', ev.target.result.toString());
    } else {
      console.warn('CSV file is empty or is not valid!');
    }
  };
  reader.readAsText(file);
}
</script>

<style lang="postcss" module>
.dropzone {
  @apply flex justify-center items-center w-full text-center;
}

.dropzoneContainer {
  @apply p-2 w-full min-h-[168px] flex flex-col justify-center rounded-[20px] bg-bg-dark;
  background-image: url("data:image/svg+xml,%3csvg width='100%25' height='100%25' xmlns='http://www.w3.org/2000/svg'%3e%3crect width='100%25' height='100%25' fill='none' stroke='%23313442' stroke-width='1' stroke-dasharray='4' stroke-dashoffset='0' stroke-linecap='square'/%3e%3c/svg%3e");
  background-size: 100% 100%;
  transition: all 0.3s;
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
