<template>
  <n-button
    class="fixed right-4 bottom-4 z-1000"
    type="primary"
    @click="showModal = true"
    circle
  >
    <template #icon>
      <n-icon><Add /></n-icon>
    </template>
  </n-button>
  <n-message-provider>
    <n-modal
      v-model:show="showModal"
      :mask-closable="false"
      :close-on-esc="false"
      :show-icon="false"
      preset="dialog"
      :title="editingId ? 'Edit Schedule' : 'Create Schedule'"
      positive-text="Confirm"
      negative-text="Cancel"
      @close="onNegativeClick"
      @positive-click="onPositiveClick"
      @negative-click="onNegativeClick"
    >
      <n-form
        ref="formRef"
        :model="model"
        :rules="rules"
        label-placement="left"
        label-width="auto"
        require-mark-placement="right-hanging"
        :size="size"
        :style="{
          maxWidth: '640px',
        }"
      >
        <n-form-item label="Date" path="date">
          <n-date-picker v-model:value="model.date" type="datetime" />
        </n-form-item>
        <n-form-item label="Content" path="content">
          <n-input
            v-model:value="model.content"
            placeholder="Textarea"
            type="textarea"
            :autosize="{
              minRows: 3,
              maxRows: 5,
            }"
          />
        </n-form-item>
        <n-form-item label="Remind" path="remind">
          <n-switch v-model:value="model.remind" />
        </n-form-item>
        <n-form-item label="Repeat" path="repeat">
          <n-select
            v-model:value="model.repeat"
            placeholder="Select"
            :options="generalOptions"
          />
        </n-form-item>
      </n-form>
    </n-modal>
  </n-message-provider>
  <n-split :size="0.8">
    <template #1>
      <div class="h-full flex flex-col p-2 box-border">
        <n-calendar
          class="flex-1 overflow-auto"
          v-model:value="value"
          #="{ year, month, date }"
          :is-date-disabled="isDateDisabled"
          @update:value="handleUpdateValue"
          @panel-change="handlePanelChange"
        >
          <div
            class="w-full truncate text-sm"
            v-for="s in buildDaySchedules(year, month, date)"
          >
            {{ s.content }}
          </div>
        </n-calendar>
      </div>
    </template>
    <template #2>
      <div class="flex-1 p-2">
        <p class="text-xl">{{ dayjs(viewDate).format("YYYY-MM-DD") }}</p>
        <div
          v-for="s in dateSchedules"
          :key="s.id"
          class="flex items-center justify-between w-full text-sm cursor-pointer rounded-lg"
          v-if="dateSchedules.length > 0"
        >
          <div class="truncate">{{ s.content }}</div>
          <div class="flex gap-2">
            <n-button quaternary circle size="small" @click="handleEdit(s)">
              <template #icon>
                <n-icon><Edit /></n-icon>
              </template>
            </n-button>
            <n-button
              quaternary
              circle
              size="small"
              type="error"
              @click="handleDelete(s.id)"
            >
              <template #icon>
                <n-icon><Delete /></n-icon>
              </template>
            </n-button>
          </div>
        </div>
        <n-empty v-else class="flex-1 justify-center">
          <template #icon> </template>
        </n-empty>
      </div>
    </template>
  </n-split>
</template>

<script setup>
import { Add, Edit, Delete } from "@vicons/carbon";
import { ref, onMounted } from "vue";
import dayjs from "dayjs";
import { invoke } from "@tauri-apps/api/core";
import { useMessage } from "naive-ui";
const emit = defineEmits(["created"]);
const message = useMessage();

const schedules = ref([]);
const viewDate = ref(dayjs().valueOf());
const dateSchedules = ref([]);
const formRef = ref(null);
const value = ref(dayjs().valueOf());
const showModal = ref(false);
const model = ref({
  content: "",
  repeat: "ONCE",
  remind: false,
  date: null,
});

const generalOptions = [
  {
    value: "ONCE",
    label: "Once",
  },
  {
    value: "DAILY",
    label: "Daily",
  },
  {
    value: "WEEKLY",
    label: "Weekly",
  },
  {
    value: "MONTHLY",
    label: "Monthly",
  },
  {
    value: "YEARLY",
    label: "Yearly",
  },
];

const editingId = ref(null);

function onNegativeClick() {
  showModal.value = false;
  editingId.value = null;
  model.value = {
    content: "",
    repeat: "ONCE",
    remind: false,
    date: null,
  };
}
async function onPositiveClick() {
  try {
    const data = {
      content: model.value.content,
      date: dayjs(model.value.date).unix(),
      remind: model.value.remind,
      repeat: model.value.repeat,
    };

    if (editingId.value) {
      await invoke("update_schedule", {
        id: editingId.value,
        data,
      });
      message.success("Update Success!");
    } else {
      await invoke("create_schedule", { data });
      message.success("Create Success!");
    }

    showModal.value = false;
    editingId.value = null;
    model.value = {
      content: "",
      repeat: "ONCE",
      remind: false,
      date: null,
    };
    await getSchedulesByMonth(
      dayjs(value.value).get("year"),
      dayjs(value.value).get("month") + 1
    );
    updateViewSchedules();
  } catch (error) {
    message.error(error);
  }
}

async function getSchedulesByMonth(year, month) {
  try {
    schedules.value = await invoke("get_schedule_by_month", {
      year,
      month,
    });
    console.log(schedules.value);
  } catch (error) {
    console.log(error);
    message.error(error);
  }
}

function buildDaySchedules(year, month, date) {
  return schedules.value.filter((item) => {
    return dayjs(item.date * 1000)
      .startOf("day")
      .isSame(dayjs(`${year}-${month}-${date}`));
  });
}

function handlePanelChange({ year, month }) {
  getSchedulesByMonth(year, month);
}

function updateViewSchedules() {
  dateSchedules.value = viewDate.value
    ? schedules.value.filter((item) => {
        return dayjs(item.date * 1000)
          .startOf("day")
          .isSame(dayjs(viewDate.value));
      })
    : [];
}

function handleUpdateValue(timestamp, info) {
  viewDate.value = timestamp;
  updateViewSchedules();
}
function isDateDisabled(timestamp) {
  return false;
}

function handleEdit(schedule) {
  editingId.value = schedule.id;
  model.value = {
    content: schedule.content,
    date: schedule.date * 1000,
    remind: schedule.remind,
    repeat: schedule.repeat,
  };
  showModal.value = true;
}

async function handleDelete(id) {
  try {
    await invoke("delete_schedule", { id });
    message.success("Delete Success!");
    await getSchedulesByMonth(
      dayjs(value.value).get("year"),
      dayjs(value.value).get("month") + 1
    );
    updateViewSchedules();
  } catch (error) {
    message.error(error);
  }
}

onMounted(() => {
  getSchedulesByMonth(dayjs().get("year"), dayjs().get("month") + 1);
});
</script>
