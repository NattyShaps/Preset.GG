/**
 * General utility functions.
 */

import { type ClassValue, clsx } from 'clsx';

/**
 * Merge Tailwind class names (wrapper for clsx).
 * If you add tailwind-merge later, integrate it here.
 */
export function cn(...inputs: ClassValue[]) {
  return clsx(inputs);
}

/**
 * Format a duration in seconds to mm:ss.
 */
export function formatTime(seconds: number): string {
  const mins = Math.floor(seconds / 60);
  const secs = Math.floor(seconds % 60);
  return `${mins}:${secs.toString().padStart(2, '0')}`;
}

/**
 * Truncate a string to a maximum length.
 */
export function truncate(str: string, maxLength: number): string {
  if (str.length <= maxLength) return str;
  return str.slice(0, maxLength - 3) + '...';
}
