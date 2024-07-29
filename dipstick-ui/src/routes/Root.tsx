import RootLayout from "../components/RootLayout";
import { CssBaseline } from "@mui/material";
import { Outlet } from "react-router-dom";

export default function Root() {
  return (
    <>
      <CssBaseline />
      <RootLayout title="Dipstick" navigationItems={[]}>
        <Outlet />
      </RootLayout>
    </>
  );
}
