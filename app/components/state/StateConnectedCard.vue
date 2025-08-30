<script setup lang="ts">
const {
  heartRate,
  batteryLevel,
  deviceName,
} = defineProps<{
  heartRate: number | null | undefined
  batteryLevel: number | null | undefined
  deviceName: string | null | undefined
}>()
const emit = defineEmits<{
  disconnect: []
}>()
</script>

<template>
  <div class="text-center space-y-6">
    <!-- Heart Rate Display -->
    <div class="flex gap-2 justify-center items-center text-red-500">
      <UIcon name="heroicons:heart-solid" class="inline-block w-18 h-18" />
      <div class="text-7xl font-bold">
        {{ heartRate ?? '--' }}
      </div>
    </div>

    <!-- Device Info -->
    <div class="bg-gray-50 dark:bg-gray-800 rounded-lg p-4">
      <div class="flex items-center justify-between text-sm">
        <span class="text-gray-600 dark:text-gray-400">Device:</span>
        <span class="font-medium">{{ deviceName || 'Unknown' }}</span>
      </div>

      <div v-if="batteryLevel !== null" class="flex items-center justify-between text-sm mt-2">
        <span class="text-gray-600 dark:text-gray-400">Battery:</span>
        <div class="flex items-center space-x-1">
          <UIcon name="i-heroicons-battery-100" class="w-4 h-4" />
          <span class="font-medium">{{ batteryLevel }}%</span>
        </div>
      </div>
    </div>

    <!-- Actions -->
    <div class="space-y-2">
      <UButton
        color="error"
        variant="outline"
        size="lg"
        block
        icon="i-heroicons-x-circle"
        @click="emit('disconnect')"
      >
        Disconnect
      </UButton>
    </div>
  </div>
</template>
