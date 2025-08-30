<script setup lang="ts">
const emit = defineEmits<{
  (e: 'connect'): void
}>()
</script>

<template>
  <div class="text-center space-y-6">
    <div class="flex justify-center">
      <div class="w-16 h-16 bg-gray-100 dark:bg-gray-800 rounded-full flex items-center justify-center">
        <UIcon name="i-heroicons-heart" class="w-8 h-8 text-gray-400" />
      </div>
    </div>

    <div>
      <h3 class="text-lg font-medium mb-2">
        Connect Your Heart Rate Device
      </h3>
      <p class="text-gray-600 dark:text-gray-400 text-sm">
        Make sure your heart rate monitor is turned on and ready to pair.
      </p>
    </div>

    <UButton
      color="primary"
      size="lg"
      block
      icon="i-heroicons-magnifying-glass"
      @click="emit('connect')"
    >
      Search Device
    </UButton>
  </div>
</template>
