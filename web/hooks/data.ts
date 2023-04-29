import { useState, useMemo, useEffect } from "react";
import { getCoreRowModel, getPaginationRowModel, getSortedRowModel, PaginationState, SortingState, Table, useReactTable } from "@tanstack/react-table";
import { useQueries, useQuery, UseQueryResult } from "@tanstack/react-query";
import { App, RangeInput, RangeKind } from "../pkg/web";

type Deserializer<T> = (buffer: ArrayBuffer, id: bigint | string) => T;

type PathRetriever = (app: App, input: RangeInput, totalCount: number) => Promise<[bigint | string, string][]>;

interface Updatable {
  isLoaded: boolean,
  isPreviousData: boolean
}

type Data<T> = T & Updatable;

export function useDataTable<T>(app: App, plural: string, kind: RangeKind, deserializer: Deserializer<T>, pathRetriever: PathRetriever, columns, totalCount: number, defaultSort = "default"): Table<Data<T>> {

  const [sorting, setSorting] = useState<SortingState>([]);

  const [{ pageIndex, pageSize }, setPagination] =
    useState<PaginationState>({
      pageIndex: 0,
      pageSize: 10,
    });

  const pagination = useMemo(
    () => ({
      pageIndex,
      pageSize,
    }),
    [pageIndex, pageSize]
  );

  const pageCount = useMemo(
    () => Math.ceil(totalCount / pageSize),
    [pageSize]
  );

  const { sortId, sortDesc } = useMemo(
    () => ({
      sortId: sorting.length == 0 ? defaultSort : sorting[0].id,
      sortDesc: sorting.length == 0 ? false : sorting[0].desc
    }),
    [sorting]);

  const paths = useQuery({
    queryKey: ["paths", plural, kind, pageIndex, pageSize, sortId, sortDesc],
    queryFn: () => pathRetriever(app, {
      settings: {
        pageIndex,
        pageSize,
        sortId,
        sortDesc
      },
      plural,
      kind
    },
      totalCount),
    keepPreviousData: true
  });

  const buffers = useQueries({
    queries: paths.data ? paths.data.map((path) => {
      return {
        queryKey: [path[1]],
        queryFn: () => useBuffer(path[0], path[1]),
        keepPreviousData: true
      }
    }) : []
  });

  const data = useQueries({
    queries: buffers.map((b) => {
      return {
        queryKey: ["buf", plural, b.data?.id.toString()],
        queryFn: () => deserializer(b.data.buffer, b.data.id),
        enabled: !!b.data,
        keepPreviousData: true
      }
    })
  });

  const defaultData = useMemo(() => [], [])

  return useReactTable(
    {
      columns: useMemo(() => columns, []),
      data: data.map(q => buildData(q)),
      pageCount,
      state: {
        sorting, pagination
      },
      onSortingChange: setSorting,
      onPaginationChange: setPagination,
      getCoreRowModel: getCoreRowModel(),
      getSortedRowModel: getSortedRowModel(),
      getPaginationRowModel: getPaginationRowModel(),
      manualPagination: kind.kind != "epoch",
      manualSorting: kind.kind != "epoch"
    }
  );
}

export function useBuffer(id: bigint | string, path: string): Promise<ModelBuffer> {
  return fetch(path)
    .then(r => r.blob())
    .then(b => b.arrayBuffer())
    .then(b => { return { id: id, buffer: b } });
}

export interface ModelBuffer {
  id: bigint | string,
  buffer: ArrayBuffer
}

function buildData<T>(query: UseQueryResult<T, unknown>): Data<T> {
  let updatable = { isLoaded: !!query.data, isPreviousData: query.isPreviousData };
  return { ...query.data, ...updatable }
}