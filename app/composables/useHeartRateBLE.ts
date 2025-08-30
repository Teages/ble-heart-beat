/// <reference types="web-bluetooth" />

import type { UseBluetoothReturn } from '@vueuse/core'

// Heart rate service and characteristic UUIDs
const HEART_RATE_SERVICE_UUID = 0x180D
const HEART_RATE_MEASUREMENT_UUID = 0x2A37
const BATTERY_SERVICE_UUID = 0x180F
const BATTERY_LEVEL_UUID = 0x2A19

export function useHeartRateBLE(): UseHeartRateBLEReturn {
  const bluetooth = useBluetooth({
    filters: [{ services: [HEART_RATE_SERVICE_UUID] }],
    optionalServices: [BATTERY_SERVICE_UUID],
  })

  const heartRate = ref<number | null>(null)
  const batteryLevel = ref<number | null>(null)

  function reset() {
    heartRate.value = null
    batteryLevel.value = null
  }

  function handleHeartRateChangeEvent(event: Event) {
    const target = event.target as any
    const value = target.value!

    // Parse heart rate data (according to Bluetooth spec)
    const flags = value.getUint8(0)
    const is16Bit = flags & 0x01

    let heartRateValue: number
    if (is16Bit) {
      heartRateValue = value.getUint16(1, true) // little endian
    }
    else {
      heartRateValue = value.getUint8(1)
    }

    heartRate.value = heartRateValue
  }
  function handleBatteryLevelChangeEvent(event: Event) {
    const target = event.target as any
    const value = target.value!

    // Battery level is a single byte
    batteryLevel.value = value.getUint8(0)
  }

  watch(bluetooth.server, async (server, _, onCleanup) => {
    if (!server) {
      return reset()
    }

    // heart rate
    const heartRateCharacteristic = await tryGetResult(async () => {
      const heartRateService = await server
        .getPrimaryService(HEART_RATE_SERVICE_UUID)
      const heartRateCharacteristic = await heartRateService
        .getCharacteristic(HEART_RATE_MEASUREMENT_UUID)
      await heartRateCharacteristic?.startNotifications()

      return heartRateCharacteristic
    })
    if (!heartRateCharacteristic) {
      return // TODO: error handling
    }
    const stopHeartRateListener = useEventListener(
      heartRateCharacteristic,
      'characteristicvaluechanged',
      handleHeartRateChangeEvent,
      { passive: true },
    )

    // battery level
    const batteryLevelCharacteristic = await tryGetResult(async () => {
      const batteryService = await server.getPrimaryService(BATTERY_SERVICE_UUID)
      const batteryLevelCharacteristic = await batteryService.getCharacteristic(BATTERY_LEVEL_UUID)
      await batteryLevelCharacteristic.startNotifications()
      return batteryLevelCharacteristic
    })
    const stopBatteryLevelListener = useEventListener(
      batteryLevelCharacteristic,
      'characteristicvaluechanged',
      handleBatteryLevelChangeEvent,
      { passive: true },
    )

    onCleanup(() => {
      stopHeartRateListener()
      stopBatteryLevelListener()
    })
  })

  return {
    ...bluetooth,
    heartRate,
    batteryLevel,
  }
}

async function tryGetResult<T>(fn: () => Promise<T>): Promise<T | null> {
  try {
    return await fn()
  }
  catch {
    return null
  }
}

interface UseHeartRateBLEReturn extends UseBluetoothReturn {
  heartRate: Ref<number | null>
  batteryLevel: Ref<number | null>
}
