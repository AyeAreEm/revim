// vite.config.js
import { defineConfig } from "file:///E:/Projects/js%20projects/revim/node_modules/vite/dist/node/index.js";
import { svelte } from "file:///E:/Projects/js%20projects/revim/node_modules/@sveltejs/vite-plugin-svelte/src/index.js";
var vite_config_default = defineConfig(async () => ({
  plugins: [svelte()],
  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  // prevent vite from obscuring rust errors
  clearScreen: false,
  // tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true
  },
  // to make use of `TAURI_DEBUG` and other env variables
  // https://tauri.studio/v1/api/config#buildconfig.beforedevcommand
  envPrefix: ["VITE_", "TAURI_"],
  build: {
    // Tauri supports es2021
    target: process.env.TAURI_PLATFORM == "windows" ? "chrome105" : "safari13",
    // don't minify for debug builds
    minify: !process.env.TAURI_DEBUG ? "esbuild" : false,
    // produce sourcemaps for debug builds
    sourcemap: !!process.env.TAURI_DEBUG
  }
}));
export {
  vite_config_default as default
};
//# sourceMappingURL=data:application/json;base64,ewogICJ2ZXJzaW9uIjogMywKICAic291cmNlcyI6IFsidml0ZS5jb25maWcuanMiXSwKICAic291cmNlc0NvbnRlbnQiOiBbImNvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9kaXJuYW1lID0gXCJFOlxcXFxQcm9qZWN0c1xcXFxqcyBwcm9qZWN0c1xcXFxyZXZpbVwiO2NvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9maWxlbmFtZSA9IFwiRTpcXFxcUHJvamVjdHNcXFxcanMgcHJvamVjdHNcXFxccmV2aW1cXFxcdml0ZS5jb25maWcuanNcIjtjb25zdCBfX3ZpdGVfaW5qZWN0ZWRfb3JpZ2luYWxfaW1wb3J0X21ldGFfdXJsID0gXCJmaWxlOi8vL0U6L1Byb2plY3RzL2pzJTIwcHJvamVjdHMvcmV2aW0vdml0ZS5jb25maWcuanNcIjtpbXBvcnQgeyBkZWZpbmVDb25maWcgfSBmcm9tIFwidml0ZVwiO1xyXG5pbXBvcnQgeyBzdmVsdGUgfSBmcm9tIFwiQHN2ZWx0ZWpzL3ZpdGUtcGx1Z2luLXN2ZWx0ZVwiO1xyXG5cclxuLy8gaHR0cHM6Ly92aXRlanMuZGV2L2NvbmZpZy9cclxuZXhwb3J0IGRlZmF1bHQgZGVmaW5lQ29uZmlnKGFzeW5jICgpID0+ICh7XHJcbiAgcGx1Z2luczogW3N2ZWx0ZSgpXSxcclxuXHJcbiAgLy8gVml0ZSBvcHRpb25zIHRhaWxvcmVkIGZvciBUYXVyaSBkZXZlbG9wbWVudCBhbmQgb25seSBhcHBsaWVkIGluIGB0YXVyaSBkZXZgIG9yIGB0YXVyaSBidWlsZGBcclxuICAvLyBwcmV2ZW50IHZpdGUgZnJvbSBvYnNjdXJpbmcgcnVzdCBlcnJvcnNcclxuICBjbGVhclNjcmVlbjogZmFsc2UsXHJcbiAgLy8gdGF1cmkgZXhwZWN0cyBhIGZpeGVkIHBvcnQsIGZhaWwgaWYgdGhhdCBwb3J0IGlzIG5vdCBhdmFpbGFibGVcclxuICBzZXJ2ZXI6IHtcclxuICAgIHBvcnQ6IDE0MjAsXHJcbiAgICBzdHJpY3RQb3J0OiB0cnVlLFxyXG4gIH0sXHJcbiAgLy8gdG8gbWFrZSB1c2Ugb2YgYFRBVVJJX0RFQlVHYCBhbmQgb3RoZXIgZW52IHZhcmlhYmxlc1xyXG4gIC8vIGh0dHBzOi8vdGF1cmkuc3R1ZGlvL3YxL2FwaS9jb25maWcjYnVpbGRjb25maWcuYmVmb3JlZGV2Y29tbWFuZFxyXG4gIGVudlByZWZpeDogW1wiVklURV9cIiwgXCJUQVVSSV9cIl0sXHJcbiAgYnVpbGQ6IHtcclxuICAgIC8vIFRhdXJpIHN1cHBvcnRzIGVzMjAyMVxyXG4gICAgdGFyZ2V0OiBwcm9jZXNzLmVudi5UQVVSSV9QTEFURk9STSA9PSBcIndpbmRvd3NcIiA/IFwiY2hyb21lMTA1XCIgOiBcInNhZmFyaTEzXCIsXHJcbiAgICAvLyBkb24ndCBtaW5pZnkgZm9yIGRlYnVnIGJ1aWxkc1xyXG4gICAgbWluaWZ5OiAhcHJvY2Vzcy5lbnYuVEFVUklfREVCVUcgPyBcImVzYnVpbGRcIiA6IGZhbHNlLFxyXG4gICAgLy8gcHJvZHVjZSBzb3VyY2VtYXBzIGZvciBkZWJ1ZyBidWlsZHNcclxuICAgIHNvdXJjZW1hcDogISFwcm9jZXNzLmVudi5UQVVSSV9ERUJVRyxcclxuICB9LFxyXG59KSk7XHJcbiJdLAogICJtYXBwaW5ncyI6ICI7QUFBbVIsU0FBUyxvQkFBb0I7QUFDaFQsU0FBUyxjQUFjO0FBR3ZCLElBQU8sc0JBQVEsYUFBYSxhQUFhO0FBQUEsRUFDdkMsU0FBUyxDQUFDLE9BQU8sQ0FBQztBQUFBO0FBQUE7QUFBQSxFQUlsQixhQUFhO0FBQUE7QUFBQSxFQUViLFFBQVE7QUFBQSxJQUNOLE1BQU07QUFBQSxJQUNOLFlBQVk7QUFBQSxFQUNkO0FBQUE7QUFBQTtBQUFBLEVBR0EsV0FBVyxDQUFDLFNBQVMsUUFBUTtBQUFBLEVBQzdCLE9BQU87QUFBQTtBQUFBLElBRUwsUUFBUSxRQUFRLElBQUksa0JBQWtCLFlBQVksY0FBYztBQUFBO0FBQUEsSUFFaEUsUUFBUSxDQUFDLFFBQVEsSUFBSSxjQUFjLFlBQVk7QUFBQTtBQUFBLElBRS9DLFdBQVcsQ0FBQyxDQUFDLFFBQVEsSUFBSTtBQUFBLEVBQzNCO0FBQ0YsRUFBRTsiLAogICJuYW1lcyI6IFtdCn0K
