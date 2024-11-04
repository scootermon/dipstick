import ErrorPage from "./routes/ErrorPage.tsx";
import Root from "./routes/Root.tsx";
import Shadow from "./routes/Shadow.tsx";
import { CoreV1Page } from "./routes/core.v1/index.tsx";
import { CoreV1LoggingPage } from "./routes/core.v1/logging.tsx";
import "@fontsource/roboto/300.css";
import "@fontsource/roboto/400.css";
import "@fontsource/roboto/500.css";
import "@fontsource/roboto/700.css";
import React from "react";
import ReactDOM from "react-dom/client";
import { RouterProvider, createBrowserRouter } from "react-router-dom";

const router = createBrowserRouter([
  {
    path: "/",
    element: <Root />,
    errorElement: <ErrorPage />,
    children: [
      {
        path: "core.v1/",
        children: [
          {
            path: "",
            element: <CoreV1Page />,
          },
          {
            path: "logging",
            element: <CoreV1LoggingPage />,
          },
        ],
      },
      {
        path: "shadow.v1/Shadow/:uniqueId",
        element: <Shadow />,
      },
    ],
  },
]);

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <RouterProvider router={router} />
  </React.StrictMode>,
);
