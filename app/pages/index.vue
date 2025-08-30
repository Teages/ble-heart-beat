<script setup lang="ts">
const { isApple } = useDevice()
const toast = useToast()

const {
  isSupported,
  isConnected,
  device,
  requestDevice,
  heartRate,
  batteryLevel,
} = useHeartRateBLE()

// Search and connect to BLE device
async function connectDevice() {
  if (!isSupported.value) {
    toast.add({
      title: 'Browser Not Supported',
      description: 'Web Bluetooth API is not supported by your browser. Please use a supported browser (like Chrome or Edge) and ensure experimental features are enabled.',
      color: 'error',
    })
    return
  }

  try {
    await requestDevice()
  }
  catch (error: any) {
    toast.add({
      title: 'Connection Failed',
      description: `Connection failed: ${error.message}`,
      color: 'error',
    })
  }
}

// Manually disconnect
function disconnectDevice() {
  if (device.value?.gatt?.connected) {
    device.value.gatt.disconnect()

    if (isApple) {
      // on MacOS, the Bluetooth connection may not be properly closed
      // so we need to reload the page
      location.reload()
    }
  }
}
</script>

<template>
  <div class="min-h-screen flex justify-center items-center">
    <UCard v-if="!isConnected" class="w-full max-w-md">
      <template #header>
        <div class="flex items-center justify-center">
          <h2 class="text-xl font-semibold">
            Heart Rate Monitor
          </h2>
        </div>
      </template>

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
          @click="connectDevice"
        >
          Search Device
        </UButton>
      </div>
    </UCard>

    <UCard v-else class="w-full max-w-md">
      <template #header>
        <div class="flex items-center justify-between">
          <h2 class="text-xl font-semibold">
            Heart Rate Monitor
          </h2>
          <UIcon name="i-heroicons-signal" class="w-6 h-6 text-green-500" />
        </div>
      </template>

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
            <span class="font-medium">{{ device?.name || 'Unknown' }}</span>
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
            @click="disconnectDevice"
          >
            Disconnect
          </UButton>
        </div>
      </div>
    </UCard>
  </div>
</template>
