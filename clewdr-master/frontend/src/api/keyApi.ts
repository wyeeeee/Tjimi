// frontend/src/api/keyApi.ts
import { KeyStatusInfo } from "../types/key.types";

/**
 * Sends a key to the server.
 * @param key The key string to send
 * @returns The fetch response object
 *
 * Possible Status Codes:
 * - 200: Success
 * - 400: Invalid key format
 * - 401: Invalid bearer token
 * - 500: Server error
 */
export async function postKey(key: string) {
  const token = localStorage.getItem("authToken") || "";
  const response = await fetch("/api/key", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${token}`,
    },
    body: JSON.stringify({ key }),
  });

  if (response.status === 401) {
    throw new Error("Authentication failed. Please set a valid auth token.");
  } else if (response.status === 500) {
    throw new Error("Server error.");
  } else if (response.status === 400) {
    throw new Error("Invalid key format");
  }

  if (!response.ok) {
    throw new Error(`Error ${response.status}: ${response.statusText}`);
  }

  return response;
}

/**
 * Gets key status information from the server.
 * @returns The key status data
 *
 * Possible Status Codes:
 * - 200: Success with key status data
 * - 401: Invalid bearer token
 * - 500: Server error
 */
export async function getKeyStatus(): Promise<KeyStatusInfo> {
  const token = localStorage.getItem("authToken") || "";
  const response = await fetch("/api/keys", {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${token}`,
    },
  });

  if (!response.ok) {
    throw new Error(`Error ${response.status}: ${response.statusText}`);
  }

  return await response.json();
}

/**
 * Deletes a key from the server.
 * @param key The key string to delete
 * @returns The fetch response object
 *
 * Possible Status Codes:
 * - 204: Success (No Content)
 * - 400: Invalid key format
 * - 401: Invalid bearer token
 * - 500: Server error
 */
export async function deleteKey(key: string) {
  const token = localStorage.getItem("authToken") || "";
  const response = await fetch(`/api/key`, {
    method: "DELETE",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${token}`,
    },
    body: JSON.stringify({ key }),
  });

  return response;
}
