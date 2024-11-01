import { Timestamp } from "@/api/google/protobuf/timestamp";
import { useCoreEntities } from "@/hooks/core";
import {
  Paper,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
} from "@mui/material";

export function CoreV1Page() {
  const { value: entities } = useCoreEntities();

  return (
    <TableContainer component={Paper}>
      <Table sx={{ minWidth: 650 }} aria-label="shadow">
        <TableHead>
          <TableRow>
            <TableCell>Package</TableCell>
            <TableCell>Kind</TableCell>
            <TableCell>Key</TableCell>
            <TableCell>Created At</TableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {entities.map((entity) => (
            <TableRow
              key={entity.spec?.uniqueId}
              sx={{ "&:last-child td, &:last-child th": { border: 0 } }}
            >
              <TableCell>{entity.spec?.package}</TableCell>
              <TableCell>{entity.spec?.kind}</TableCell>
              <TableCell>{entity.spec?.key}</TableCell>
              <TableCell align="right">
                {Timestamp.toJsonString(entity.status!.createdAt!)}
              </TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </TableContainer>
  );
}
