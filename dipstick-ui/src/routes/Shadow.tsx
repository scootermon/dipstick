import { Value } from "@/api/google/protobuf/struct";
import { useShadowSignals } from "@/hooks/shadow";
import {
  Paper,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
} from "@mui/material";
import { useParams } from "react-router-dom";

export default function Shadow() {
  const { uniqueId } = useParams();
  const { signals } = useShadowSignals(parseInt(uniqueId!));

  return (
    <TableContainer component={Paper}>
      <Table sx={{ minWidth: 650 }} aria-label="shadow">
        <TableHead>
          <TableRow>
            <TableCell>Signal</TableCell>
            <TableCell align="right">Value</TableCell>
            <TableCell align="right">Update Count</TableCell>
            <TableCell align="right">Changed At</TableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {Object.entries(signals).map(([signal, state]) => (
            <TableRow
              key={signal}
              sx={{ "&:last-child td, &:last-child th": { border: 0 } }}
            >
              <TableCell component="th" scope="row">
                {signal}
              </TableCell>
              <TableCell align="right">
                {Value.toJsonString(state.value)}
              </TableCell>
              <TableCell align="right">{state.updateCount}</TableCell>
              <TableCell align="right">
                {state.changedAt?.toLocaleString()}
              </TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </TableContainer>
  );
}
