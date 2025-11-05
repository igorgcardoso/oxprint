import type { HealthCheckResponse } from "@/@types/api";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { api } from "@/lib/api";
import { cn } from "@/lib/utils";
import { useQuery } from "@tanstack/react-query";
import { createFileRoute } from "@tanstack/react-router";
import { FileText, Printer, Thermometer } from "lucide-react";

export const Route = createFileRoute("/")({
  component: Dashboard,
});

function Dashboard() {
  const {
    data: healthData,
    isLoading,
    isError,
    refetch,
  } = useQuery<HealthCheckResponse>({
    queryKey: ["health"],
    queryFn: async () => {
      const response = await api.health.check();

      return response.data;
    },
    refetchInterval: 30 * 1000, // 30 seconds
  });

  const backendStatus = isLoading
    ? "Checking..."
    : isError
      ? "Disconnected"
      : healthData?.status === "healthy"
        ? "Connected"
        : "Error";

  return (
    <div className="container mx-auto px-4 py-8">
      <header className="mb-8">
        <h1 className="mb-2 text-4xl font-bold text-gray-100">
          OxPrint Dashboard
        </h1>
        <p className="text-gray-400">Modern 3d Printer Management System</p>
      </header>

      <div className="mb-8 grid grid-cols-1 gap-6 md:grid-cols-3">
        {/* Status Card */}
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">
              Printer Status
            </CardTitle>
            <Printer className="text-muted-foreground size-4" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">Operational</div>
            <p className="text-muted-foreground text-xs">Ready to print</p>
          </CardContent>
        </Card>

        {/* Temperature Card */}
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">
              Printer Temperature
            </CardTitle>
            <Thermometer className="text-muted-foreground size-4" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">210ºC</div>
            <p className="text-muted-foreground text-xs">Hotend / 24ºC Bed</p>
          </CardContent>
        </Card>

        {/* Files Card */}
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">G-Code Files</CardTitle>
            <FileText className="text-muted-foreground size-4" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">12</div>
            <p className="text-muted-foreground text-xs">Ready to print</p>
          </CardContent>
        </Card>
      </div>

      {/* Backend Connection Status */}
      <Card>
        <CardHeader>
          <CardTitle>System Status</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="itens-center flex justify-between">
            <div className="flex items-center">
              <div
                className={cn("mr-3 size-3 rounded-full", {
                  "bg-green-500": backendStatus === "Connected",
                  "bg-red-500": backendStatus !== "Connected",
                })}
              />
              <span className="font-medium">Backend:</span>{" "}
              <span
                className={cn({
                  "text-green-500": backendStatus === "Connected",
                  "text-red-500": backendStatus !== "Connected",
                })}
              >
                {backendStatus}
              </span>
            </div>
            <Button
              variant="outline"
              size="sm"
              onClick={() => refetch()}
              disabled={isLoading}
            >
              {isLoading ? "Checking..." : "Refresh"}
            </Button>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
