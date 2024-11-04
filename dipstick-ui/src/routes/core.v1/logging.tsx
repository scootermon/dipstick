import { Timestamp } from "@/api/google/protobuf/timestamp";
import { useLogs } from "@/hooks/core";
import {
  Paper,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
} from "@mui/material";

export function CoreV1LoggingPage() {
  const logs = useLogs();

  return (
    <TableContainer component={Paper}>
      <Table>
        <TableHead>
          <TableRow>
            <TableCell>Timestamp</TableCell>
            <TableCell>Target</TableCell>
            <TableCell>Message</TableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {logs.reverse().map((log) => (
            <TableRow key={Timestamp.toJsonString(log.timestamp!)}>
              <TableCell>{Timestamp.toJsonString(log.timestamp!)}</TableCell>
              <TableCell>{log.target}</TableCell>
              <TableCell>{log.message}</TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </TableContainer>
  );
}
