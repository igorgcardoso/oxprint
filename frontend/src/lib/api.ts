import type { HealthCheckResponse } from "@/@types/api";
import axios, { type AxiosResponse } from "axios";

export const apiClient = axios.create({
  baseURL: import.meta.env.VITE_API_URL || "http://localhost:8080",
  headers: {
    "Content-Type": "application/json",
  },
});

apiClient.interceptors.request.use((config) => {
  const token = localStorage.getItem("oxprint_token");

  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
  }

  return config;
});

export const api = {
  health: {
    check: (): Promise<AxiosResponse<HealthCheckResponse>> =>
      apiClient.get("/api/health"),
  },
};
