import react from "@vitejs/plugin-react-swc";
import { defineConfig } from "vite";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  server: {
    // 2024-11-01: somehow Firefox doesn't like localhost
    host: "127.0.0.1",
  },
});
