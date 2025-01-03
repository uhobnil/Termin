<script setup>
import { onMounted, h, ref } from "vue";
import { RouterView } from "vue-router";
import { listen } from "@tauri-apps/api/event";
import {
  isPermissionGranted,
  requestPermission,
  sendNotification,
} from "@tauri-apps/plugin-notification";

import { useRouter } from "vue-router";
import {
  EventSchedule as ScheduleIcon,
  CalendarSettings as ConfigIcon,
} from "@vicons/carbon";
import { NIcon } from "naive-ui";
import provider from "./Provider.vue";

const router = useRouter();
const collapsed = ref(true);

function renderIcon(icon) {
  return () => h(NIcon, null, { default: () => h(icon) });
}

const menuOptions = [
  {
    label: "Schedule",
    key: "/",
    icon: renderIcon(ScheduleIcon),
  },
  {
    label: "Config",
    key: "/config",
    icon: renderIcon(ConfigIcon),
  },
];

function handleMenuUpdateValue(value) {
  router.push(value);
}

onMounted(async () => {
  listen("notify", async (event) => {
    const schedules = event.payload;
    let permissionGranted = await isPermissionGranted();
    if (!permissionGranted) {
      const permission = await requestPermission();
      permissionGranted = permission === "granted";
    }
    if (permissionGranted) {
      sendNotification({
        title: "Warning:",
        body: schedules.map((item) => item.content).join("\n"),
      });
    }
  });
});
</script>

<template>
  <provider>
    <n-layout has-sider>
      <n-layout-sider
        bordered
        :collapsed="collapsed"
        :collapsed-width="48"
        collapse-mode="width"
        show-trigger
        @collapse="collapsed = true"
        @expand="collapsed = false"
      >
        <n-menu
          :collapsed="collapsed"
          :collapsed-width="48"
          :collapsed-icon-size="22"
          :options="menuOptions"
          @update:value="handleMenuUpdateValue"
        />
      </n-layout-sider>
      <n-layout-content>
        <router-view>
          <template #default="{ Component }">
            <component :is="Component" />
          </template>
        </router-view>
      </n-layout-content>
    </n-layout>
  </provider>
</template>

<style scoped></style>
