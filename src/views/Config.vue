<template>
  <div class="p-2">
    <n-form
      ref="formRef"
      inline
      :label-width="80"
      :model="formValue"
      size="small"
    >
      <n-form-item label="Auto Start" path="autoStart">
        <n-switch
          v-model:value="formValue.autoStart"
          @update:value="handleAutoStartChange"
        ></n-switch>
      </n-form-item>
    </n-form>
  </div>
</template>

<script setup>
import { ref, onMounted } from "vue";
import { load } from "@tauri-apps/plugin-store";
import { enable, disable } from "@tauri-apps/plugin-autostart";
const formRef = ref(null);
const formValue = ref({
  autoStart: true,
});

const handleAutoStartChange = async (value) => {
  const store = await load("store.json", { autoSave: false });
  await store.set("autoStart", value);
  await store.save();
  if (value) {
    await enable();
  } else {
    await disable();
  }
};

onMounted(async () => {
  const store = await load("store.json", { autoSave: false });
  formValue.value.autoStart = await store.get("autoStart");
});
</script>
