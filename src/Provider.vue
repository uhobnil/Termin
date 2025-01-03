<script setup></script>

<template>
  <n-config-provider>
    <n-loading-bar-provider>
      <n-modal-provider>
        <n-dialog-provider>
          <n-notification-provider>
            <n-message-provider>
              <slot />
            </n-message-provider>
          </n-notification-provider>
        </n-dialog-provider>
      </n-modal-provider>
    </n-loading-bar-provider>
  </n-config-provider>
</template>
