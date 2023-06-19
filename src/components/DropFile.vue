<template>
    <div :class="$style.dropzone">
      <div :class="$style.dropzoneContainer" @dragover="dragover" @dragleave="dragleave" @drop="drop">
        <input
          ref="fileRef"
          type="file"
          name="file"
          id="fileInput"
          accept=".json"
          :class="$style.hiddenInput"
          @change="onChange"
        />
  
        <label for="fileInput" :class="$style.fileLabel">
          <div v-if="file">
            <h4 class="mb-1">{{ file.name }}</h4>
            <p class="mb-0">&nbsp;</p>
          </div>
          <div v-else>
            <h4 class="mb-1">Upload file</h4>
            <p v-if="isDragging" class="mb-0">Release to drop files here.</p>
            <p v-else class="mb-0">Drag & drop or click to upload.</p>
          </div>
        </label>
      </div>
    </div>
  </template>
  
  <script lang="ts" setup>
  import { defineComponent, ref } from "vue";
  
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
    @apply p-2 w-full min-h-[168px] flex flex-col justify-center bg-bg-light border-1 border-bg-lighter;
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
  