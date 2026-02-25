/**
 * Audius SDK initialization and helpers.
 * TODO: Initialize with actual Audius SDK
 */

// import { sdk } from '@audius/sdk';

// let audiusInstance: ReturnType<typeof sdk> | null = null;

// export function getAudiusSdk() {
//   if (!audiusInstance) {
//     audiusInstance = sdk({
//       appName: 'Preset.gg',
//     });
//   }
//   return audiusInstance;
// }

export function formatDuration(seconds: number): string {
  const mins = Math.floor(seconds / 60);
  const secs = Math.floor(seconds % 60);
  return `${mins}:${secs.toString().padStart(2, '0')}`;
}
