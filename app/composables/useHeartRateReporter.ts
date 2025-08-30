import { invoke } from '@tauri-apps/api/core'

/**
 * Composable for reporting heart rate data to Tauri backend
 */
export function useHeartRateReporter() {
  /**
   * Report heart rate value to the Rust backend
   * @param heartRate Heart rate value in BPM
   */
  async function reportHeartRate(heartRate: number): Promise<void> {
    if (heartRate <= 0) {
      return
    }

    try {
      await invoke('update_heart_beat', { heartRate })
    }
    catch (error) {
      console.error('Failed to report heart rate:', error)
      throw error
    }
  }

  return {
    reportHeartRate,
  }
}
