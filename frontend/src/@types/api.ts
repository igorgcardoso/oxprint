export type HealthCheckResponse = {
  status: "healthy" | "unhealthy";
  timestamp?: string;
};

export type PrinterStatus = {
  id: string;
  name: string;
  status: "idle" | "printing" | "error" | "offline";
  temperature?: {
    hotend: number;
    bed: number;
  };
};
