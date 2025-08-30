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
    <UCard class="w-full max-w-md max-[28rem]:border-0">
      <template #header>
        <div class="flex items-center justify-between">
          <h2 class="text-xl font-semibold">
            Heart Rate Monitor
          </h2>
          <UIcon
            v-if="isConnected"
            name="i-heroicons-signal"
            class="w-6 h-6 text-green-500"
          />
        </div>
      </template>

      <StatePendingCard
        v-if="!isConnected"
        class="w-full max-w-md"
        @connect="connectDevice"
      />

      <StateConnectedCard
        v-else
        class="w-full max-w-md"
        :heart-rate="heartRate"
        :battery-level="batteryLevel"
        :device-name="device?.name"
        @disconnect="disconnectDevice"
      />
    </UCard>
  </div>
</template>
