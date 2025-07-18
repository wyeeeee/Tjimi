/**
 * Formats a Unix timestamp to a localized date and time string
 */
export const formatTimestamp = (timestamp: number): string => {
  if (!timestamp) return "N/A";
  try {
    return new Date(timestamp * 1000).toLocaleString();
  } catch {
    return "Invalid date";
  }
};

/**
 * Formats seconds into a human-readable time elapsed string
 */
export const formatTimeElapsed = (seconds: number): string => {
  if (!seconds && seconds !== 0) return "unknown";
  if (seconds < 60) return `${seconds} sec`;
  if (seconds < 3600)
    return `${Math.floor(seconds / 60)} min ${seconds % 60} sec`;
  return `${Math.floor(seconds / 3600)} hr ${Math.floor(
    (seconds % 3600) / 60,
  )} min`;
};

/**
 * Masks a token for display, showing only first and last few characters
 */
export const maskToken = (token: string): string => {
  if (!token || token.length <= 8) return "••••••••";
  return token.substring(0, 4) + "••••••••" + token.substring(token.length - 4);
};
